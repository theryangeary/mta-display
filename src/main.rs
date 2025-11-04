use axum::routing::{post, put, delete};
use gif::{Encoder, Frame, Repeat};
use image::{ImageBuffer, Rgb};
use rust_embed::Embed;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::str::FromStr;

use std::io::Write;
use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, HeaderValue, StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::{Form, Json};
use axum::{Router, routing::get};
use maud::{DOCTYPE, Markup, html};
use serde::Deserialize;
use tokio::signal;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod db;
mod models;
mod pattern;
mod types;
use types::BulbDisplay;
use types::BulbDisplayConfig;

use crate::db::{Database, SqliteDatabase};
use crate::models::GalleryEntry;
use crate::types::{BulbDisplaySize, Train};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:gallery.db".to_string());
    let db = Arc::new(SqliteDatabase::new(&database_url).await?);

    let app = Router::new()
        .route("/", get(get_index_markup))
        .route("/health", get(health_check))
        .route("/static/{file}", get(get_static_file))
        .route("/generate", post(post_generate))
        .route("/gif/{size}/{train}/{message}", get(get_gif_file))
        .route("/gallery", get(get_gallery))
        .route("/gallery/entry", get(get_gallery_entry))
        .route("/gallery/entry", post(post_gallery_entry))
        .route("/gallery/review", get(get_gallery_review))
        .route("/gallery/review/{id}/approve", put(put_gallery_review_approve))
        .route("/gallery/review/{id}/reject", delete(put_gallery_review_reject))
        .layer(TraceLayer::new_for_http())
        .with_state(db);

    // Run it on localhost:3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    let server =
        axum::serve(listener, app.into_make_service()).with_graceful_shutdown(shutdown_signal());

    if let Err(e) = server.await {
        tracing::error!("server error: {}", e);
    }

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("received Ctrl+C signal");
        }
        _ = terminate => {
            tracing::info!("received SIGTERM signal");
        }
    }

    tracing::info!("starting graceful shutdown");
}

#[derive(Embed)]
#[folder = "$OUT_DIR/static"]
struct Assets;

async fn get_static_file(Path(path): Path<String>) -> impl IntoResponse {
    tracing::info!("static");
    match Assets::get(&path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();

            ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
        }
        None => {
            println!(
                "{} not found in {:?}",
                path,
                Assets::iter().collect::<Vec<_>>()
            );
            not_found().await
        }
    }
}

async fn not_found() -> Response {
    (StatusCode::NOT_FOUND, "404").into_response()
}

async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "service": "mta-display"
    }))
}

fn head(title: &str) -> Markup {
    html! {
        head {
            (DOCTYPE)
            meta charset="UTF-8" {};
            meta name="viewport" content="width=device-width, initial-scale=1.0" {};
            link rel="stylesheet" href="/static/output.css";
            script src="/static/htmx.min.js" {};
            title { (title) }
        }
    }
}

async fn get_gallery_entry() -> Markup {
    get_gallery_entry_markup()
}

