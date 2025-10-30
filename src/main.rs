use axum::routing::post;
use gif::{Encoder, Frame, Repeat};
use image::{ImageBuffer, Rgb};
use rust_embed::Embed;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::str::FromStr;

use std::io::Write;

use axum::extract::{Path, Query};
use axum::http::{HeaderMap, HeaderValue, StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::{Form, Json};
use axum::{Router, routing::get};
use maud::{DOCTYPE, Markup, html};
use serde::Deserialize;
use tokio::signal;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod pattern;
mod types;
use types::BulbDisplay;
use types::BulbDisplayConfig;

use crate::types::BulbDisplaySize;

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

    let app = Router::new()
        .route("/", get(get_index_markup))
        .route("/health", get(health_check))
        .route("/static/{file}", get(get_static_file))
        .route("/generate", post(post_generate))
        .route("/gif/{size}/{message}", get(get_gif_file))
        .layer(TraceLayer::new_for_http());

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

#[derive(Deserialize)]
struct GenerateGifForm {
    message: String,
}

/// Handle form submission to generate a new GIF, returning the updated markup to replace the existing image.
async fn post_generate(
    Form(generate_gif_form): Form<GenerateGifForm>,
) -> Result<Response, StatusCode> {
    let url = HeaderValue::from_str(&format!("/?message={}", &generate_gif_form.message)).map_err(
        |e| {
            tracing::error!("failed to turn message param into push-url: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        },
    )?;

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("text/html"));
    headers.insert("hx-push-url", url);

    Ok((headers, gif_markup(&generate_gif_form.message)).into_response())
}

fn gif_markup(message: &str) -> Markup {
    html! {
        div class="
            flex
            justify-center
            mb-8
        " {
            img
                id="mta-sign-gif"
                src=(&format!("/gif/sm/{}", message))
                alt=(&format!("Generated MTA Display with message {}", message))
                class="h-auto max-w-full"
            ;
        }
    }
}

async fn get_gif_file(Path((size, message)): Path<(String, String)>) -> Response {
    let bulb_display_size = match BulbDisplaySize::from_str(&size) {
        Ok(s) => s,
        Err(e) => {
            tracing::error!("failed to parse size parameter from '{}': {}", size, e);
            tracing::debug!("defaulting to XSmall size");
            BulbDisplaySize::XSmall
        }
    };
    let config = BulbDisplayConfig::new_from_size(bulb_display_size);

    let train = types::Train::A;

    let uppercase_message = message.to_ascii_uppercase();
    let message_parts = split_message_into_parts(&config, &uppercase_message);

    let frames = generate_frames_for_message(&config, train, message_parts).unwrap();

    let gif_data = write_frames_to_gif_in_memory(&config, &frames).unwrap();

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("image/gif"));

    (headers, gif_data).into_response()
}

async fn get_index_markup(Query(message): Query<HashMap<String, String>>) -> Markup {
    let message = message
        .get("message")
        .cloned()
        .unwrap_or_else(|| "Welcome to the MTA display generator".into());
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
                    (gif_markup(&message))
                    form
                        hx-post="/generate"
                        hx-target="#mta-sign-gif"
                        hx-swap="outerHTML"
                        class="
                            bg-gray-800
                            border-gray-700
                            p-4
                            rounded-xl
                            mb-4
                        "
                    {
                        label class="text-gray-200 flex w-100% block mb-2" for="message" {
                            span class="flex-grow" { "Message: " }
                            span class="flex-shrink relative group" {
                                button type="button" class="
                                    text-gray-500 
                                    cursor-pointer
                                    focus:outline-none
                                    " 
                                    onclick="this.nextElementSibling.classList.toggle('hidden')"
                                {
                                    "ⓘ"
                                }

                                span class="
                                    absolute 
                                    hidden 
                                    group-hover:block 
                                    bg-gray-300 
                                    text-black 
                                    text-xs 
                                    rounded 
                                    py-1 
                                    px-2  
                                    -left-50 
                                    top-full 
                                    mb-1
                                    z-10 
                                    whitespace-nowrap
                                    " 
                                {
                                    p class="mb-1" {"Max 6 rows, 14 characters per row. "}
                                    p class="mb-1" {"Use linebreaks to separate pages manually. "}
                                    p class="mb-1" {"Unsupported characters will be ignored." }
                                }
                            }
                        }
                        textarea
                            class="
                                w-full
                                p-2
                                mt-2
                                mb-4
                                bg-white
                                border
                                border-gray-600
                                rounded-lg
                                text-black
                                focus:outline-none
                                focus:border-blue-500"
                            name="message"
                            id="message"
                            rows="4"
                            placeholder="Type your message here..." {
                                (message)
                            }
                        br;
                        button type="submit" class="
                            bg-yellow-500 
                            text-black 
                            font-bold
                            py-2 
                            px-4 
                            rounded 
                            hover:bg-yellow-600
                        "
                        { "Generate" }
                    }

                    h2 { "About"}
                    p class="prose" { "This is a fun side project that generates GIFs that simulate a display as you would see on the New York City MTA Subway." }
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
        let frame =
            Frame::from_rgb_speed(config.img_width(), config.img_height(), &img.into_raw(), 30);

        let frame_duration = 10;
        for _ in 0..frame_duration {
            encoder.write_frame(&frame)?;
        }
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
    // trailing whitespace to a row.
    for (i, chr) in message.char_indices() {
        if chr.is_whitespace() {
            last_space = i;
        }
        if i - start >= config.max_chars_per_row() as usize {
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
    // todo find actual bullet width
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
}
