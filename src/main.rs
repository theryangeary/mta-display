use axum::routing::{delete, post, put};
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
use axum::http::{header, HeaderMap, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::{routing::get, Router};
use axum::{Form, Json};
use maud::{html, Markup};
use serde::Deserialize;
use tokio::signal;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use types::BulbDisplay;
use types::BulbDisplayConfig;

use crate::db::SqliteDatabase;
use crate::types::{BulbDisplaySize, Train};

mod envvars {
        pub const DATABASE_URL: &str = "DATABASE_URL";
        pub const BASE_URL: &str = "BASE_URL";
        pub const NTFY_URL: &str = "NTFY_URL";
        pub const NTFY_TOPIC: &str = "NTFY_TOPIC";
        pub const NTFY_TOKEN: &str = "NTFY_TOKEN";
}

const DEFAULT_MESSAGE: &str = "Welcome to the MTA display generator";
const MISSING_SUBMITTER_NAME_FALLBACK: &str = "anonymous";

pub struct AppState {
    db: Arc<dyn db::Database>,
    notifier: Arc<dyn notification::NotificationService>,
    base_url: String,
}

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

    let database_url = env::var(envvars::DATABASE_URL).unwrap_or_else(|_| "sqlite:gallery.db".to_string());
    let db = Arc::new(SqliteDatabase::new(&database_url).await?);

    let base_url = env::var(envvars::BASE_URL).unwrap_or_else(|_| "http://localhost:3000".to_string());

    let notifier: Arc<dyn notification::NotificationService> = match env::var(envvars::NTFY_URL) {
        Ok(ntfy_url) => {
            let ntfy_topic =
                env::var(envvars::NTFY_TOPIC).expect("NTFY_TOPIC must be set if NTFY_URL is set");
            let ntfy_token =
                env::var(envvars::NTFY_TOKEN).expect("NTFY_TOKEN must be set if NTFY_URL is set");
            let client = reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .expect("failed to build reqwest client");
            tracing::info!("ntfy notifications enabled: {}/{}", ntfy_url, ntfy_topic);
            Arc::new(notification::NtfyNotificationService {
                client,
                ntfy_url,
                ntfy_topic,
                ntfy_token,
            })
        }
        Err(_) => {
            tracing::warn!("{} not set — submission notifications are disabled", envvars::NTFY_URL);
            Arc::new(notification::NoopNotificationService)
        }
    };

    let state = Arc::new(AppState {
        db,
        notifier,
        base_url,
    });

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
        .route(
            "/gallery/review/{id}/approve",
            put(put_gallery_review_approve),
        )
        .route(
            "/gallery/review/{id}/reject",
            delete(put_gallery_review_reject),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(state);

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

#[derive(Deserialize, Debug)]
struct GalleryEntryForm {
    message: String,
    train: Train,
    submitter_name: Option<String>,
    description: Option<String>,
}

async fn get_index_markup(Query(params): Query<HashMap<String, String>>) -> Markup {
    let display_message = params
        .get("message")
        .map(|s| {
            if s.is_empty() {
                DEFAULT_MESSAGE.into()
            } else {
                s.clone()
            }
        })
        .unwrap_or_else(|| DEFAULT_MESSAGE.into());

    let train = Train::from_str(&params.get("train").cloned().unwrap_or_else(|| "A".into()))
        .unwrap_or(Train::A);

    markup::index_markup(train, &display_message)
}

async fn get_gallery_entry() -> Markup {
    markup::get_gallery_entry_markup()
}

async fn post_gallery_entry(
    State(state): State<Arc<AppState>>,
    Form(gallery_entry_form): Form<GalleryEntryForm>,
) -> Result<Response, StatusCode> {
    let message = gallery_entry_form.message;
    let train_str = gallery_entry_form.train;
    let submitter_name = gallery_entry_form.submitter_name;
    let description = gallery_entry_form.description;

    // reject if message is empty or only whitespace
    if message.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    state
        .db
        .create_gallery_entry(
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

    let notifier = Arc::clone(&state.notifier);
    let notif_message = message.clone();
    let notif_train = gallery_entry_form.train.to_string();
    let notif_submitter = submitter_name.clone();
    let base_url = state.base_url.clone();
    let gif_path = types::GifPath::new(
        BulbDisplaySize::Small,
        gallery_entry_form.train,
        &message,
    )
    .to_url_path();

    tokio::spawn(async move {
        if let Err(e) = notifier
            .send_submission_notification(
                notif_submitter.as_deref(),
                &notif_train,
                &notif_message,
                &base_url,
                &gif_path,
            )
            .await
        {
            tracing::error!("failed to send submission notification: {}", e);
        }
    });

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("text/html"));
    headers.insert("hx-push-url", "/gallery".parse().unwrap());

    let markup = get_gallery_with_banner_markup(State(Arc::clone(&state))).await?;

    Ok((headers, markup).into_response())
}

async fn get_gallery_review(State(state): State<Arc<AppState>>) -> Result<Markup, StatusCode> {
    let entries = state.db.list_pending_gallery_entries().await.map_err(|e| {
        tracing::error!("failed to get gallery entries: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(markup::get_gallery_review_markup(entries))
}

async fn put_gallery_review_approve(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Markup, StatusCode> {
    state.db.approve_gallery_entry(id).await.map_err(|e| {
        tracing::error!("failed to approve gallery entry {}: {}", id, e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(html! {div{}})
}

async fn put_gallery_review_reject(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Markup, StatusCode> {
    state.db.reject_gallery_entry(id).await.map_err(|e| {
        tracing::error!("failed to reject gallery entry {}: {}", id, e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(html! {div{}})
}

async fn get_gallery(State(state): State<Arc<AppState>>) -> Result<Markup, StatusCode> {
    let entries = state
        .db
        .list_approved_gallery_entries()
        .await
        .map_err(|e| {
            tracing::error!("failed to get gallery entries: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(markup::get_gallery_markup(entries, None))
}

pub async fn get_gallery_with_banner_markup(
    State(state): State<Arc<AppState>>,
) -> Result<Response, StatusCode> {
    let entries = state
        .db
        .list_approved_gallery_entries()
        .await
        .map_err(|e| {
            tracing::error!("failed to get gallery entries: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("text/html"));
    headers.insert("hx-push-url", "/gallery".parse().unwrap());

    Ok((
        headers,
        markup::get_gallery_markup(
            entries,
            Some("Your submission has been received and is pending approval.".into()),
        ),
    )
        .into_response())
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
    let mut sanitized_message = generate_gif_form.message.trim().to_string();
    if sanitized_message.is_empty() {
        sanitized_message = DEFAULT_MESSAGE.into();
    }
    let encoded_message = urlencoding::encode(&sanitized_message);
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
        markup::gif_markup(generate_gif_form.train, &sanitized_message),
    )
        .into_response())
}

async fn get_gif_file(Path((size, train, message)): Path<(String, Train, String)>) -> Response {
    tracing::info!(
        "Generating GIF for size='{}', train='{:?}', message='{}'",
        size,
        train,
        message
    );
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

    'CHARS: for (_, c) in message.chars().enumerate() {
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

mod markup {
    use super::DEFAULT_MESSAGE;
    use crate::MISSING_SUBMITTER_NAME_FALLBACK;
    use crate::{models::GalleryEntry, types};
    use crate::types::{BulbDisplaySize, Train};
    use maud::{html, Markup, DOCTYPE};

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

    pub fn navbar_markup() -> Markup {
        html! {
            nav class=" bg-gray-200 p-4 mb-4 " {
                div class=" max-w-7xl mx-auto flex items-center justify-between " {
                    a href="/" class=" text-black font-bold text-lg " { "MTA Display Generator" }
                    div class=" space-x-4 " {
                        a href="/" class=" text-black hover:text-green-500 " { "Home" }
                        a href="/gallery" class=" text-black hover:text-green-500 " { "Gallery" }
                    }
                }
            }
        }
    }

    pub fn get_gallery_entry_markup() -> Markup {
        html! {
            (head("Create Gallery Entry"))
            body {
                (navbar_markup())
                div class=" flex justify-center " {
                    div
                        class="
                    grid content-center
                    max-w-4xl
                    mx-4
                    "
                    {
                        h1 { "Submit to Gallery" }

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
                                placeholder="Type your message here..."
                                required
                                oninput="this.setCustomValidity(this.value.trim() === '' ? 'Message cannot be empty or whitespace' : ''); this.reportValidity();"
                                {}
                            br;

                            label class=" flex w-100% block mb-2" for="train" {
                                span class="flex-grow" { "Train: " }
                                (select_train())
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

    pub fn get_gallery_review_markup(entries: Vec<GalleryEntry>) -> Markup {
        html! {
            (head("Gallery Review"))
            body {
                (navbar_markup())
                div class=" flex justify-center " {
                    div
                        class="
                    grid content-center
                    max-w-4xl
                    mx-4
                    "
                    {
                        h1 { "Gallery Review" }

                        @for entry in entries {
                            @let i = format!("gallery-entry-{}", entry.id);
                            div id=(i) class=" mb-8 " {
                                div class=" flex justify-center mb-1 " {
                                    img
                                        src=(types::GifPath::new(BulbDisplaySize::Small, entry.train, &entry.message).to_url_path())
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
                                            (MISSING_SUBMITTER_NAME_FALLBACK)
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

    pub fn get_gallery_markup(entries: Vec<GalleryEntry>, banner: Option<String>) -> Markup {
        html! {
            (head("Gallery"))
            body {
                (navbar_markup())
                div class=" flex justify-center " {
                    div
                        class="
                    grid content-center
                    max-w-4xl
                    mx-4
                    "
                    {
                        h1 class="mb-4" { "Gallery" }

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
                                        src=(types::GifPath::new(BulbDisplaySize::Small, entry.train, &entry.message).to_url_path())
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
                                            (MISSING_SUBMITTER_NAME_FALLBACK)
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
    pub fn gif_markup(train: Train, message: &str) -> Markup {
        let url_encoded_message = urlencoding::encode(message);
        html! {
            div id="mta-sign-gif" {
                div
                    class=" flex justify-center mb-4 "
                {
                    img
                        src=(
                            types::GifPath::new(BulbDisplaySize::Medium, train, message).to_url_path())
                        alt=(&format!("Generated MTA Display with message {}", message))
                        class="h-auto max-w-full"
                    ;
                }

                div class=" mb-8 flex justify-center " {
                    div class=" divide-x-1 divide-yellow-700 rounded-xl " {
                        button class="bg-yellow-500 text-black font-light text-sm py-2 px-4 hover:bg-yellow-600 rounded-l-xl" { a target="_blank" href=(types::GifPath::new(BulbDisplaySize::Small, train, message).to_url_path()) { "Small" } }
                        button class="bg-yellow-500 text-black font-light text-sm py-2 px-4 hover:bg-yellow-600 " { a target="_blank" href=(types::GifPath::new(BulbDisplaySize::Medium, train, message).to_url_path()) { "Medium" } }
                        button class="bg-yellow-500 text-black font-light text-sm py-2 px-4 hover:bg-yellow-600 " { a target="_blank" href=(types::GifPath::new(BulbDisplaySize::Large, train, message).to_url_path()) { "Large" } }
                        button class="bg-yellow-500 text-black font-light text-sm py-2 px-4 hover:bg-yellow-600 rounded-r-xl" { a target="_blank" href=(types::GifPath::new(BulbDisplaySize::XLarge, train, message).to_url_path()) { "Extra Large" } }
                    }
                }
            }
        }
    }

    pub fn index_markup(train: Train, display_message: &str) -> Markup {
        html! {
            (head("MTA Display Generator"))
            body {
                (navbar_markup())
                div class="flex justify-center" {
                    div
                        class="
                    grid content-center
                    max-w-lg
                    mx-4
                    "
                    {
                        h1 { "MTA Display Generator" }

                        (gif_markup(train, &display_message))

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
                                    @if !(display_message == DEFAULT_MESSAGE) {
                                        (display_message)
                                    }
                                }
                            br;

                            label class=" flex w-100% block mb-2" for="train" {
                                span class="flex-grow" { "Train: " }
                                (select_train())
                            }

                            button type="submit" class=" bg-yellow-500 text-black font-bold py-2 px-4 rounded hover:bg-yellow-600 " { "Generate" }
                        }

                        h2 { "Gallery" }
                        p class="prose" { "Check out the " a class="underline underline-offset-2 hover:decoration-2" href="/gallery" { "gallery" } " to see submissions from other users, or submit your own!" }

                        h2 { "About"}
                        p class="prose" { r#"
                        One day I was on the Subway, deep in thought, when I noticed the train car I was on had this
                        display for captions of the announcements. At that moment it was conveying a message about
                        how it is "#
                            span class="font-mono font-bold" {"unlawful to consume alcohol in the system"}
                            r#". The incongruity of the display that usually cycles through the time, destination, and next
                            stops showing "#
                            span class="font-mono font-bold" { r#""CONSUME" "ALCOHOL""# }
                            r#" threw me - and that's when I started pondering showing
                            arbitrary messages on those displays."#
                        }
                        p class="prose" { r#"
                        From this rabbit hole this fun side project that generates GIFs that simulate those same displays
                        was born. It is modeled after the displays inside the newest Subway cars which have been around
                        since 2023."#
                        }
                        img src="/static/irlexample.png" alt="Example of an MTA Subway display inside a subway car" class="h-auto max-w-full my-4";
                        p class="prose" { r#"
                            Send your friends a funny message or leave one for future visitors to see in the gallery."#
                        }
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

    fn select_train() -> Markup {
        html! {
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
    }
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

mod notification {
    use anyhow::Result;
    use crate::MISSING_SUBMITTER_NAME_FALLBACK;

    #[async_trait::async_trait]
    pub trait NotificationService: Send + Sync {
        async fn send_submission_notification(
            &self,
            submitter_name: Option<&str>,
            train: &str,
            message: &str,
            base_url: &str,
            gif_path: &str,
        ) -> Result<()>;
    }

    pub struct NtfyNotificationService {
        pub client: reqwest::Client,
        pub ntfy_url: String,
        pub ntfy_topic: String,
        pub ntfy_token: String,
    }

    #[async_trait::async_trait]
    impl NotificationService for NtfyNotificationService {
        async fn send_submission_notification(
            &self,
            submitter_name: Option<&str>,
            train: &str,
            message: &str,
            base_url: &str,
            gif_path: &str,
        ) -> Result<()> {
            let name = submitter_name.unwrap_or(MISSING_SUBMITTER_NAME_FALLBACK);
            let body = format!(
                "New gallery submission!\nSubmitter: {}\nTrain: {}\nMessage: {}\nPreview: {}{}\nReview: {}/gallery/review",
                name, train, message, base_url, gif_path, base_url
            );

            let url = format!("{}/{}", self.ntfy_url, self.ntfy_topic);

            self.client
                .post(&url)
                .bearer_auth(&self.ntfy_token)
                .header("Content-Type", "text/plain")
                .body(body)
                .send()
                .await?
                .error_for_status()?;

            Ok(())
        }
    }

    pub struct NoopNotificationService;

    #[async_trait::async_trait]
    impl NotificationService for NoopNotificationService {
        async fn send_submission_notification(
            &self,
            _submitter_name: Option<&str>,
            _train: &str,
            _message: &str,
            _base_url: &str,
            _gif_path: &str,
        ) -> Result<()> {
            Ok(())
        }
    }
}

mod db {
    use std::str::FromStr;

    use anyhow::Result;
    use sqlx::{
        migrate::MigrateDatabase,
        sqlite::{SqliteConnectOptions, SqlitePool},
        ConnectOptions, FromRow, Row, Sqlite,
    };

    use crate::{models::GalleryEntry, types::Train};

    #[async_trait::async_trait]
    pub trait Database: Send + Sync {
        async fn create_gallery_entry(
            &self,
            message: &str,
            train: Train,
            submitter_name: Option<&str>,
            description: Option<&str>,
        ) -> Result<()>;

        async fn approve_gallery_entry(&self, entry_id: i64) -> Result<()>;

        async fn reject_gallery_entry(&self, entry_id: i64) -> Result<()>;

        async fn list_pending_gallery_entries(&self) -> Result<Vec<GalleryEntry>>;

        async fn list_approved_gallery_entries(&self) -> Result<Vec<GalleryEntry>>;
    }
    pub struct SqliteDatabase {
        pub pool: SqlitePool,
    }

    impl SqliteDatabase {
        pub async fn new(database_url: &str) -> Result<Self> {
            if !Sqlite::database_exists(database_url).await.unwrap_or(false) {
                Sqlite::create_database(database_url).await?;
            }

            let options = SqliteConnectOptions::from_str(database_url)?
                .log_statements(tracing::log::LevelFilter::Trace)
                .foreign_keys(true);

            let pool = SqlitePool::connect_with(options).await?;

            let db = SqliteDatabase { pool };
            db.migrate().await?;

            Ok(db)
        }

        async fn migrate(&self) -> Result<()> {
            sqlx::migrate!("./migrations").run(&self.pool).await?;

            Ok(())
        }
    }

    #[async_trait::async_trait]
    impl Database for SqliteDatabase {
        async fn create_gallery_entry(
            &self,
            message: &str,
            train: Train,
            submitter_name: Option<&str>,
            description: Option<&str>,
        ) -> Result<()> {
            sqlx::query(
            "INSERT INTO gallery_entries (message, train, submitter_name, description) VALUES (?, ?, ?, ?)",
        )
        .bind(message)
        .bind(train as Train)
        .bind(submitter_name)
        .bind(description)
        .execute(&self.pool)
        .await?;

            Ok(())
        }

        async fn approve_gallery_entry(&self, entry_id: i64) -> Result<()> {
            sqlx::query("UPDATE gallery_entries SET approved_at = CURRENT_TIMESTAMP WHERE id = ?")
                .bind(entry_id)
                .execute(&self.pool)
                .await?;

            Ok(())
        }

        async fn reject_gallery_entry(&self, entry_id: i64) -> Result<()> {
            sqlx::query("DELETE FROM gallery_entries WHERE id = ?")
                .bind(entry_id)
                .execute(&self.pool)
                .await?;

            Ok(())
        }

        async fn list_approved_gallery_entries(&self) -> Result<Vec<GalleryEntry>> {
            sqlx::query_as::<_, GalleryEntry>(
            "SELECT id, message, train, submitter_name, submitted_at, approved_at, description FROM gallery_entries WHERE approved_at IS NOT NULL",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.into())
        }

        async fn list_pending_gallery_entries(&self) -> Result<Vec<GalleryEntry>> {
            sqlx::query_as::<_, GalleryEntry>(
            "SELECT id, message, train, submitter_name, submitted_at, approved_at, description FROM gallery_entries WHERE approved_at IS NULL",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.into())
        }
    }
}

mod models {
    use serde::{Deserialize, Serialize};
    use sqlx::prelude::FromRow;

    use crate::types::Train;

    #[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
    pub struct GalleryEntry {
        pub id: i64,
        pub message: String,
        pub train: Train,
        pub submitter_name: Option<String>,
        pub submitted_at: chrono::NaiveDateTime,
        pub approved_at: Option<chrono::NaiveDateTime>,
        pub description: Option<String>,
    }
}

mod types {
    use std::str::FromStr;

    use image::Rgb;
    use serde::{Deserialize, Serialize};
    use sqlx::{Decode, Sqlite};

    use crate::pattern;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::EnumString, strum::ToString)]
    pub enum BulbDisplaySize {
        #[strum(serialize = "xs")]
        XSmall,
        #[strum(serialize = "sm")]
        Small,
        #[strum(serialize = "md")]
        Medium,
        #[strum(serialize = "lg")]
        Large,
        #[strum(serialize = "xl")]
        XLarge,
    }

    pub type BulbDisplay = Vec<Vec<Rgb<u8>>>;

    pub struct BulbDisplayConfig {
        pub num_bulb_rows: u16,
        pub num_bulb_cols: u16,
        /// the number of pixels around the border of the display that do not have bulbs on them
        pub display_margin: u16,
        /// the resulting image height in pixels
        img_height: u16,
        /// the resulting image width in pixels
        img_width: u16,
        /// bulb_size_ratio is the ratio of the bulb diameter to the bounding box width
        bulb_size_ratio: f64,
    }

    impl BulbDisplayConfig {
        pub fn new(
            num_bulb_rows: u16,
            num_bulb_cols: u16,
            display_margin: u16,
            bulb_bounding_box_size: u16,
            bulb_size_ratio: f64,
        ) -> Self {
            let height = (num_bulb_rows * bulb_bounding_box_size) + (2 * display_margin);
            let width = (num_bulb_cols * bulb_bounding_box_size) + (2 * display_margin);

            Self {
                num_bulb_rows,
                num_bulb_cols,
                display_margin,
                img_height: height,
                img_width: width,
                bulb_size_ratio,
            }
        }

        pub fn new_from_size(size: BulbDisplaySize) -> Self {
            match size {
                BulbDisplaySize::XSmall => Self::new(16, 160, 4, 1, 0.75),
                BulbDisplaySize::Small => Self::new(16, 160, 4, 2, 1.0),
                BulbDisplaySize::Medium => Self::new(16, 160, 4, 4, 0.75),
                BulbDisplaySize::Large => Self::new(16, 160, 8, 8, 0.75),
                BulbDisplaySize::XLarge => Self::new(16, 160, 10, 16, 0.75),
            }
        }

        pub fn bulb_region_side_length(&self) -> u16 {
            (self.img_height - (2 * self.display_margin)) / self.num_bulb_rows
        }

        pub fn bulb_width(&self) -> u16 {
            (self.bulb_region_side_length() as f64 * self.bulb_size_ratio) as u16
        }

        pub fn img_width(&self) -> u16 {
            self.img_width
        }

        pub fn img_height(&self) -> u16 {
            self.img_height
        }

        pub fn max_chars_per_row(&self) -> u16 {
            (self.num_bulb_cols - pattern::TRAIN_BULLET_PATTERN_WIDTH)
                / pattern::LETTER_PATTERN_SLOT_WIDTH
        }
    }

    #[derive(
        Debug,
        Clone,
        Copy,
        PartialEq,
        Eq,
        Hash,
        Serialize,
        Deserialize,
        strum::EnumString,
        strum::Display,
        sqlx::Type,
    )]
    pub enum Train {
        One,
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        A,
        B,
        C,
        D,
        E,
        F,
        G,
        J,
        L,
        M,
        N,
        Q,
        R,
        S,
        W,
        Z,
    }

    pub struct GifPath {
        pub size: BulbDisplaySize,
        pub train: Train,
        pub message: String,
    }

    impl GifPath {
        pub fn new(size: BulbDisplaySize, train: Train, message: &str) -> Self {
            Self {
                size,
                train,
                message: message.to_owned(),
            }
        }

        pub fn to_url_path(&self) -> String {
            format!(
                "/gif/{}/{}/{}",
                self.size.to_string(),
                self.train,
                urlencoding::encode(&self.message)
            )
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::str::FromStr;

        #[test]
        fn test_bulb_display_size_from_str() {
            assert_eq!(
                BulbDisplaySize::from_str("xs").unwrap(),
                BulbDisplaySize::XSmall
            );
            assert_eq!(
                BulbDisplaySize::from_str("sm").unwrap(),
                BulbDisplaySize::Small
            );
            assert_eq!(
                BulbDisplaySize::from_str("md").unwrap(),
                BulbDisplaySize::Medium
            );
            assert_eq!(
                BulbDisplaySize::from_str("lg").unwrap(),
                BulbDisplaySize::Large
            );
            assert_eq!(
                BulbDisplaySize::from_str("xl").unwrap(),
                BulbDisplaySize::XLarge
            );
        }

        #[test]
        fn test_bulb_display_size_to_debug() {
            assert_eq!(BulbDisplaySize::XSmall.to_string(), "xs");
            assert_eq!(BulbDisplaySize::Small.to_string(), "sm");
            assert_eq!(BulbDisplaySize::Medium.to_string(), "md");
            assert_eq!(BulbDisplaySize::Large.to_string(), "lg");
            assert_eq!(BulbDisplaySize::XLarge.to_string(), "xl");
        }

        #[test]
        fn test_url_generation() {
            assert_eq!(
                GifPath::new(BulbDisplaySize::Small, Train::A, "test message").to_url_path(),
                "/gif/sm/A/test%20message"
            )
        }

        #[test]
        fn test_train_from_str() {
            assert_eq!(Train::from_str("One").unwrap(), Train::One);
            assert_eq!(Train::from_str("A").unwrap(), Train::A);
            assert_eq!(Train::from_str("Z").unwrap(), Train::Z);
        }
    }
}

mod pattern {
    use image::Rgb;

    use crate::types::{BulbDisplay, Train};

    pub const LETTER_PATTERN_WIDTH: u16 = 8;
    #[allow(dead_code)]
    pub const LETTER_PATTERN_HEIGHT: u16 = 16;
    pub const LETTER_PATTERN_SPACING: u16 = 2;
    pub const LETTER_PATTERN_SLOT_WIDTH: u16 = LETTER_PATTERN_WIDTH + LETTER_PATTERN_SPACING;
    pub const TRAIN_BULLET_PATTERN_WIDTH: u16 = 16;
    pub const TRAIN_BULLET_PATTERN_SPACING: u16 = 4;
    #[allow(dead_code)]
    pub const TRAIN_BULLET_PATTERN_HEIGHT: u16 = 16;

    // short names for colors used in patterns, makes patterns easier to read as they becomes somewhat 1:1 aspect ratio
    // Color definitions for MTA lines
    const W1: Rgb<u8> = Rgb([255, 255, 255]); // White (letters/numbers)
    const B0: Rgb<u8> = Rgb([0, 0, 0]); // Black (background)
    const BL: Rgb<u8> = Rgb([0, 204, 255]); // ACE blue
    const RD: Rgb<u8> = Rgb([255, 0, 0]); // 123 red
    const GR: Rgb<u8> = Rgb([0, 147, 0]); // 456 green
    const PU: Rgb<u8> = Rgb([185, 51, 173]); // 7 purple
    const OR: Rgb<u8> = Rgb([255, 99, 25]); // BDFM orange
    const LG: Rgb<u8> = Rgb([108, 190, 69]); // G light green
    const BR: Rgb<u8> = Rgb([153, 102, 51]); // JZ brown
    const GY: Rgb<u8> = Rgb([167, 169, 172]); // LS gray
    const YE: Rgb<u8> = Rgb([252, 204, 10]); // NQRW yellow

    pub fn pattern_for_train(train: Train) -> BulbDisplay {
        match train {
            Train::One => one_train_bullet_pattern(),
            Train::Two => two_train_bullet_pattern(),
            Train::Three => three_train_bullet_pattern(),
            Train::Four => four_train_bullet_pattern(),
            Train::Five => five_train_bullet_pattern(),
            Train::Six => six_train_bullet_pattern(),
            Train::Seven => seven_train_bullet_pattern(),
            Train::A => a_train_bullet_pattern(),
            Train::B => b_train_bullet_pattern(),
            Train::C => c_train_bullet_pattern(),
            Train::D => d_train_bullet_pattern(),
            Train::E => e_train_bullet_pattern(),
            Train::F => f_train_bullet_pattern(),
            Train::G => g_train_bullet_pattern(),
            Train::J => j_train_bullet_pattern(),
            Train::L => l_train_bullet_pattern(),
            Train::M => m_train_bullet_pattern(),
            Train::N => n_train_bullet_pattern(),
            Train::Q => q_train_bullet_pattern(),
            Train::R => r_train_bullet_pattern(),
            Train::S => s_train_bullet_pattern(),
            Train::W => w_train_bullet_pattern(),
            Train::Z => z_train_bullet_pattern(),
        }
    }

    fn one_train_bullet_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![
                B0, B0, B0, B0, B0, RD, RD, W1, W1, RD, RD, B0, B0, B0, B0, B0,
            ],
            vec![
                B0, B0, B0, RD, RD, RD, W1, W1, W1, RD, RD, RD, RD, B0, B0, B0,
            ],
            vec![
                B0, B0, RD, RD, RD, W1, W1, W1, W1, RD, RD, RD, RD, RD, B0, B0,
            ],
            vec![
                B0, RD, RD, RD, W1, W1, RD, W1, W1, RD, RD, RD, RD, RD, RD, B0,
            ],
            vec![
                B0, RD, RD, RD, RD, RD, RD, W1, W1, RD, RD, RD, RD, RD, RD, B0,
            ],
            vec![
                RD, RD, RD, RD, RD, RD, RD, W1, W1, RD, RD, RD, RD, RD, RD, RD,
            ],
            vec![
                RD, RD, RD, RD, RD, RD, RD, W1, W1, RD, RD, RD, RD, RD, RD, RD,
            ],
            vec![
                RD, RD, RD, RD, RD, RD, RD, W1, W1, RD, RD, RD, RD, RD, RD, RD,
            ],
            vec![
                RD, RD, RD, RD, RD, RD, RD, W1, W1, RD, RD, RD, RD, RD, RD, RD,
            ],
            vec![
                RD, RD, RD, RD, RD, RD, RD, W1, W1, RD, RD, RD, RD, RD, RD, RD,
            ],
            vec![
                RD, RD, RD, RD, RD, RD, RD, W1, W1, RD, RD, RD, RD, RD, RD, RD,
            ],
            vec![
                B0, RD, RD, RD, RD, RD, RD, W1, W1, RD, RD, RD, RD, RD, RD, B0,
            ],
            vec![
                B0, RD, RD, RD, RD, RD, RD, W1, W1, RD, RD, RD, RD, RD, RD, B0,
            ],
            vec![
                B0, B0, RD, RD, RD, RD, RD, W1, W1, RD, RD, RD, RD, RD, B0, B0,
            ],
            vec![
                B0, B0, B0, RD, RD, RD, RD, W1, W1, RD, RD, RD, RD, B0, B0, B0,
            ],
            vec![
                B0, B0, B0, B0, B0, RD, RD, W1, W1, RD, RD, B0, B0, B0, B0, B0,
            ],
        ];
        pattern
    }

    fn two_train_bullet_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![
                B0, B0, B0, B0, B0, RD, RD, RD, RD, RD, RD, B0, B0, B0, B0, B0,
            ],
            vec![
                B0, B0, B0, RD, RD, RD, W1, W1, W1, W1, RD, RD, RD, B0, B0, B0,
            ],
            vec![
                B0, B0, RD, RD, RD, W1, W1, W1, W1, W1, W1, RD, RD, RD, B0, B0,
            ],
            vec![
                B0, RD, RD, RD, W1, W1, RD, RD, RD, RD, W1, W1, RD, RD, RD, B0,
            ],
            vec![
                B0, RD, RD, RD, W1, W1, RD, RD, RD, RD, W1, W1, RD, RD, RD, B0,
            ],
            vec![
                RD, RD, RD, RD, RD, RD, RD, RD, RD, RD, W1, W1, RD, RD, RD, RD,
            ],
            vec![
                RD, RD, RD, RD, RD, RD, RD, RD, RD, W1, W1, RD, RD, RD, RD, RD,
            ],
            vec![
                RD, RD, RD, RD, RD, RD, RD, RD, W1, W1, RD, RD, RD, RD, RD, RD,
            ],
            vec![
                RD, RD, RD, RD, RD, RD, RD, W1, W1, RD, RD, RD, RD, RD, RD, RD,
            ],
            vec![
                RD, RD, RD, RD, RD, RD, W1, W1, RD, RD, RD, RD, RD, RD, RD, RD,
            ],
            vec![
                RD, RD, RD, RD, RD, W1, W1, RD, RD, RD, RD, RD, RD, RD, RD, RD,
            ],
            vec![
                B0, RD, RD, RD, W1, W1, RD, RD, RD, RD, RD, RD, RD, RD, RD, B0,
            ],
            vec![
                B0, RD, RD, RD, W1, W1, W1, W1, W1, W1, W1, W1, RD, RD, RD, B0,
            ],
            vec![
                B0, B0, RD, RD, W1, W1, W1, W1, W1, W1, W1, W1, RD, RD, B0, B0,
            ],
            vec![
                B0, B0, B0, RD, RD, RD, RD, RD, RD, RD, RD, RD, RD, B0, B0, B0,
            ],
            vec![
                B0, B0, B0, B0, B0, RD, RD, RD, RD, RD, RD, B0, B0, B0, B0, B0,
            ],
        ];
        pattern
    }

    fn three_train_bullet_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![
                B0, B0, B0, B0, B0, RD, RD, RD, RD, RD, RD, B0, B0, B0, B0, B0,
            ],
            vec![
                B0, B0, B0, RD, RD, RD, W1, W1, W1, W1, RD, RD, RD, B0, B0, B0,
            ],
            vec![
                B0, B0, RD, RD, RD, W1, W1, W1, W1, W1, W1, RD, RD, RD, B0, B0,
            ],
            vec![
                B0, RD, RD, RD, W1, W1, RD, RD, RD, RD, W1, W1, RD, RD, RD, B0,
            ],
            vec![
                B0, RD, RD, RD, W1, W1, RD, RD, RD, RD, W1, W1, RD, RD, RD, B0,
            ],
            vec![
                RD, RD, RD, RD, RD, RD, RD, RD, RD, RD, W1, W1, RD, RD, RD, RD,
            ],
            vec![
                RD, RD, RD, RD, RD, RD, RD, RD, RD, RD, W1, RD, RD, RD, RD, RD,
            ],
            vec![
                RD, RD, RD, RD, RD, RD, RD, RD, W1, W1, RD, RD, RD, RD, RD, RD,
            ],
            vec![
                RD, RD, RD, RD, RD, RD, RD, RD, W1, W1, RD, RD, RD, RD, RD, RD,
            ],
            vec![
                RD, RD, RD, RD, RD, RD, RD, RD, RD, RD, W1, RD, RD, RD, RD, RD,
            ],
            vec![
                RD, RD, RD, RD, RD, RD, RD, RD, RD, RD, W1, W1, RD, RD, RD, RD,
            ],
            vec![
                B0, RD, RD, RD, W1, W1, RD, RD, RD, RD, W1, W1, RD, RD, RD, B0,
            ],
            vec![
                B0, RD, RD, RD, W1, W1, RD, RD, RD, RD, W1, W1, RD, RD, RD, B0,
            ],
            vec![
                B0, B0, RD, RD, RD, W1, W1, W1, W1, W1, W1, RD, RD, RD, B0, B0,
            ],
            vec![
                B0, B0, B0, RD, RD, RD, W1, W1, W1, W1, RD, RD, RD, B0, B0, B0,
            ],
            vec![
                B0, B0, B0, B0, B0, RD, RD, RD, RD, RD, RD, B0, B0, B0, B0, B0,
            ],
        ];
        pattern
    }

    fn four_train_bullet_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![
                B0, B0, B0, B0, B0, GR, GR, GR, GR, GR, GR, B0, B0, B0, B0, B0,
            ],
            vec![
                B0, B0, B0, GR, GR, GR, GR, GR, GR, W1, W1, GR, GR, B0, B0, B0,
            ],
            vec![
                B0, B0, GR, GR, GR, GR, GR, GR, W1, W1, W1, GR, GR, GR, B0, B0,
            ],
            vec![
                B0, GR, GR, GR, GR, GR, GR, W1, W1, W1, W1, GR, GR, GR, GR, B0,
            ],
            vec![
                B0, GR, GR, GR, GR, GR, GR, W1, GR, W1, W1, GR, GR, GR, GR, B0,
            ],
            vec![
                GR, GR, GR, GR, GR, GR, W1, W1, GR, W1, W1, GR, GR, GR, GR, GR,
            ],
            vec![
                GR, GR, GR, GR, GR, GR, W1, GR, GR, W1, W1, GR, GR, GR, GR, GR,
            ],
            vec![
                GR, GR, GR, GR, GR, W1, W1, GR, GR, W1, W1, GR, GR, GR, GR, GR,
            ],
            vec![
                GR, GR, GR, GR, GR, W1, GR, GR, GR, W1, W1, GR, GR, GR, GR, GR,
            ],
            vec![
                GR, GR, GR, GR, W1, W1, GR, GR, GR, W1, W1, GR, GR, GR, GR, GR,
            ],
            vec![
                GR, GR, GR, GR, W1, W1, W1, W1, W1, W1, W1, W1, GR, GR, GR, GR,
            ],
            vec![
                B0, GR, GR, GR, W1, W1, W1, W1, W1, W1, W1, W1, GR, GR, GR, B0,
            ],
            vec![
                B0, GR, GR, GR, GR, GR, GR, GR, GR, W1, W1, GR, GR, GR, GR, B0,
            ],
            vec![
                B0, B0, GR, GR, GR, GR, GR, GR, GR, W1, W1, GR, GR, GR, B0, B0,
            ],
            vec![
                B0, B0, B0, GR, GR, GR, GR, GR, GR, W1, W1, GR, GR, B0, B0, B0,
            ],
            vec![
                B0, B0, B0, B0, B0, GR, GR, GR, GR, GR, GR, B0, B0, B0, B0, B0,
            ],
        ];
        pattern
    }

    fn five_train_bullet_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![
                B0, B0, B0, B0, B0, GR, GR, GR, GR, GR, GR, B0, B0, B0, B0, B0,
            ],
            vec![
                B0, B0, B0, GR, GR, GR, GR, GR, GR, GR, GR, GR, GR, B0, B0, B0,
            ],
            vec![
                B0, B0, GR, GR, W1, W1, W1, W1, W1, W1, W1, W1, GR, GR, B0, B0,
            ],
            vec![
                B0, GR, GR, GR, W1, W1, W1, W1, W1, W1, W1, W1, GR, GR, GR, B0,
            ],
            vec![
                B0, GR, GR, GR, W1, W1, GR, GR, GR, GR, GR, GR, GR, GR, GR, B0,
            ],
            vec![
                GR, GR, GR, GR, W1, W1, GR, GR, GR, GR, GR, GR, GR, GR, GR, GR,
            ],
            vec![
                GR, GR, GR, GR, W1, W1, GR, GR, GR, GR, GR, GR, GR, GR, GR, GR,
            ],
            vec![
                GR, GR, GR, GR, W1, W1, W1, W1, W1, W1, GR, GR, GR, GR, GR, GR,
            ],
            vec![
                GR, GR, GR, GR, W1, W1, W1, W1, W1, W1, W1, GR, GR, GR, GR, GR,
            ],
            vec![
                GR, GR, GR, GR, GR, GR, GR, GR, GR, GR, W1, W1, GR, GR, GR, GR,
            ],
            vec![
                GR, GR, GR, GR, GR, GR, GR, GR, GR, GR, W1, W1, GR, GR, GR, GR,
            ],
            vec![
                B0, GR, GR, GR, W1, W1, GR, GR, GR, GR, W1, W1, GR, GR, GR, B0,
            ],
            vec![
                B0, GR, GR, GR, W1, W1, GR, GR, GR, GR, W1, W1, GR, GR, GR, B0,
            ],
            vec![
                B0, B0, GR, GR, GR, W1, W1, W1, W1, W1, W1, GR, GR, GR, B0, B0,
            ],
            vec![
                B0, B0, B0, GR, GR, GR, W1, W1, W1, W1, GR, GR, GR, B0, B0, B0,
            ],
            vec![
                B0, B0, B0, B0, B0, GR, GR, GR, GR, GR, GR, B0, B0, B0, B0, B0,
            ],
        ];
        pattern
    }

    fn six_train_bullet_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![
                B0, B0, B0, B0, B0, GR, GR, GR, GR, GR, GR, B0, B0, B0, B0, B0,
            ],
            vec![
                B0, B0, B0, GR, GR, GR, GR, GR, GR, GR, GR, GR, GR, B0, B0, B0,
            ],
            vec![
                B0, B0, GR, GR, GR, GR, W1, W1, W1, W1, GR, GR, GR, GR, B0, B0,
            ],
            vec![
                B0, GR, GR, GR, GR, W1, W1, W1, W1, W1, W1, GR, GR, GR, GR, B0,
            ],
            vec![
                B0, GR, GR, GR, W1, W1, GR, GR, GR, GR, GR, GR, GR, GR, GR, B0,
            ],
            vec![
                GR, GR, GR, GR, W1, W1, GR, GR, GR, GR, GR, GR, GR, GR, GR, GR,
            ],
            vec![
                GR, GR, GR, GR, W1, W1, GR, GR, GR, GR, GR, GR, GR, GR, GR, GR,
            ],
            vec![
                GR, GR, GR, GR, W1, W1, W1, W1, W1, W1, GR, GR, GR, GR, GR, GR,
            ],
            vec![
                GR, GR, GR, GR, W1, W1, W1, W1, W1, W1, W1, GR, GR, GR, GR, GR,
            ],
            vec![
                GR, GR, GR, GR, W1, W1, GR, GR, GR, GR, W1, W1, GR, GR, GR, GR,
            ],
            vec![
                GR, GR, GR, GR, W1, W1, GR, GR, GR, GR, W1, W1, GR, GR, GR, GR,
            ],
            vec![
                B0, GR, GR, GR, W1, W1, GR, GR, GR, GR, W1, W1, GR, GR, GR, B0,
            ],
            vec![
                B0, GR, GR, GR, W1, W1, GR, GR, GR, GR, W1, W1, GR, GR, GR, B0,
            ],
            vec![
                B0, B0, GR, GR, GR, W1, W1, W1, W1, W1, W1, GR, GR, GR, B0, B0,
            ],
            vec![
                B0, B0, B0, GR, GR, GR, W1, W1, W1, W1, GR, GR, GR, B0, B0, B0,
            ],
            vec![
                B0, B0, B0, B0, B0, GR, GR, GR, GR, GR, GR, B0, B0, B0, B0, B0,
            ],
        ];
        pattern
    }

    fn seven_train_bullet_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![
                B0, B0, B0, B0, B0, PU, PU, PU, PU, PU, PU, B0, B0, B0, B0, B0,
            ],
            vec![
                B0, B0, B0, PU, PU, PU, PU, PU, PU, PU, PU, PU, PU, B0, B0, B0,
            ],
            vec![
                B0, B0, PU, PU, PU, PU, PU, PU, PU, PU, PU, PU, PU, PU, B0, B0,
            ],
            vec![
                B0, PU, PU, PU, W1, W1, W1, W1, W1, W1, W1, W1, PU, PU, PU, B0,
            ],
            vec![
                B0, PU, PU, PU, W1, W1, W1, W1, W1, W1, W1, W1, PU, PU, PU, B0,
            ],
            vec![
                PU, PU, PU, PU, PU, PU, PU, PU, PU, PU, W1, W1, PU, PU, PU, PU,
            ],
            vec![
                PU, PU, PU, PU, PU, PU, PU, PU, PU, W1, W1, PU, PU, PU, PU, PU,
            ],
            vec![
                PU, PU, PU, PU, PU, PU, PU, PU, W1, W1, PU, PU, PU, PU, PU, PU,
            ],
            vec![
                PU, PU, PU, PU, PU, PU, PU, W1, W1, PU, PU, PU, PU, PU, PU, PU,
            ],
            vec![
                PU, PU, PU, PU, PU, PU, W1, W1, PU, PU, PU, PU, PU, PU, PU, PU,
            ],
            vec![
                PU, PU, PU, PU, PU, W1, W1, PU, PU, PU, PU, PU, PU, PU, PU, PU,
            ],
            vec![
                B0, PU, PU, PU, W1, W1, PU, PU, PU, PU, PU, PU, PU, PU, PU, B0,
            ],
            vec![
                B0, PU, PU, PU, W1, PU, PU, PU, PU, PU, PU, PU, PU, PU, PU, B0,
            ],
            vec![
                B0, B0, PU, PU, PU, PU, PU, PU, PU, PU, PU, PU, PU, PU, B0, B0,
            ],
            vec![
                B0, B0, B0, PU, PU, PU, PU, PU, PU, PU, PU, PU, PU, B0, B0, B0,
            ],
            vec![
                B0, B0, B0, B0, B0, PU, PU, PU, PU, PU, PU, B0, B0, B0, B0, B0,
            ],
        ];
        pattern
    }

    fn a_train_bullet_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![
                B0, B0, B0, B0, B0, BL, BL, BL, BL, BL, BL, B0, B0, B0, B0, B0,
            ],
            vec![
                B0, B0, B0, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, B0, B0, B0,
            ],
            vec![
                B0, B0, BL, BL, BL, BL, BL, W1, W1, BL, BL, BL, BL, BL, B0, B0,
            ],
            vec![
                B0, BL, BL, BL, BL, BL, BL, W1, W1, BL, BL, BL, BL, BL, BL, B0,
            ],
            vec![
                B0, BL, BL, BL, BL, BL, W1, W1, W1, W1, BL, BL, BL, BL, BL, B0,
            ],
            vec![
                BL, BL, BL, BL, BL, W1, W1, BL, BL, W1, W1, BL, BL, BL, BL, BL,
            ],
            vec![
                BL, BL, BL, BL, BL, W1, W1, BL, BL, W1, W1, BL, BL, BL, BL, BL,
            ],
            vec![
                BL, BL, BL, BL, BL, W1, W1, BL, BL, W1, W1, BL, BL, BL, BL, BL,
            ],
            vec![
                BL, BL, BL, BL, W1, W1, BL, BL, BL, BL, W1, W1, BL, BL, BL, BL,
            ],
            vec![
                BL, BL, BL, BL, W1, W1, W1, W1, W1, W1, W1, W1, BL, BL, BL, BL,
            ],
            vec![
                BL, BL, BL, BL, W1, W1, W1, W1, W1, W1, W1, W1, BL, BL, BL, BL,
            ],
            vec![
                B0, BL, BL, W1, W1, BL, BL, BL, BL, BL, BL, W1, W1, BL, BL, B0,
            ],
            vec![
                B0, BL, BL, W1, W1, BL, BL, BL, BL, BL, BL, W1, W1, BL, BL, B0,
            ],
            vec![
                B0, B0, BL, W1, W1, BL, BL, BL, BL, BL, BL, W1, W1, BL, B0, B0,
            ],
            vec![
                B0, B0, B0, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, B0, B0, B0,
            ],
            vec![
                B0, B0, B0, B0, B0, BL, BL, BL, BL, BL, BL, B0, B0, B0, B0, B0,
            ],
        ];
        pattern
    }

    fn b_train_bullet_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![
                B0, B0, B0, B0, B0, OR, OR, OR, OR, OR, OR, B0, B0, B0, B0, B0,
            ],
            vec![
                B0, B0, B0, OR, OR, OR, OR, OR, OR, OR, OR, OR, OR, B0, B0, B0,
            ],
            vec![
                B0, B0, OR, OR, W1, W1, W1, W1, W1, W1, OR, OR, OR, OR, B0, B0,
            ],
            vec![
                B0, OR, OR, OR, W1, W1, W1, W1, W1, W1, W1, OR, OR, OR, OR, B0,
            ],
            vec![
                B0, OR, OR, OR, W1, W1, OR, OR, OR, OR, W1, W1, OR, OR, OR, B0,
            ],
            vec![
                OR, OR, OR, OR, W1, W1, OR, OR, OR, OR, W1, W1, OR, OR, OR, OR,
            ],
            vec![
                OR, OR, OR, OR, W1, W1, OR, OR, OR, OR, W1, OR, OR, OR, OR, OR,
            ],
            vec![
                OR, OR, OR, OR, W1, W1, W1, W1, W1, W1, OR, OR, OR, OR, OR, OR,
            ],
            vec![
                OR, OR, OR, OR, W1, W1, OR, OR, OR, OR, W1, OR, OR, OR, OR, OR,
            ],
            vec![
                OR, OR, OR, OR, W1, W1, OR, OR, OR, OR, W1, W1, OR, OR, OR, OR,
            ],
            vec![
                OR, OR, OR, OR, W1, W1, OR, OR, OR, OR, W1, W1, OR, OR, OR, OR,
            ],
            vec![
                B0, OR, OR, OR, W1, W1, OR, OR, OR, OR, W1, W1, OR, OR, OR, B0,
            ],
            vec![
                B0, OR, OR, OR, W1, W1, W1, W1, W1, W1, W1, W1, OR, OR, OR, B0,
            ],
            vec![
                B0, B0, OR, OR, OR, W1, W1, W1, W1, W1, W1, OR, OR, OR, B0, B0,
            ],
            vec![
                B0, B0, B0, OR, OR, OR, OR, OR, OR, OR, OR, OR, OR, B0, B0, B0,
            ],
            vec![
                B0, B0, B0, B0, B0, OR, OR, OR, OR, OR, OR, B0, B0, B0, B0, B0,
            ],
        ];
        pattern
    }

    fn c_train_bullet_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![
                B0, B0, B0, B0, B0, BL, BL, BL, BL, BL, BL, B0, B0, B0, B0, B0,
            ],
            vec![
                B0, B0, B0, BL, BL, BL, W1, W1, W1, W1, BL, BL, BL, B0, B0, B0,
            ],
            vec![
                B0, B0, BL, BL, BL, W1, W1, W1, W1, W1, W1, BL, BL, BL, B0, B0,
            ],
            vec![
                B0, BL, BL, BL, W1, W1, BL, BL, BL, BL, W1, W1, BL, BL, BL, B0,
            ],
            vec![
                B0, BL, BL, BL, W1, W1, BL, BL, BL, BL, W1, W1, BL, BL, BL, B0,
            ],
            vec![
                BL, BL, BL, W1, W1, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL,
            ],
            vec![
                BL, BL, BL, W1, W1, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL,
            ],
            vec![
                BL, BL, BL, W1, W1, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL,
            ],
            vec![
                BL, BL, BL, W1, W1, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL,
            ],
            vec![
                BL, BL, BL, W1, W1, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL,
            ],
            vec![
                BL, BL, BL, W1, W1, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL,
            ],
            vec![
                B0, BL, BL, BL, W1, W1, BL, BL, BL, BL, W1, W1, BL, BL, BL, B0,
            ],
            vec![
                B0, BL, BL, BL, W1, W1, BL, BL, BL, BL, W1, W1, BL, BL, BL, B0,
            ],
            vec![
                B0, B0, BL, BL, BL, W1, W1, W1, W1, W1, W1, BL, BL, BL, B0, B0,
            ],
            vec![
                B0, B0, B0, BL, BL, BL, W1, W1, W1, W1, BL, BL, BL, B0, B0, B0,
            ],
            vec![
                B0, B0, B0, B0, B0, BL, BL, BL, BL, BL, BL, B0, B0, B0, B0, B0,
            ],
        ];
        pattern
    }

    fn d_train_bullet_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![
                B0, B0, B0, B0, B0, OR, OR, OR, OR, OR, OR, B0, B0, B0, B0, B0,
            ],
            vec![
                B0, B0, B0, OR, OR, OR, OR, OR, OR, OR, OR, OR, OR, B0, B0, B0,
            ],
            vec![
                B0, B0, OR, OR, W1, W1, W1, W1, W1, W1, OR, OR, OR, OR, B0, B0,
            ],
            vec![
                B0, OR, OR, OR, W1, W1, W1, W1, W1, W1, W1, OR, OR, OR, OR, B0,
            ],
            vec![
                B0, OR, OR, OR, W1, W1, OR, OR, OR, OR, W1, W1, OR, OR, OR, B0,
            ],
            vec![
                OR, OR, OR, OR, W1, W1, OR, OR, OR, OR, W1, W1, OR, OR, OR, OR,
            ],
            vec![
                OR, OR, OR, OR, W1, W1, OR, OR, OR, OR, W1, W1, OR, OR, OR, OR,
            ],
            vec![
                OR, OR, OR, OR, W1, W1, OR, OR, OR, OR, W1, W1, OR, OR, OR, OR,
            ],
            vec![
                OR, OR, OR, OR, W1, W1, OR, OR, OR, OR, W1, W1, OR, OR, OR, OR,
            ],
            vec![
                OR, OR, OR, OR, W1, W1, OR, OR, OR, OR, W1, W1, OR, OR, OR, OR,
            ],
            vec![
                OR, OR, OR, OR, W1, W1, OR, OR, OR, OR, W1, W1, OR, OR, OR, OR,
            ],
            vec![
                B0, OR, OR, OR, W1, W1, OR, OR, OR, OR, W1, W1, OR, OR, OR, B0,
            ],
            vec![
                B0, OR, OR, OR, W1, W1, W1, W1, W1, W1, W1, OR, OR, OR, OR, B0,
            ],
            vec![
                B0, B0, OR, OR, OR, W1, W1, W1, W1, W1, OR, OR, OR, OR, B0, B0,
            ],
            vec![
                B0, B0, B0, OR, OR, OR, OR, OR, OR, OR, OR, OR, OR, B0, B0, B0,
            ],
            vec![
                B0, B0, B0, B0, B0, OR, OR, OR, OR, OR, OR, B0, B0, B0, B0, B0,
            ],
        ];
        pattern
    }

    fn e_train_bullet_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![
                B0, B0, B0, B0, B0, BL, BL, BL, BL, BL, BL, B0, B0, B0, B0, B0,
            ],
            vec![
                B0, B0, B0, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, B0, B0, B0,
            ],
            vec![
                B0, B0, BL, BL, W1, W1, W1, W1, W1, W1, W1, W1, BL, BL, B0, B0,
            ],
            vec![
                B0, BL, BL, BL, W1, W1, W1, W1, W1, W1, W1, W1, BL, BL, BL, B0,
            ],
            vec![
                B0, BL, BL, BL, W1, W1, BL, BL, BL, BL, BL, BL, BL, BL, BL, B0,
            ],
            vec![
                BL, BL, BL, BL, W1, W1, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL,
            ],
            vec![
                BL, BL, BL, BL, W1, W1, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL,
            ],
            vec![
                BL, BL, BL, BL, W1, W1, W1, W1, W1, W1, BL, BL, BL, BL, BL, BL,
            ],
            vec![
                BL, BL, BL, BL, W1, W1, W1, W1, W1, W1, BL, BL, BL, BL, BL, BL,
            ],
            vec![
                BL, BL, BL, BL, W1, W1, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL,
            ],
            vec![
                BL, BL, BL, BL, W1, W1, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL,
            ],
            vec![
                B0, BL, BL, BL, W1, W1, BL, BL, BL, BL, BL, BL, BL, BL, BL, B0,
            ],
            vec![
                B0, BL, BL, BL, W1, W1, W1, W1, W1, W1, W1, W1, BL, BL, BL, B0,
            ],
            vec![
                B0, B0, BL, BL, W1, W1, W1, W1, W1, W1, W1, W1, BL, BL, B0, B0,
            ],
            vec![
                B0, B0, B0, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, B0, B0, B0,
            ],
            vec![
                B0, B0, B0, B0, B0, BL, BL, BL, BL, BL, BL, B0, B0, B0, B0, B0,
            ],
        ];
        pattern
    }

    fn f_train_bullet_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![
                B0, B0, B0, B0, B0, OR, OR, OR, OR, OR, OR, B0, B0, B0, B0, B0,
            ],
            vec![
                B0, B0, B0, OR, OR, OR, OR, OR, OR, OR, OR, OR, OR, B0, B0, B0,
            ],
            vec![
                B0, B0, OR, OR, W1, W1, W1, W1, W1, W1, W1, W1, OR, OR, B0, B0,
            ],
            vec![
                B0, OR, OR, OR, W1, W1, W1, W1, W1, W1, W1, W1, OR, OR, OR, B0,
            ],
            vec![
                B0, OR, OR, OR, W1, W1, OR, OR, OR, OR, OR, OR, OR, OR, OR, B0,
            ],
            vec![
                OR, OR, OR, OR, W1, W1, OR, OR, OR, OR, OR, OR, OR, OR, OR, OR,
            ],
            vec![
                OR, OR, OR, OR, W1, W1, OR, OR, OR, OR, OR, OR, OR, OR, OR, OR,
            ],
            vec![
                OR, OR, OR, OR, W1, W1, W1, W1, W1, W1, OR, OR, OR, OR, OR, OR,
            ],
            vec![
                OR, OR, OR, OR, W1, W1, W1, W1, W1, W1, OR, OR, OR, OR, OR, OR,
            ],
            vec![
                OR, OR, OR, OR, W1, W1, OR, OR, OR, OR, OR, OR, OR, OR, OR, OR,
            ],
            vec![
                OR, OR, OR, OR, W1, W1, OR, OR, OR, OR, OR, OR, OR, OR, OR, OR,
            ],
            vec![
                B0, OR, OR, OR, W1, W1, OR, OR, OR, OR, OR, OR, OR, OR, OR, B0,
            ],
            vec![
                B0, OR, OR, OR, W1, W1, OR, OR, OR, OR, OR, OR, OR, OR, OR, B0,
            ],
            vec![
                B0, B0, OR, OR, W1, W1, OR, OR, OR, OR, OR, OR, OR, OR, B0, B0,
            ],
            vec![
                B0, B0, B0, OR, OR, OR, OR, OR, OR, OR, OR, OR, OR, B0, B0, B0,
            ],
            vec![
                B0, B0, B0, B0, B0, OR, OR, OR, OR, OR, OR, B0, B0, B0, B0, B0,
            ],
        ];
        pattern
    }

    fn g_train_bullet_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![
                B0, B0, B0, B0, B0, LG, LG, LG, LG, LG, LG, B0, B0, B0, B0, B0,
            ],
            vec![
                B0, B0, B0, LG, LG, LG, W1, W1, W1, W1, LG, LG, LG, B0, B0, B0,
            ],
            vec![
                B0, B0, LG, LG, LG, W1, W1, W1, W1, W1, W1, LG, LG, LG, B0, B0,
            ],
            vec![
                B0, LG, LG, LG, W1, W1, LG, LG, LG, LG, W1, W1, LG, LG, LG, B0,
            ],
            vec![
                B0, LG, LG, LG, W1, W1, LG, LG, LG, LG, W1, W1, LG, LG, LG, B0,
            ],
            vec![
                LG, LG, LG, W1, W1, LG, LG, LG, LG, LG, LG, LG, LG, LG, LG, LG,
            ],
            vec![
                LG, LG, LG, W1, W1, LG, LG, LG, LG, LG, LG, LG, LG, LG, LG, LG,
            ],
            vec![
                LG, LG, LG, W1, W1, LG, LG, LG, LG, LG, LG, LG, LG, LG, LG, LG,
            ],
            vec![
                LG, LG, LG, W1, W1, LG, LG, LG, LG, W1, W1, W1, LG, LG, LG, LG,
            ],
            vec![
                LG, LG, LG, W1, W1, LG, LG, LG, LG, W1, W1, W1, LG, LG, LG, LG,
            ],
            vec![
                LG, LG, LG, W1, W1, LG, LG, LG, LG, LG, W1, W1, LG, LG, LG, LG,
            ],
            vec![
                B0, LG, LG, LG, W1, W1, LG, LG, LG, LG, W1, W1, LG, LG, LG, B0,
            ],
            vec![
                B0, LG, LG, LG, W1, W1, LG, LG, LG, LG, W1, W1, LG, LG, LG, B0,
            ],
            vec![
                B0, B0, LG, LG, LG, W1, W1, W1, W1, W1, W1, LG, LG, LG, B0, B0,
            ],
            vec![
                B0, B0, B0, LG, LG, LG, W1, W1, W1, W1, LG, LG, LG, B0, B0, B0,
            ],
            vec![
                B0, B0, B0, B0, B0, LG, LG, LG, LG, LG, LG, B0, B0, B0, B0, B0,
            ],
        ];
        pattern
    }

    fn j_train_bullet_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![
                B0, B0, B0, B0, B0, BR, BR, BR, BR, BR, BR, B0, B0, B0, B0, B0,
            ],
            vec![
                B0, B0, B0, BR, BR, BR, BR, BR, BR, BR, BR, BR, BR, B0, B0, B0,
            ],
            vec![
                B0, B0, BR, BR, BR, BR, BR, BR, BR, W1, W1, BR, BR, BR, B0, B0,
            ],
            vec![
                B0, BR, BR, BR, BR, BR, BR, BR, BR, W1, W1, BR, BR, BR, BR, B0,
            ],
            vec![
                B0, BR, BR, BR, BR, BR, BR, BR, BR, W1, W1, BR, BR, BR, BR, B0,
            ],
            vec![
                BR, BR, BR, BR, BR, BR, BR, BR, BR, W1, W1, BR, BR, BR, BR, BR,
            ],
            vec![
                BR, BR, BR, BR, BR, BR, BR, BR, BR, W1, W1, BR, BR, BR, BR, BR,
            ],
            vec![
                BR, BR, BR, BR, BR, BR, BR, BR, BR, W1, W1, BR, BR, BR, BR, BR,
            ],
            vec![
                BR, BR, BR, BR, BR, BR, BR, BR, BR, W1, W1, BR, BR, BR, BR, BR,
            ],
            vec![
                BR, BR, BR, BR, BR, BR, BR, BR, BR, W1, W1, BR, BR, BR, BR, BR,
            ],
            vec![
                BR, BR, BR, BR, BR, BR, BR, BR, BR, W1, W1, BR, BR, BR, BR, BR,
            ],
            vec![
                B0, BR, BR, BR, W1, W1, BR, BR, BR, W1, W1, BR, BR, BR, BR, B0,
            ],
            vec![
                B0, BR, BR, BR, W1, W1, W1, W1, W1, W1, BR, BR, BR, BR, BR, B0,
            ],
            vec![
                B0, B0, BR, BR, BR, W1, W1, W1, W1, BR, BR, BR, BR, BR, B0, B0,
            ],
            vec![
                B0, B0, B0, BR, BR, BR, BR, BR, BR, BR, BR, BR, BR, B0, B0, B0,
            ],
            vec![
                B0, B0, B0, B0, B0, BR, BR, BR, BR, BR, BR, B0, B0, B0, B0, B0,
            ],
        ];
        pattern
    }

    fn l_train_bullet_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![
                B0, B0, B0, B0, B0, GY, GY, GY, GY, GY, GY, B0, B0, B0, B0, B0,
            ],
            vec![
                B0, B0, B0, GY, GY, GY, GY, GY, GY, GY, GY, GY, GY, B0, B0, B0,
            ],
            vec![
                B0, B0, GY, GY, W1, W1, GY, GY, GY, GY, GY, GY, GY, GY, B0, B0,
            ],
            vec![
                B0, GY, GY, GY, W1, W1, GY, GY, GY, GY, GY, GY, GY, GY, GY, B0,
            ],
            vec![
                B0, GY, GY, GY, W1, W1, GY, GY, GY, GY, GY, GY, GY, GY, GY, B0,
            ],
            vec![
                GY, GY, GY, GY, W1, W1, GY, GY, GY, GY, GY, GY, GY, GY, GY, GY,
            ],
            vec![
                GY, GY, GY, GY, W1, W1, GY, GY, GY, GY, GY, GY, GY, GY, GY, GY,
            ],
            vec![
                GY, GY, GY, GY, W1, W1, GY, GY, GY, GY, GY, GY, GY, GY, GY, GY,
            ],
            vec![
                GY, GY, GY, GY, W1, W1, GY, GY, GY, GY, GY, GY, GY, GY, GY, GY,
            ],
            vec![
                GY, GY, GY, GY, W1, W1, GY, GY, GY, GY, GY, GY, GY, GY, GY, GY,
            ],
            vec![
                GY, GY, GY, GY, W1, W1, GY, GY, GY, GY, GY, GY, GY, GY, GY, GY,
            ],
            vec![
                B0, GY, GY, GY, W1, W1, GY, GY, GY, GY, GY, GY, GY, GY, GY, B0,
            ],
            vec![
                B0, GY, GY, GY, W1, W1, W1, W1, W1, W1, W1, W1, GY, GY, GY, B0,
            ],
            vec![
                B0, B0, GY, GY, W1, W1, W1, W1, W1, W1, W1, W1, GY, GY, B0, B0,
            ],
            vec![
                B0, B0, B0, GY, GY, GY, GY, GY, GY, GY, GY, GY, GY, B0, B0, B0,
            ],
            vec![
                B0, B0, B0, B0, B0, GY, GY, GY, GY, GY, GY, B0, B0, B0, B0, B0,
            ],
        ];
        pattern
    }

    fn m_train_bullet_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![
                B0, B0, B0, B0, B0, OR, OR, OR, OR, OR, OR, B0, B0, B0, B0, B0,
            ],
            vec![
                B0, B0, B0, OR, OR, OR, OR, OR, OR, OR, OR, OR, OR, B0, B0, B0,
            ],
            vec![
                B0, B0, OR, OR, W1, W1, OR, OR, OR, OR, W1, W1, OR, OR, B0, B0,
            ],
            vec![
                B0, OR, OR, OR, W1, W1, OR, OR, OR, OR, W1, W1, OR, OR, OR, B0,
            ],
            vec![
                B0, OR, OR, OR, W1, W1, W1, OR, OR, W1, W1, W1, OR, OR, OR, B0,
            ],
            vec![
                OR, OR, OR, OR, W1, W1, W1, OR, OR, W1, W1, W1, OR, OR, OR, OR,
            ],
            vec![
                OR, OR, OR, OR, W1, W1, W1, W1, W1, W1, W1, W1, OR, OR, OR, OR,
            ],
            vec![
                OR, OR, OR, OR, W1, W1, OR, W1, W1, OR, W1, W1, OR, OR, OR, OR,
            ],
            vec![
                OR, OR, OR, OR, W1, W1, OR, W1, W1, OR, W1, W1, OR, OR, OR, OR,
            ],
            vec![
                OR, OR, OR, OR, W1, W1, OR, OR, OR, OR, W1, W1, OR, OR, OR, OR,
            ],
            vec![
                OR, OR, OR, OR, W1, W1, OR, OR, OR, OR, W1, W1, OR, OR, OR, OR,
            ],
            vec![
                B0, OR, OR, OR, W1, W1, OR, OR, OR, OR, W1, W1, OR, OR, OR, B0,
            ],
            vec![
                B0, OR, OR, OR, W1, W1, OR, OR, OR, OR, W1, W1, OR, OR, OR, B0,
            ],
            vec![
                B0, B0, OR, OR, W1, W1, OR, OR, OR, OR, W1, W1, OR, OR, B0, B0,
            ],
            vec![
                B0, B0, B0, OR, OR, OR, OR, OR, OR, OR, OR, OR, OR, B0, B0, B0,
            ],
            vec![
                B0, B0, B0, B0, B0, OR, OR, OR, OR, OR, OR, B0, B0, B0, B0, B0,
            ],
        ];
        pattern
    }

    fn n_train_bullet_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![
                B0, B0, B0, B0, B0, YE, YE, YE, YE, YE, YE, B0, B0, B0, B0, B0,
            ],
            vec![
                B0, B0, B0, YE, YE, YE, YE, YE, YE, YE, YE, YE, YE, B0, B0, B0,
            ],
            vec![
                B0, B0, YE, YE, W1, W1, YE, YE, YE, YE, W1, W1, YE, YE, B0, B0,
            ],
            vec![
                B0, YE, YE, YE, W1, W1, YE, YE, YE, YE, W1, W1, YE, YE, YE, B0,
            ],
            vec![
                B0, YE, YE, YE, W1, W1, W1, YE, YE, YE, W1, W1, YE, YE, YE, B0,
            ],
            vec![
                YE, YE, YE, YE, W1, W1, W1, YE, YE, YE, W1, W1, YE, YE, YE, YE,
            ],
            vec![
                YE, YE, YE, YE, W1, W1, W1, W1, YE, YE, W1, W1, YE, YE, YE, YE,
            ],
            vec![
                YE, YE, YE, YE, W1, W1, YE, W1, W1, YE, W1, W1, YE, YE, YE, YE,
            ],
            vec![
                YE, YE, YE, YE, W1, W1, YE, W1, W1, YE, W1, W1, YE, YE, YE, YE,
            ],
            vec![
                YE, YE, YE, YE, W1, W1, YE, YE, W1, W1, W1, W1, YE, YE, YE, YE,
            ],
            vec![
                YE, YE, YE, YE, W1, W1, YE, YE, YE, W1, W1, W1, YE, YE, YE, YE,
            ],
            vec![
                B0, YE, YE, YE, W1, W1, YE, YE, YE, W1, W1, W1, YE, YE, YE, B0,
            ],
            vec![
                B0, YE, YE, YE, W1, W1, YE, YE, YE, YE, W1, W1, YE, YE, YE, B0,
            ],
            vec![
                B0, B0, YE, YE, W1, W1, YE, YE, YE, YE, W1, W1, YE, YE, B0, B0,
            ],
            vec![
                B0, B0, B0, YE, YE, YE, YE, YE, YE, YE, YE, YE, YE, B0, B0, B0,
            ],
            vec![
                B0, B0, B0, B0, B0, YE, YE, YE, YE, YE, YE, B0, B0, B0, B0, B0,
            ],
        ];
        pattern
    }

    fn q_train_bullet_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![
                B0, B0, B0, B0, B0, YE, YE, YE, YE, YE, YE, B0, B0, B0, B0, B0,
            ],
            vec![
                B0, B0, B0, YE, YE, YE, W1, W1, W1, W1, YE, YE, YE, B0, B0, B0,
            ],
            vec![
                B0, B0, YE, YE, YE, W1, W1, W1, W1, W1, W1, YE, YE, YE, B0, B0,
            ],
            vec![
                B0, YE, YE, YE, W1, W1, YE, YE, YE, YE, W1, W1, YE, YE, YE, B0,
            ],
            vec![
                B0, YE, YE, YE, W1, W1, YE, YE, YE, YE, W1, W1, YE, YE, YE, B0,
            ],
            vec![
                YE, YE, YE, W1, W1, YE, YE, YE, YE, YE, YE, W1, W1, YE, YE, YE,
            ],
            vec![
                YE, YE, YE, W1, W1, YE, YE, YE, YE, YE, YE, W1, W1, YE, YE, YE,
            ],
            vec![
                YE, YE, YE, W1, W1, YE, YE, YE, YE, YE, YE, W1, W1, YE, YE, YE,
            ],
            vec![
                YE, YE, YE, W1, W1, YE, YE, YE, YE, YE, YE, W1, W1, YE, YE, YE,
            ],
            vec![
                YE, YE, YE, W1, W1, YE, YE, YE, YE, YE, YE, W1, W1, YE, YE, YE,
            ],
            vec![
                YE, YE, YE, YE, W1, W1, YE, YE, YE, YE, W1, W1, YE, YE, YE, YE,
            ],
            vec![
                B0, YE, YE, YE, W1, W1, YE, YE, YE, YE, W1, W1, YE, YE, YE, B0,
            ],
            vec![
                B0, YE, YE, YE, YE, W1, W1, W1, W1, W1, W1, YE, YE, YE, YE, B0,
            ],
            vec![
                B0, B0, YE, YE, YE, YE, YE, YE, W1, W1, YE, YE, YE, YE, B0, B0,
            ],
            vec![
                B0, B0, B0, YE, YE, YE, YE, YE, W1, W1, W1, W1, YE, B0, B0, B0,
            ],
            vec![
                B0, B0, B0, B0, B0, YE, YE, YE, YE, YE, YE, B0, B0, B0, B0, B0,
            ],
        ];
        pattern
    }

    fn r_train_bullet_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![
                B0, B0, B0, B0, B0, YE, YE, YE, YE, YE, YE, B0, B0, B0, B0, B0,
            ],
            vec![
                B0, B0, B0, YE, YE, YE, YE, YE, YE, YE, YE, YE, YE, B0, B0, B0,
            ],
            vec![
                B0, B0, YE, YE, W1, W1, W1, W1, W1, W1, YE, YE, YE, YE, B0, B0,
            ],
            vec![
                B0, YE, YE, YE, W1, W1, W1, W1, W1, W1, W1, YE, YE, YE, YE, B0,
            ],
            vec![
                B0, YE, YE, YE, W1, W1, YE, YE, YE, YE, W1, W1, YE, YE, YE, B0,
            ],
            vec![
                YE, YE, YE, YE, W1, W1, YE, YE, YE, YE, W1, W1, YE, YE, YE, YE,
            ],
            vec![
                YE, YE, YE, YE, W1, W1, YE, YE, YE, YE, W1, W1, YE, YE, YE, YE,
            ],
            vec![
                YE, YE, YE, YE, W1, W1, W1, W1, W1, W1, W1, YE, YE, YE, YE, YE,
            ],
            vec![
                YE, YE, YE, YE, W1, W1, W1, W1, W1, W1, YE, YE, YE, YE, YE, YE,
            ],
            vec![
                YE, YE, YE, YE, W1, W1, YE, YE, W1, W1, YE, YE, YE, YE, YE, YE,
            ],
            vec![
                YE, YE, YE, YE, W1, W1, YE, YE, YE, W1, W1, YE, YE, YE, YE, YE,
            ],
            vec![
                B0, YE, YE, YE, W1, W1, YE, YE, YE, W1, W1, YE, YE, YE, YE, B0,
            ],
            vec![
                B0, YE, YE, YE, W1, W1, YE, YE, YE, YE, W1, W1, YE, YE, YE, B0,
            ],
            vec![
                B0, B0, YE, YE, W1, W1, YE, YE, YE, YE, W1, W1, YE, YE, B0, B0,
            ],
            vec![
                B0, B0, B0, YE, YE, YE, YE, YE, YE, YE, YE, YE, YE, B0, B0, B0,
            ],
            vec![
                B0, B0, B0, B0, B0, YE, YE, YE, YE, YE, YE, B0, B0, B0, B0, B0,
            ],
        ];
        pattern
    }

    fn s_train_bullet_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![
                B0, B0, B0, B0, B0, GY, GY, GY, GY, GY, GY, B0, B0, B0, B0, B0,
            ],
            vec![
                B0, B0, B0, GY, GY, GY, W1, W1, W1, W1, GY, GY, GY, B0, B0, B0,
            ],
            vec![
                B0, B0, GY, GY, GY, W1, W1, W1, W1, W1, W1, GY, GY, GY, B0, B0,
            ],
            vec![
                B0, GY, GY, GY, W1, W1, GY, GY, GY, GY, W1, W1, GY, GY, GY, B0,
            ],
            vec![
                B0, GY, GY, GY, W1, W1, GY, GY, GY, GY, W1, W1, GY, GY, GY, B0,
            ],
            vec![
                GY, GY, GY, GY, W1, W1, GY, GY, GY, GY, GY, GY, GY, GY, GY, GY,
            ],
            vec![
                GY, GY, GY, GY, W1, W1, GY, GY, GY, GY, GY, GY, GY, GY, GY, GY,
            ],
            vec![
                GY, GY, GY, GY, GY, W1, W1, W1, W1, W1, GY, GY, GY, GY, GY, GY,
            ],
            vec![
                GY, GY, GY, GY, GY, GY, W1, W1, W1, W1, W1, GY, GY, GY, GY, GY,
            ],
            vec![
                GY, GY, GY, GY, GY, GY, GY, GY, GY, GY, W1, W1, GY, GY, GY, GY,
            ],
            vec![
                GY, GY, GY, GY, GY, GY, GY, GY, GY, GY, W1, W1, GY, GY, GY, GY,
            ],
            vec![
                B0, GY, GY, GY, W1, W1, GY, GY, GY, GY, W1, W1, GY, GY, GY, B0,
            ],
            vec![
                B0, GY, GY, GY, W1, W1, GY, GY, GY, GY, W1, W1, GY, GY, GY, B0,
            ],
            vec![
                B0, B0, GY, GY, GY, W1, W1, W1, W1, W1, W1, GY, GY, GY, B0, B0,
            ],
            vec![
                B0, B0, B0, GY, GY, GY, W1, W1, W1, W1, GY, GY, GY, B0, B0, B0,
            ],
            vec![
                B0, B0, B0, B0, B0, GY, GY, GY, GY, GY, GY, B0, B0, B0, B0, B0,
            ],
        ];
        pattern
    }

    fn w_train_bullet_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![
                B0, B0, B0, B0, B0, YE, YE, YE, YE, YE, YE, B0, B0, B0, B0, B0,
            ],
            vec![
                B0, B0, B0, YE, YE, YE, YE, YE, YE, YE, YE, YE, YE, B0, B0, B0,
            ],
            vec![
                B0, B0, YE, W1, W1, YE, YE, YE, YE, YE, YE, W1, W1, YE, B0, B0,
            ],
            vec![
                B0, YE, YE, W1, W1, YE, YE, YE, YE, YE, YE, W1, W1, YE, YE, B0,
            ],
            vec![
                B0, YE, YE, W1, W1, YE, YE, YE, YE, YE, YE, W1, W1, YE, YE, B0,
            ],
            vec![
                YE, YE, YE, W1, W1, YE, YE, YE, YE, YE, YE, W1, W1, YE, YE, YE,
            ],
            vec![
                YE, YE, YE, YE, W1, W1, YE, YE, YE, YE, W1, W1, YE, YE, YE, YE,
            ],
            vec![
                YE, YE, YE, YE, W1, W1, YE, YE, YE, YE, W1, W1, YE, YE, YE, YE,
            ],
            vec![
                YE, YE, YE, YE, W1, W1, YE, W1, W1, YE, W1, W1, YE, YE, YE, YE,
            ],
            vec![
                YE, YE, YE, YE, W1, W1, YE, W1, W1, YE, W1, W1, YE, YE, YE, YE,
            ],
            vec![
                YE, YE, YE, YE, YE, W1, W1, W1, W1, W1, W1, YE, YE, YE, YE, YE,
            ],
            vec![
                B0, YE, YE, YE, YE, W1, W1, YE, YE, W1, W1, YE, YE, YE, YE, B0,
            ],
            vec![
                B0, YE, YE, YE, YE, W1, W1, YE, YE, W1, W1, YE, YE, YE, YE, B0,
            ],
            vec![
                B0, B0, YE, YE, YE, W1, W1, YE, YE, W1, W1, YE, YE, YE, B0, B0,
            ],
            vec![
                B0, B0, B0, YE, YE, YE, YE, YE, YE, YE, YE, YE, YE, B0, B0, B0,
            ],
            vec![
                B0, B0, B0, B0, B0, YE, YE, YE, YE, YE, YE, B0, B0, B0, B0, B0,
            ],
        ];
        pattern
    }

    fn z_train_bullet_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![
                B0, B0, B0, B0, B0, BR, BR, BR, BR, BR, BR, B0, B0, B0, B0, B0,
            ],
            vec![
                B0, B0, B0, BR, BR, BR, BR, BR, BR, BR, BR, BR, BR, B0, B0, B0,
            ],
            vec![
                B0, B0, BR, BR, W1, W1, W1, W1, W1, W1, W1, W1, BR, BR, B0, B0,
            ],
            vec![
                B0, BR, BR, BR, W1, W1, W1, W1, W1, W1, W1, W1, BR, BR, BR, B0,
            ],
            vec![
                B0, BR, BR, BR, BR, BR, BR, BR, BR, BR, W1, W1, BR, BR, BR, B0,
            ],
            vec![
                BR, BR, BR, BR, BR, BR, BR, BR, BR, W1, W1, BR, BR, BR, BR, BR,
            ],
            vec![
                BR, BR, BR, BR, BR, BR, BR, BR, W1, W1, BR, BR, BR, BR, BR, BR,
            ],
            vec![
                BR, BR, BR, BR, BR, BR, BR, W1, W1, BR, BR, BR, BR, BR, BR, BR,
            ],
            vec![
                BR, BR, BR, BR, BR, BR, W1, W1, BR, BR, BR, BR, BR, BR, BR, BR,
            ],
            vec![
                BR, BR, BR, BR, BR, W1, W1, BR, BR, BR, BR, BR, BR, BR, BR, BR,
            ],
            vec![
                BR, BR, BR, BR, W1, W1, BR, BR, BR, BR, BR, BR, BR, BR, BR, BR,
            ],
            vec![
                B0, BR, BR, BR, W1, W1, W1, W1, W1, W1, W1, W1, BR, BR, BR, B0,
            ],
            vec![
                B0, BR, BR, BR, W1, W1, W1, W1, W1, W1, W1, W1, BR, BR, BR, B0,
            ],
            vec![
                B0, B0, BR, BR, BR, BR, BR, BR, BR, BR, BR, BR, BR, BR, B0, B0,
            ],
            vec![
                B0, B0, B0, BR, BR, BR, BR, BR, BR, BR, BR, BR, BR, B0, B0, B0,
            ],
            vec![
                B0, B0, B0, B0, B0, BR, BR, BR, BR, BR, BR, B0, B0, B0, B0, B0,
            ],
        ];
        pattern
    }

    // Patterns for letters A-Z and space
    // TODO check if they actually match MTA display fonts

    pub fn pattern_for_letter(letter: char) -> BulbDisplay {
        match letter {
            ' ' => space_pattern(),
            ':' => colon_pattern(),
            'A' => letter_a_pattern(),
            'B' => letter_b_pattern(),
            'C' => letter_c_pattern(),
            'D' => letter_d_pattern(),
            'E' => letter_e_pattern(),
            'F' => letter_f_pattern(),
            'G' => letter_g_pattern(),
            'H' => letter_h_pattern(),
            'I' => letter_i_pattern(),
            'J' => letter_j_pattern(),
            'K' => letter_k_pattern(),
            'L' => letter_l_pattern(),
            'M' => letter_m_pattern(),
            'N' => letter_n_pattern(),
            'O' => letter_o_pattern(),
            'P' => letter_p_pattern(),
            'Q' => letter_q_pattern(),
            'R' => letter_r_pattern(),
            'S' => letter_s_pattern(),
            'T' => letter_t_pattern(),
            'U' => letter_u_pattern(),
            'V' => letter_v_pattern(),
            'W' => letter_w_pattern(),
            'X' => letter_x_pattern(),
            'Y' => letter_y_pattern(),
            'Z' => letter_z_pattern(),
            '1' => number_one_pattern(),
            '2' => number_two_pattern(),
            '3' => number_three_pattern(),
            '4' => number_four_pattern(),
            '5' => number_five_pattern(),
            '6' => number_six_pattern(),
            '7' => number_seven_pattern(),
            '8' => number_eight_pattern(),
            '9' => number_nine_pattern(),
            '0' => number_zero_pattern(),
            _ => space_pattern(),
        }
    }

    fn space_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![B0, B0, B0, B0, B0, B0, B0, B0],
            vec![B0, B0, B0, B0, B0, B0, B0, B0],
            vec![B0, B0, B0, B0, B0, B0, B0, B0],
            vec![B0, B0, B0, B0, B0, B0, B0, B0],
            vec![B0, B0, B0, B0, B0, B0, B0, B0],
            vec![B0, B0, B0, B0, B0, B0, B0, B0],
            vec![B0, B0, B0, B0, B0, B0, B0, B0],
            vec![B0, B0, B0, B0, B0, B0, B0, B0],
            vec![B0, B0, B0, B0, B0, B0, B0, B0],
            vec![B0, B0, B0, B0, B0, B0, B0, B0],
            vec![B0, B0, B0, B0, B0, B0, B0, B0],
            vec![B0, B0, B0, B0, B0, B0, B0, B0],
            vec![B0, B0, B0, B0, B0, B0, B0, B0],
            vec![B0, B0, B0, B0, B0, B0, B0, B0],
            vec![B0, B0, B0, B0, B0, B0, B0, B0],
            vec![B0, B0, B0, B0, B0, B0, B0, B0],
        ];
        pattern
    }

    fn letter_a_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![B0, B0, W1, W1, W1, W1, B0, B0],
            vec![B0, W1, W1, W1, W1, W1, W1, B0],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
        ];
        pattern
    }

    fn letter_b_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![W1, W1, W1, W1, W1, W1, B0, B0],
            vec![W1, W1, W1, W1, W1, W1, W1, B0],
            vec![W1, W1, B0, B0, B0, W1, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, W1, W1, W1],
            vec![W1, W1, W1, W1, W1, W1, W1, B0],
            vec![W1, W1, W1, W1, W1, W1, B0, B0],
            vec![W1, W1, W1, W1, W1, W1, W1, B0],
            vec![W1, W1, B0, B0, B0, W1, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, W1, W1, W1],
            vec![W1, W1, W1, W1, W1, W1, W1, B0],
            vec![W1, W1, W1, W1, W1, W1, B0, B0],
        ];
        pattern
    }

    fn letter_c_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![B0, W1, W1, W1, W1, W1, W1, B0],
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![B0, W1, W1, W1, W1, W1, W1, B0],
        ];
        pattern
    }

    fn letter_d_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![W1, W1, W1, W1, W1, W1, B0, B0],
            vec![W1, W1, W1, W1, W1, W1, W1, B0],
            vec![W1, W1, B0, B0, B0, W1, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, W1, W1, W1],
            vec![W1, W1, W1, W1, W1, W1, W1, B0],
            vec![W1, W1, W1, W1, W1, W1, B0, B0],
        ];
        pattern
    }

    fn letter_e_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, W1, W1, W1, W1, W1, B0],
            vec![W1, W1, W1, W1, W1, W1, W1, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
        ];
        pattern
    }

    fn letter_f_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, W1, W1, W1, W1, W1, B0],
            vec![W1, W1, W1, W1, W1, W1, W1, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
        ];
        pattern
    }

    fn letter_g_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![B0, W1, W1, W1, W1, W1, W1, B0],
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, W1, W1, W1, W1],
            vec![W1, W1, B0, B0, W1, W1, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![B0, W1, W1, W1, W1, W1, W1, B0],
        ];
        pattern
    }

    fn letter_h_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
        ];
        pattern
    }

    fn letter_i_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![B0, B0, B0, W1, W1, B0, B0, B0],
            vec![B0, B0, B0, W1, W1, B0, B0, B0],
            vec![B0, B0, B0, W1, W1, B0, B0, B0],
            vec![B0, B0, B0, W1, W1, B0, B0, B0],
            vec![B0, B0, B0, W1, W1, B0, B0, B0],
            vec![B0, B0, B0, W1, W1, B0, B0, B0],
            vec![B0, B0, B0, W1, W1, B0, B0, B0],
            vec![B0, B0, B0, W1, W1, B0, B0, B0],
            vec![B0, B0, B0, W1, W1, B0, B0, B0],
            vec![B0, B0, B0, W1, W1, B0, B0, B0],
            vec![B0, B0, B0, W1, W1, B0, B0, B0],
            vec![B0, B0, B0, W1, W1, B0, B0, B0],
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
        ];
        pattern
    }

    fn letter_j_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![B0, B0, B0, B0, W1, W1, W1, W1],
            vec![B0, B0, B0, B0, W1, W1, W1, W1],
            vec![B0, B0, B0, B0, B0, B0, W1, W1],
            vec![B0, B0, B0, B0, B0, B0, W1, W1],
            vec![B0, B0, B0, B0, B0, B0, W1, W1],
            vec![B0, B0, B0, B0, B0, B0, W1, W1],
            vec![B0, B0, B0, B0, B0, B0, W1, W1],
            vec![B0, B0, B0, B0, B0, B0, W1, W1],
            vec![B0, B0, B0, B0, B0, B0, W1, W1],
            vec![B0, B0, B0, B0, B0, B0, W1, W1],
            vec![B0, B0, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![B0, W1, W1, W1, W1, W1, W1, B0],
        ];
        pattern
    }

    fn letter_k_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, W1, W1, W1],
            vec![W1, W1, B0, B0, W1, W1, W1, B0],
            vec![W1, W1, B0, W1, W1, W1, B0, B0],
            vec![W1, W1, W1, W1, W1, B0, B0, B0],
            vec![W1, W1, W1, W1, B0, B0, B0, B0],
            vec![W1, W1, W1, W1, B0, B0, B0, B0],
            vec![W1, W1, W1, W1, W1, B0, B0, B0],
            vec![W1, W1, W1, W1, W1, B0, B0, B0],
            vec![W1, W1, W1, W1, W1, W1, B0, B0],
            vec![W1, W1, B0, W1, W1, W1, B0, B0],
            vec![W1, W1, B0, W1, W1, W1, W1, B0],
            vec![W1, W1, B0, B0, W1, W1, W1, B0],
            vec![W1, W1, B0, B0, W1, W1, W1, W1],
            vec![W1, W1, B0, B0, B0, W1, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
        ];
        pattern
    }

    fn letter_l_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
        ];
        pattern
    }

    fn letter_m_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, W1, B0, B0, W1, W1, W1],
            vec![W1, W1, W1, B0, B0, W1, W1, W1],
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![W1, W1, B0, W1, W1, B0, W1, W1],
            vec![W1, W1, B0, W1, W1, B0, W1, W1],
            vec![W1, W1, B0, W1, W1, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
        ];
        pattern
    }

    fn letter_n_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, W1, B0, B0, B0, W1, W1],
            vec![W1, W1, W1, B0, B0, B0, W1, W1],
            vec![W1, W1, W1, W1, B0, B0, W1, W1],
            vec![W1, W1, W1, W1, B0, B0, W1, W1],
            vec![W1, W1, W1, W1, B0, B0, W1, W1],
            vec![W1, W1, B0, W1, W1, B0, W1, W1],
            vec![W1, W1, B0, W1, W1, B0, W1, W1],
            vec![W1, W1, B0, W1, W1, W1, W1, W1],
            vec![W1, W1, B0, B0, W1, W1, W1, W1],
            vec![W1, W1, B0, B0, W1, W1, W1, W1],
            vec![W1, W1, B0, B0, W1, W1, W1, W1],
            vec![W1, W1, B0, B0, B0, W1, W1, W1],
            vec![W1, W1, B0, B0, B0, W1, W1, W1],
            vec![W1, W1, B0, B0, B0, W1, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
        ];
        pattern
    }

    fn letter_o_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![B0, W1, W1, W1, W1, W1, W1, B0],
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![B0, W1, W1, W1, W1, W1, W1, B0],
        ];
        pattern
    }

    fn letter_p_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![W1, W1, W1, W1, W1, W1, B0, B0],
            vec![W1, W1, W1, W1, W1, W1, W1, B0],
            vec![W1, W1, B0, B0, B0, W1, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, W1, W1, W1],
            vec![W1, W1, W1, W1, W1, W1, W1, B0],
            vec![W1, W1, W1, W1, W1, W1, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
        ];
        pattern
    }

    fn letter_q_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![B0, W1, W1, W1, W1, W1, W1, B0],
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, W1, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, W1, W1, W1],
            vec![W1, W1, B0, B0, B0, W1, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![B0, W1, W1, W1, W1, W1, W1, W1],
        ];
        pattern
    }

    fn letter_r_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![W1, W1, W1, W1, W1, W1, B0, B0],
            vec![W1, W1, W1, W1, W1, W1, W1, B0],
            vec![W1, W1, B0, B0, B0, W1, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, W1, W1, W1],
            vec![W1, W1, W1, W1, W1, W1, W1, B0],
            vec![W1, W1, W1, W1, W1, W1, B0, B0],
            vec![W1, W1, B0, W1, W1, W1, B0, B0],
            vec![W1, W1, B0, B0, W1, W1, B0, B0],
            vec![W1, W1, B0, B0, W1, W1, W1, B0],
            vec![W1, W1, B0, B0, B0, W1, W1, B0],
            vec![W1, W1, B0, B0, B0, W1, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
        ];
        pattern
    }

    fn letter_s_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![B0, W1, W1, W1, W1, W1, W1, B0],
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, W1, B0, B0, B0, B0, B0],
            vec![W1, W1, W1, W1, W1, W1, B0, B0],
            vec![B0, W1, W1, W1, W1, W1, W1, B0],
            vec![B0, B0, W1, W1, W1, W1, W1, W1],
            vec![B0, B0, B0, B0, B0, W1, W1, W1],
            vec![B0, B0, B0, B0, B0, B0, W1, W1],
            vec![B0, B0, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![B0, W1, W1, W1, W1, W1, W1, B0],
        ];
        pattern
    }

    fn letter_t_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![B0, B0, B0, W1, W1, B0, B0, B0],
            vec![B0, B0, B0, W1, W1, B0, B0, B0],
            vec![B0, B0, B0, W1, W1, B0, B0, B0],
            vec![B0, B0, B0, W1, W1, B0, B0, B0],
            vec![B0, B0, B0, W1, W1, B0, B0, B0],
            vec![B0, B0, B0, W1, W1, B0, B0, B0],
            vec![B0, B0, B0, W1, W1, B0, B0, B0],
            vec![B0, B0, B0, W1, W1, B0, B0, B0],
            vec![B0, B0, B0, W1, W1, B0, B0, B0],
            vec![B0, B0, B0, W1, W1, B0, B0, B0],
            vec![B0, B0, B0, W1, W1, B0, B0, B0],
            vec![B0, B0, B0, W1, W1, B0, B0, B0],
            vec![B0, B0, B0, W1, W1, B0, B0, B0],
            vec![B0, B0, B0, W1, W1, B0, B0, B0],
        ];
        pattern
    }

    fn letter_u_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![B0, W1, W1, W1, W1, W1, W1, B0],
        ];
        pattern
    }

    fn letter_v_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, W1, B0, B0, W1, W1, W1],
            vec![B0, W1, W1, B0, B0, W1, W1, B0],
            vec![B0, W1, W1, B0, B0, W1, W1, B0],
            vec![B0, W1, W1, W1, W1, W1, W1, B0],
            vec![B0, B0, W1, W1, W1, W1, B0, B0],
            vec![B0, B0, W1, W1, W1, W1, B0, B0],
            vec![B0, B0, B0, W1, W1, B0, B0, B0],
        ];
        pattern
    }

    fn letter_w_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, W1, W1, B0, W1, W1],
            vec![W1, W1, B0, W1, W1, B0, W1, W1],
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![W1, W1, W1, B0, B0, W1, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
        ];
        pattern
    }

    fn letter_x_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, W1, B0, B0, W1, W1, W1],
            vec![B0, W1, W1, B0, B0, W1, W1, B0],
            vec![B0, W1, W1, W1, W1, W1, W1, B0],
            vec![B0, B0, W1, W1, W1, W1, B0, B0],
            vec![B0, B0, W1, W1, W1, W1, B0, B0],
            vec![B0, B0, B0, W1, W1, B0, B0, B0],
            vec![B0, B0, B0, W1, W1, B0, B0, B0],
            vec![B0, B0, B0, W1, W1, B0, B0, B0],
            vec![B0, B0, W1, W1, W1, W1, B0, B0],
            vec![B0, B0, W1, W1, W1, W1, B0, B0],
            vec![B0, W1, W1, W1, W1, W1, W1, B0],
            vec![B0, W1, W1, B0, B0, W1, W1, B0],
            vec![W1, W1, W1, B0, B0, W1, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
        ];
        pattern
    }

    fn letter_y_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, W1, B0, B0, W1, W1, W1],
            vec![B0, W1, W1, B0, B0, W1, W1, B0],
            vec![B0, W1, W1, B0, B0, W1, W1, B0],
            vec![B0, W1, W1, W1, W1, W1, W1, B0],
            vec![B0, B0, W1, W1, W1, W1, B0, B0],
            vec![B0, B0, W1, W1, W1, W1, B0, B0],
            vec![B0, B0, B0, W1, W1, B0, B0, B0],
            vec![B0, B0, B0, W1, W1, B0, B0, B0],
            vec![B0, B0, B0, W1, W1, B0, B0, B0],
            vec![B0, B0, B0, W1, W1, B0, B0, B0],
            vec![B0, B0, B0, W1, W1, B0, B0, B0],
            vec![B0, B0, B0, W1, W1, B0, B0, B0],
            vec![B0, B0, B0, W1, W1, B0, B0, B0],
            vec![B0, B0, B0, W1, W1, B0, B0, B0],
        ];
        pattern
    }

    fn letter_z_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![B0, B0, B0, B0, B0, B0, W1, W1],
            vec![B0, B0, B0, B0, B0, W1, W1, W1],
            vec![B0, B0, B0, B0, W1, W1, W1, B0],
            vec![B0, B0, B0, W1, W1, W1, B0, B0],
            vec![B0, B0, B0, W1, W1, B0, B0, B0],
            vec![B0, B0, W1, W1, W1, B0, B0, B0],
            vec![B0, B0, W1, W1, B0, B0, B0, B0],
            vec![B0, W1, W1, W1, B0, B0, B0, B0],
            vec![B0, W1, W1, B0, B0, B0, B0, B0],
            vec![W1, W1, W1, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
        ];
        pattern
    }

    fn number_one_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![B0, B0, W1, W1],
            vec![B0, W1, W1, W1],
            vec![W1, W1, W1, W1],
            vec![W1, W1, W1, W1],
            vec![B0, B0, W1, W1],
            vec![B0, B0, W1, W1],
            vec![B0, B0, W1, W1],
            vec![B0, B0, W1, W1],
            vec![B0, B0, W1, W1],
            vec![B0, B0, W1, W1],
            vec![B0, B0, W1, W1],
            vec![B0, B0, W1, W1],
            vec![B0, B0, W1, W1],
            vec![B0, B0, W1, W1],
            vec![B0, B0, W1, W1],
            vec![B0, B0, W1, W1],
        ];
        pattern
    }

    fn number_two_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![B0, W1, W1, W1, W1, W1, W1, B0],
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![B0, B0, B0, B0, B0, B0, W1, W1],
            vec![B0, B0, B0, B0, B0, W1, W1, W1],
            vec![B0, B0, B0, B0, W1, W1, W1, B0],
            vec![B0, B0, B0, W1, W1, W1, B0, B0],
            vec![B0, B0, W1, W1, W1, B0, B0, B0],
            vec![B0, W1, W1, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![B0, W1, W1, W1, W1, W1, W1, B0],
        ];
        pattern
    }

    fn number_three_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![B0, W1, W1, W1, W1, W1, W1, B0],
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![B0, B0, B0, B0, B0, B0, W1, W1],
            vec![B0, B0, B0, B0, B0, B0, W1, W1],
            vec![B0, B0, B0, B0, W1, W1, W1, B0],
            vec![B0, B0, B0, W1, W1, W1, B0, B0],
            vec![B0, B0, B0, B0, W1, W1, W1, B0],
            vec![B0, B0, B0, B0, B0, B0, W1, W1],
            vec![B0, B0, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![B0, W1, W1, W1, W1, W1, W1, B0],
        ];
        pattern
    }

    fn number_four_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![B0, B0, B0, B0, B0, W1, W1, B0],
            vec![B0, B0, B0, B0, W1, W1, W1, B0],
            vec![B0, B0, B0, W1, W1, W1, W1, B0],
            vec![B0, B0, W1, W1, B0, W1, W1, B0],
            vec![B0, W1, W1, B0, B0, W1, W1, B0],
            vec![W1, W1, B0, B0, B0, W1, W1, B0],
            vec![W1, W1, B0, B0, B0, W1, W1, B0],
            vec![W1, W1, B0, B0, B0, W1, W1, B0],
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![B0, B0, B0, B0, B0, W1, W1, B0],
            vec![B0, B0, B0, B0, B0, W1, W1, B0],
            vec![B0, B0, B0, B0, B0, W1, W1, B0],
            vec![B0, B0, B0, B0, B0, W1, W1, B0],
            vec![B0, B0, B0, B0, B0, W1, W1, B0],
            vec![B0, B0, B0, B0, B0, W1, W1, B0],
        ];
        pattern
    }

    fn number_five_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, W1, W1, W1, W1, W1, B0],
            vec![B0, W1, W1, W1, W1, W1, W1, B0],
            vec![B0, B0, B0, B0, B0, B0, W1, W1],
            vec![B0, B0, B0, B0, B0, B0, W1, W1],
            vec![B0, B0, B0, B0, B0, B0, W1, W1],
            vec![B0, B0, B0, B0, B0, B0, W1, W1],
            vec![B0, B0, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![B0, W1, W1, W1, W1, W1, W1, B0],
        ];
        pattern
    }

    fn number_six_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![B0, W1, W1, W1, W1, W1, W1, B0],
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, W1, W1, W1, W1, W1, B0],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![B0, W1, W1, W1, W1, W1, W1, B0],
        ];
        pattern
    }

    fn number_seven_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![B0, B0, B0, B0, B0, B0, W1, W1],
            vec![B0, B0, B0, B0, B0, W1, W1, W1],
            vec![B0, B0, B0, B0, W1, W1, W1, B0],
            vec![B0, B0, B0, W1, W1, W1, B0, B0],
            vec![B0, B0, W1, W1, W1, B0, B0, B0],
            vec![B0, W1, W1, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
            vec![W1, W1, B0, B0, B0, B0, B0, B0],
        ];
        pattern
    }

    fn number_eight_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![B0, W1, W1, W1, W1, W1, W1, B0],
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, W1, W1, W1, W1, W1, B0],
            vec![B0, W1, W1, W1, W1, W1, W1, B0],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![B0, W1, W1, W1, W1, W1, W1, B0],
        ];
        pattern
    }

    fn number_nine_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![B0, B0, W1, W1, W1, W1, B0, B0],
            vec![B0, W1, W1, W1, W1, W1, W1, B0],
            vec![W1, W1, W1, B0, B0, W1, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, W1, B0, B0, B0, W1, W1],
            vec![B0, W1, W1, W1, W1, W1, W1, W1],
            vec![B0, B0, W1, W1, W1, W1, W1, W1],
            vec![B0, B0, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, W1, B0, B0, B0, W1, W1],
            vec![B0, W1, W1, W1, W1, W1, W1, B0],
            vec![B0, B0, W1, W1, W1, W1, B0, B0],
        ];
        pattern
    }

    fn number_zero_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![B0, W1, W1, W1, W1, W1, W1, B0],
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, W1, W1, W1],
            vec![W1, W1, B0, B0, W1, W1, W1, W1],
            vec![W1, W1, B0, W1, W1, B0, W1, W1],
            vec![W1, W1, W1, W1, B0, B0, W1, W1],
            vec![W1, W1, W1, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, B0, B0, B0, B0, W1, W1],
            vec![W1, W1, W1, W1, W1, W1, W1, W1],
            vec![B0, W1, W1, W1, W1, W1, W1, B0],
        ];
        pattern
    }

    fn colon_pattern() -> BulbDisplay {
        let pattern = vec![
            vec![B0, B0],
            vec![B0, B0],
            vec![B0, B0],
            vec![B0, B0],
            vec![B0, B0],
            vec![W1, W1],
            vec![W1, W1],
            vec![B0, B0],
            vec![B0, B0],
            vec![B0, B0],
            vec![W1, W1],
            vec![W1, W1],
            vec![B0, B0],
            vec![B0, B0],
            vec![B0, B0],
            vec![B0, B0],
        ];
        pattern
    }
}