fn get_gallery_entry_markup() -> Markup {
    html! {
        (head("Create Gallery Entry"))
        body {
            div class=" flex justify-center " {
                div
                    class="
                    grid content-center
                    max-w-4xl
                    mx-4
                    "
                {
                    h1 { a href="/gallery" { "Submit to Gallery" } }

                    form
                        hx-post="/gallery/entry"
                        hx-target="body"
                        class="bg-gray-200 border-gray-300 p-4 rounded-xl mb-4"
                    {
                        label class="flex w-100% block mb-2" for="message" {
                            span class="flex-grow" { "Message: " }
                            span class="flex-shrink relative group" {
                                button type="button"
                                    class="cursor-pointer focus:outline-none "
                                    onclick="this.nextElementSibling.classList.toggle('hidden')"
                                {
                                    "ⓘ"
                                }

                                span class=" absolute hidden group-hover:block bg-gray-800 text-white text-xs rounded py-1 px-2 -left-50 top-full mb-1 z-10 whitespace-nowrap " {
                                    p class="mb-1" {"Max 6 rows, 14 characters per row. "}
                                    p class="mb-1" {"Use linebreaks to separate pages manually. "}
                                    p class="mb-1" {"Unsupported characters will be ignored." }
                                }
                            }
                        }
                        textarea
                            class=" w-full p-2 mt-2 mb-4 bg-white border border-gray-600 rounded-lg text-black focus:outline-none focus:border-blue-500"
                            name="message"
                            id="message"
                            rows="4"
                            placeholder="Type your message here..." {}
                        br;

                        label class=" flex w-100% block mb-2" for="train" {
                            span class="flex-grow" { "Train: " }
                            // todo layout all bullets in a grid for selection
                            select
                                name="train"
                                id="train"
                                class=" ml-2 p-2 bg-white border border-gray-600 rounded-lg text-black focus:outline-none focus:border-blue-500 "
                            {
                                option value="One" { "1" }
                                option value="Two" { "2" }
                                option value="Three" { "3" }
                                option value="Four" { "4" }
                                option value="Five" { "5" }
                                option value="Six" { "6" }
                            }
                        }

                        label class=" flex w-100% block mb-2" for="submitter_name" {
                            span class="flex-grow" { "Your Name (optional): " }
                            input
                                type="text"
                                name="submitter_name"
                                id="submitter_name"
                                class=" ml-2 p-2 bg-white border border-gray-600 rounded-lg text-black focus:outline-none focus:border-blue-500 "
                            ;
                        }

                        label class=" flex w-100% block mb-2" for="description" {
                            span class="flex-grow" { "Caption (optional): " }
                            textarea
                                class=" w-full p-2 mt-2 mb-4 bg-white border border-gray-600 rounded-lg text-black focus:outline-none focus:border-blue-500"
                                name="description"
                                id="description"
                                rows="4"
                                placeholder="Caption your submission..." {}
                        }

                        button type="submit" class=" bg-yellow-500 text-black font-bold py-2 px-4 rounded hover:bg-yellow-600 " { "Submit to Gallery" }
                    }
                }
            }
        }
    }
}

#[derive(Deserialize, Debug)]
struct GalleryEntryForm {
    message: String,
    train: Train,
    submitter_name: Option<String>,
    description: Option<String>,
}

async fn post_gallery_entry(
    State(db): State<Arc<dyn Database>>,
    Form(gallery_entry_form): Form<GalleryEntryForm>,
) -> Result<Response, StatusCode> {
    let message = gallery_entry_form.message;
    let train_str = gallery_entry_form.train;
    let submitter_name = gallery_entry_form.submitter_name;
    let description = gallery_entry_form.description;

    db.create_gallery_entry(
        &message,
        train_str,
        submitter_name.as_deref(),
        description.as_deref(),
    )
    .await
    .map_err(|e| {
        tracing::error!("failed to create gallery entry: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("text/html"));
    headers.insert("hx-push-url", "/gallery".parse().unwrap());

    let markup = get_gallery_with_banner_markup(State(db)).await?;

    Ok((headers, markup).into_response())
}

async fn get_gallery_review(
    State(db): State<Arc<dyn Database>>,
) -> Result<Markup, StatusCode> {
    let entries = db.list_pending_gallery_entries().await.map_err(|e| {
        tracing::error!("failed to get gallery entries: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(get_gallery_review_markup(entries))
}

fn get_gallery_review_markup(entries: Vec<GalleryEntry>) -> Markup {
    html! {
        (head("Gallery Review"))
        body {
            div class=" flex justify-center " {
                div
                    class="
                    grid content-center
                    max-w-4xl
                    mx-4
                    "
                {
                    h1 { a href="/gallery" { "Gallery Review" } }

                    @for entry in entries {
                        @let i = format!("gallery-entry-{}", entry.id);
                        div id=(i) class=" mb-8 " {
                            div class=" flex justify-center mb-1 " {
                                img
                                    src=(&format!("/gif/sm/{}/{}", entry.train, urlencoding::encode(&entry.message)))
                                    alt=(&format!("Generated MTA Display with message {}", entry.message))
                                    class="h-auto max-w-full"
                                ;
                            }

                            @if let Some(desc) = &entry.description {
                                p class=" prose prose-sm mx-auto mb-2 " {
                                    (desc)
                                }
                            }

                            p class=" text-center text-sm text-gray-600 mb-2 " {
                                "Submitted by "
                                span class=" font-bold " {
                                    @if entry.submitter_name.is_some() && !entry.submitter_name.as_ref().unwrap().is_empty() {
                                        (entry.submitter_name.as_ref().unwrap())
                                    } @else {
                                        "anonymous"
                                    }
                                }
                                " on "
                                (entry.submitted_at.format("%Y-%m-%d"))
                            }
                            div class=" flex justify-center space-x-4 " {
                                button
                                    hx-put=(format!("/gallery/review/{}/approve", entry.id))
                                    hx-target=(format!("#{}", i))
                                    class=" bg-green-500 text-white font-bold py-2 px-4 rounded hover:bg-green-600 "
                                { "Approve" }

                                button
                                    hx-delete=(format!("/gallery/review/{}/reject", entry.id))
                                    hx-target=(format!("#{}", i))
                                    class=" bg-red-500 text-white font-bold py-2 px-4 rounded hover:bg-red-600 "
                                { "Reject" }
                            }
                        }
                    }
                }
            }
        }
    }
}

async fn put_gallery_review_approve(
    State(db): State<Arc<dyn Database>>,
    Path(id): Path<i64>,
) -> Result<Markup, StatusCode> {
    db.approve_gallery_entry(id).await.map_err(|e| {
        tracing::error!("failed to approve gallery entry {}: {}", id, e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(html!{div{}})
}

async fn put_gallery_review_reject(
    State(db): State<Arc<dyn Database>>,
    Path(id): Path<i64>,
) -> Result<Markup, StatusCode> {
    db.reject_gallery_entry(id).await.map_err(|e| {
        tracing::error!("failed to reject gallery entry {}: {}", id, e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(html!{div{}})
}

async fn get_gallery(State(db): State<Arc<dyn Database>>) -> Result<Markup, StatusCode> {
    let entries = db.list_approved_gallery_entries().await.map_err(|e| {
        tracing::error!("failed to get gallery entries: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(get_gallery_markup(entries, None))
}

async fn get_gallery_with_banner_markup(
    State(db): State<Arc<dyn Database>>,
) -> Result<Response, StatusCode> {
    let entries = db.list_approved_gallery_entries().await.map_err(|e| {
        tracing::error!("failed to get gallery entries: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("text/html"));
    headers.insert("hx-push-url", "/gallery".parse().unwrap());

    Ok((
        headers,
        get_gallery_markup(
            entries,
            Some("Your submission has been received and is pending approval.".into()),
        ),
    )
        .into_response())
}

fn get_gallery_markup(entries: Vec<GalleryEntry>, banner: Option<String>) -> Markup {
    html! {
        (head("Gallery"))
        body {
            div class=" flex justify-center " {
                div
                    class="
                    grid content-center
                    max-w-4xl
                    mx-4
                    "
                {
                    h1 class="mb-4" { a href="/" { "Gallery" } }

                    @if let Some(banner_msg) = banner {
                        div class=" bg-green-200 border border-green-400 text-green-800 px-4 py-3 rounded relative mb-4 " role="alert" {
                            strong class=" font-bold " { "Success! " }
                            span class=" block sm:inline " { (banner_msg) }
                        }
                    }

                    // link to submit new entry page
                    p class=" mb-8 w-full text-right " {
                        a href="/gallery/entry" class=" text-blue-500 hover:underline text-right" { "✒ Create Submission" }
                    }

                    @for entry in entries {
                        div class=" mb-8 " {
                            div class=" flex justify-center mb-1 " {
                                img
                                    src=(&format!("/gif/sm/{}/{}", entry.train, urlencoding::encode(&entry.message)))
                                    alt=(&format!("Generated MTA Display with message {}", entry.message))
                                    class="h-auto max-w-full"
                                ;
                            }

                            @if let Some(desc) = &entry.description {
                                p class=" prose prose-sm mx-auto mb-2 " {
                                    (desc)
                                }
                            }

                            p class=" text-center text-sm text-gray-600 mb-2 " {
                                "Submitted by "
                                span class=" font-bold " {
                                    @if entry.submitter_name.is_some() && !entry.submitter_name.as_ref().unwrap().is_empty() {
                                        (entry.submitter_name.as_ref().unwrap())
                                    } @else {
                                        "anonymous"
                                    }
                                }
                                " on "
                                (entry.submitted_at.format("%Y-%m-%d"))
                            }


                        }
                    }
                }
            }
        }
    }
}

#[derive(Deserialize, Debug)]
struct GenerateGifForm {
    message: String,
    train: Train,
}

/// Handle form submission to generate a new GIF, returning the updated markup to replace the existing image.
async fn post_generate(
    Form(generate_gif_form): Form<GenerateGifForm>,
) -> Result<Response, StatusCode> {
    let encoded_message = urlencoding::encode(&generate_gif_form.message);
    let header_input = format!(
        "/?message={}&train={}",
        encoded_message, &generate_gif_form.train
    );
    let url = HeaderValue::from_str(&header_input).map_err(|e| {
        tracing::error!(
            "failed to turn message param into push-url: {}. HeaderValue input: {}",
            e,
            header_input
        );
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("text/html"));
    headers.insert("hx-push-url", url);

    Ok((
        headers,
        gif_markup(generate_gif_form.train, &generate_gif_form.message),
    )
        .into_response())
}

fn gif_markup(train: Train, message: &str) -> Markup {
    let url_encoded_message = urlencoding::encode(message);
    html! {
        div id="mta-sign-gif" {
            div
                class=" flex justify-center mb-4 "
            {
                img
                    src=(&format!("/gif/md/{}/{}", train, url_encoded_message))
                    alt=(&format!("Generated MTA Display with message {}", message))
                    class="h-auto max-w-full"
                ;
            }

            div class=" mb-8 flex justify-center " {
                div class=" divide-x-1 divide-yellow-700 rounded-xl " {
                    button class="bg-yellow-500 text-black font-light text-sm py-2 px-4 hover:bg-yellow-600 rounded-l-xl" { a target="_blank" href=(format!("/gif/sm/{}/{}", train, url_encoded_message)) { "Small" } }
                    button class="bg-yellow-500 text-black font-light text-sm py-2 px-4 hover:bg-yellow-600 " { a target="_blank" href=(format!("/gif/md/{}/{}", train, url_encoded_message)) { "Medium" } }
                    button class="bg-yellow-500 text-black font-light text-sm py-2 px-4 hover:bg-yellow-600 " { a target="_blank" href=(format!("/gif/lg/{}/{}", train, url_encoded_message)) { "Large" } }
                    button class="bg-yellow-500 text-black font-light text-sm py-2 px-4 hover:bg-yellow-600 rounded-r-xl" { a target="_blank" href=(format!("/gif/xl/{}/{}", train, url_encoded_message)) { "Extra Large" } }
                }
            }
        }
    }
}

async fn get_gif_file(Path((size, train, message)): Path<(String, Train, String)>) -> Response {
    let bulb_display_size = match BulbDisplaySize::from_str(&size) {
        Ok(s) => s,
        Err(e) => {
            tracing::error!("failed to parse size parameter from '{}': {}", size, e);
            tracing::debug!("defaulting to XSmall size");
            BulbDisplaySize::XSmall
        }
    };
    let config = BulbDisplayConfig::new_from_size(bulb_display_size);

    let uppercase_message = message.to_ascii_uppercase();
    let message_parts = split_message_into_parts(&config, &uppercase_message);
    // prevent screens that would show only blank rows
    let visible_message_parts: Vec<&str> = message_parts
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect();

    let frames = generate_frames_for_message(&config, train, visible_message_parts).unwrap();

    let gif_data = write_frames_to_gif_in_memory(&config, &frames).unwrap();

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("image/gif"));

    (headers, gif_data).into_response()
}

async fn get_index_markup(Query(params): Query<HashMap<String, String>>) -> Markup {
    let message = params
        .get("message")
        .cloned()
        .unwrap_or_else(|| "Welcome to the MTA display generator".into());

    let train = Train::from_str(&params.get("train").cloned().unwrap_or_else(|| "A".into()))
        .unwrap_or(Train::A);

    html! {
        (head("MTA Display Generator"))
        body {
            div class="flex justify-center" {
                div
                    class="
                    grid content-center
                    max-w-lg
                    mx-4
                    "
                {
                    h1 { a href="/" { "MTA Display Generator" } }

                    (gif_markup(train, &message))

                    h2 { "Make your own!" }

                    form
                        hx-post="/generate"
                        hx-target="#mta-sign-gif"
                        hx-swap="outerHTML"
                        class="bg-gray-200 border-gray-300 p-4 rounded-xl mb-4"
                    {
                        label class="flex w-100% block mb-2" for="message" {
                            span class="flex-grow" { "Message: " }
                            span class="flex-shrink relative group" {
                                button type="button"
                                    class="cursor-pointer focus:outline-none "
                                    onclick="this.nextElementSibling.classList.toggle('hidden')"
                                {
                                    "ⓘ"
                                }

                                span class=" absolute hidden group-hover:block bg-gray-800 text-white text-xs rounded py-1 px-2 -left-50 top-full mb-1 z-10 whitespace-nowrap " {
                                    p class="mb-1" {"Max 6 rows, 14 characters per row. "}
                                    p class="mb-1" {"Use linebreaks to separate pages manually. "}
                                    p class="mb-1" {"Unsupported characters will be ignored." }
                                }
                            }
                        }
                        textarea
                            class=" w-full p-2 mt-2 mb-4 bg-white border border-gray-600 rounded-lg text-black focus:outline-none focus:border-blue-500"
                            name="message"
                            id="message"
                            rows="4"
                            placeholder="Type your message here..." {
                                (message)
                            }
                        br;

                        label class=" flex w-100% block mb-2" for="train" {
                            span class="flex-grow" { "Train: " }
                            // todo layout all bullets in a grid for selection
                            select
                                name="train"
                                id="train"
                                class=" ml-2 p-2 bg-white border border-gray-600 rounded-lg text-black focus:outline-none focus:border-blue-500 "
                            {
                                option value="One" { "1" }
                                option value="Two" { "2" }
                                option value="Three" { "3" }
                                option value="Four" { "4" }
                                option value="Five" { "5" }
                                option value="Six" { "6" }
                                option value="Seven" { "7" }
                                option value="A" { "A" }
                                option value="B" { "B" }
                                option value="C" { "C" }
                                option value="D" { "D" }
                                option value="E" { "E" }
                                option value="F" { "F" }
                                option value="G" { "G" }
                                option value="J" { "J" }
                                option value="L" { "L" }
                                option value="M" { "M" }
                                option value="N" { "N" }
                                option value="Q" { "Q" }
                                option value="R" { "R" }
                                option value="S" { "S" }
                                option value="W" { "W" }
                                option value="Z" { "Z" }
                            }
                        }

                        button type="submit" class=" bg-yellow-500 text-black font-bold py-2 px-4 rounded hover:bg-yellow-600 " { "Generate" }
                    }

                    h2 { "Gallery" }
                    p class="prose" { "Check out the " a class="underline underline-offset-2 hover:decoration-2" href="/gallery" { "gallery" } " to see submissions from other users, or submit your own!" }

                    h2 { "About"}
                    p class="prose" { "This is a fun side project that generates GIFs that simulate a display as you would see on the New York City MTA Subway. It is modeled after the displays inside the newest Subway cars." }
                    img src="/static/irlexample.png" alt="Example of an MTA Subway display inside a subway car" class="h-auto max-w-full my-4";
                    p class="prose" {
                        "If you enjoy this, please consider "
                        a class="underline underline-offset-2 hover:decoration-2" href="https://github.com/theryangeary/mta-display/issues" {
                            "contributing photos of real MTA displays or character layout improvements"
                        }
                        ", as the \"font\" used here is not true to the actual MTA display font and I need reference materials."
                    }
                }
            }
        }
    }
}

fn write_frames_to_gif_at_path(
    config: &BulbDisplayConfig,
    frames: &Vec<BulbDisplay>,
    path: std::path::PathBuf,
) -> Result<(), Box<dyn Error>> {
    let mut image_file = File::create(path)?;
    write_frames_to_gif(config, frames, &mut image_file)
}

fn write_frames_to_gif_in_memory(
    config: &BulbDisplayConfig,
    frames: &Vec<BulbDisplay>,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut buffer: Vec<u8> = vec![];
    write_frames_to_gif(config, frames, &mut buffer)?;
    Ok(buffer)
}

fn write_frames_to_gif(
    config: &BulbDisplayConfig,
    frames: &Vec<BulbDisplay>,
    output_gif: &mut dyn Write,
) -> Result<(), Box<dyn Error>> {
    let mut encoder = Encoder::new(output_gif, config.img_width(), config.img_height(), &[])?;
    encoder.set_repeat(Repeat::Infinite)?;
    Ok(for bulb_array in frames {
        let mut img: ImageBuffer<Rgb<_>, Vec<u8>> =
            ImageBuffer::new(config.img_width().into(), config.img_height().into());

        // Draw your graphics here
        // ... manipulate pixels in img ...
        for (row_num, row) in bulb_array.iter().enumerate() {
            for (col_num, rgb) in row.iter().enumerate() {
                draw_bulb(&mut img, &config, row_num as u16, col_num as u16, *rgb)
            }
        }

        // Add frame to GIF
        let mut frame =
            Frame::from_rgb_speed(config.img_width(), config.img_height(), &img.into_raw(), 30);
        frame.delay = 85;
        encoder.write_frame(&frame)?;
    })
}

fn generate_frames_for_message(
    config: &BulbDisplayConfig,
    train: types::Train,
    message_parts: Vec<&str>,
) -> Result<Vec<BulbDisplay>, Box<dyn Error>> {
    let mut frames = vec![];
    for msg in &message_parts {
        let mut bulb_array: BulbDisplay =
            vec![vec![Rgb([0, 0, 0]); config.num_bulb_cols.into()]; config.num_bulb_rows.into()];

        // draw a train bullet in the left edge of the bulb array
        let train_bullet = pattern::pattern_for_train(train);
        for (row_num, row) in train_bullet.iter().enumerate() {
            for (col_num, &rgb) in row.iter().enumerate() {
                bulb_array[row_num][col_num] = rgb;
            }
        }

        // write text into the bulb array
        write_text(&mut bulb_array, &msg)?;

        frames.push(bulb_array);
    }
    Ok(frames)
}

fn split_message_into_parts<'a>(config: &BulbDisplayConfig, message: &'a str) -> Vec<&'a str> {
    let mut message_parts = vec![];

    let mut start = 0;
    let mut last_space = 0;

    // Split message into parts based on max chars per row, without breaking
    // words unless the word is longer than max chars per row. Avoid adding
    // trailing whitespace to a row. Always break at newlines.
    for (i, chr) in message.char_indices() {
        if chr.is_whitespace() {
            last_space = i;
        }
        if i - start >= config.max_chars_per_row() as usize || chr == '\n' {
            if last_space > start {
                message_parts.push(&message[start..last_space]);
                start = last_space + 1; // skip the space
            } else {
                message_parts.push(&message[start..i]);
                start = i;
            }
        }
    }
    if start < message.len() {
        message_parts.push(&message[start..].trim_end());
    }

    message_parts
}

fn draw_bulb(
    img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    config: &BulbDisplayConfig,
    row_num: u16,
    col_num: u16,
    rgb: Rgb<u8>,
) {
    let top_left = (
        config.display_margin + (col_num * config.bulb_region_side_length()),
        config.display_margin + (row_num * config.bulb_region_side_length()),
    );
    let bottom_right = (
        top_left.0 + config.bulb_region_side_length(),
        top_left.1 + config.bulb_region_side_length(),
    );
    let center = (
        (top_left.0 + bottom_right.0) / 2,
        (top_left.1 + bottom_right.1) / 2,
    );

    for x in top_left.0..bottom_right.0 {
        for y in top_left.1..bottom_right.1 {
            if ((x.abs_diff(center.0) as u32).pow(2) + (y.abs_diff(center.1) as u32).pow(2)) as f64
                <= ((config.bulb_width() as f64) / 2.0).powi(2)
            {
                img[(x as u32, y as u32)] = rgb;
            } else {
                img[(x as u32, y as u32)] = Rgb([50, 50, 50]);
            }
        }
    }
}

/// Writes text into the bulb array starting after the bullet's fixed width.
/// Returns the number of characters from the message that were progressed.
/// Progressed here indicates that they were written into the bulb array, or skipped
/// if there was no pattern for that character.
fn write_text(
    bulb_array: &mut BulbDisplay,
    message: &str,
) -> Result<usize, Box<dyn std::error::Error>> {
    let mut ret = 0;

    let num_cols = bulb_array[0].len();
    let bullet_width = pattern::TRAIN_BULLET_PATTERN_WIDTH as usize
        + pattern::TRAIN_BULLET_PATTERN_SPACING as usize;

    // find width of all chars in message plus inter-character spacing
    let message_width: usize = message
        .chars()
        .map(|c| pattern::pattern_for_letter(c)[0].len() + pattern::LETTER_PATTERN_SPACING as usize)
        .sum();

    let left_pad = if message_width < (num_cols - bullet_width) {
        bullet_width + (num_cols - bullet_width - message_width) / 2
    } else {
        bullet_width
    };

    let mut next_char_start_column = left_pad;

    'CHARS: for (i, c) in message.chars().enumerate() {
        let char_pattern = pattern::pattern_for_letter(c);

        for (row_num, row) in char_pattern.iter().enumerate() {
            for (col_num, &rgb) in row.iter().enumerate() {
                let target_row = row_num;
                let target_col = next_char_start_column + col_num;
                if target_row >= bulb_array.len() || target_col >= bulb_array[0].len() {
                    break 'CHARS;
                }
                bulb_array[target_row][target_col] = rgb;
            }
        }

        next_char_start_column += char_pattern[0].len() + pattern::LETTER_PATTERN_SPACING as usize;

        ret += 1;
    }
    Ok(ret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_message_into_parts() {
        let config = BulbDisplayConfig::new(16, 160, 10, 4, 0.75);
        let message = "This is a test message to split into parts for the MTA sign GIF generator.";
        let parts = split_message_into_parts(&config, message);
        assert_eq!(parts.len(), 6);
        assert_eq!(parts[0], "This is a test");
        assert_eq!(parts[1], "message to");
        assert_eq!(parts[2], "split into");
        assert_eq!(parts[3], "parts for the");
        assert_eq!(parts[4], "MTA sign GIF");
        assert_eq!(parts[5], "generator.");
    }

    #[test]
    fn test_split_message_into_parts_long_word() {
        let config = BulbDisplayConfig::new(16, 160, 10, 4, 0.75);
        let message = "Thisisaverylongwordthatshouldbetested ";
        let parts = split_message_into_parts(&config, message);
        let chars_per_row = config.max_chars_per_row() as usize;
        assert_eq!(parts.len(), message.chars().count() / chars_per_row + 1);
        assert_eq!(parts[0], "Thisisaverylon");
        assert_eq!(parts[1], "gwordthatshoul");
        assert_eq!(parts[2], "dbetested");
    }

    #[test]
    fn test_split_message_into_parts_with_newlines() {
        let config = BulbDisplayConfig::new(16, 160, 10, 4, 0.75);
        let message = "This is a test\nmessage with newlines\nto split into parts.";
        let parts = split_message_into_parts(&config, message);
        assert_eq!(parts.len(), 5);
        assert_eq!(parts[0], "This is a test");
        assert_eq!(parts[1], "message with");
        assert_eq!(parts[2], "newlines");
        assert_eq!(parts[3], "to split into");
        assert_eq!(parts[4], "parts.");
    }

    #[test]
    fn test_split_message_into_parts_newlines() {
        let config = BulbDisplayConfig::new(16, 160, 10, 4, 0.75);
        let message = "new message
I want a newline
another newline";
        let parts = split_message_into_parts(&config, message);
        assert_eq!(parts.len(), 5);
        assert_eq!(parts[0], "new message");
        assert_eq!(parts[1], "I want a");
        assert_eq!(parts[2], "newline");
        assert_eq!(parts[3], "another");
        assert_eq!(parts[4], "newline");
    }
}
