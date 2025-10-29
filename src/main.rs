use gif::{Encoder, Frame, Repeat};
use image::{ImageBuffer, Rgb};
use std::error::Error;
use std::fs::File;

use std::io::Write;

use axum::Json;
use axum::extract::Path;
use axum::http::{HeaderMap, HeaderValue, header};
use axum::response::{IntoResponse, Response};
use axum::{Router, routing::get};
use maud::{Markup, html};
use tokio::signal;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod pattern;
mod types;
use types::BulbDisplay;
use types::BulbDisplayConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "debug".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();


        let app = Router::new()
        .route("/", get(get_index))
        .route("/health", get(health_check))
        .route("/gif/{message}", get(get_gif))
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

async fn get_gif(Path(message): Path<String>) -> Response {
    // config setup
    let margin = 10;
    let bulb_rows = 16;
    let bulb_cols = 160;
    let bulb_bounding_box_size = 8;
    let bulb_size_ratio = 0.75;

    let config = BulbDisplayConfig::new(
        bulb_rows,
        bulb_cols,
        margin,
        bulb_bounding_box_size,
        bulb_size_ratio,
    );

    let train = types::Train::A;

    let message_parts = split_message_into_parts(&config, &message.to_ascii_uppercase());

    let frames = generate_frames_for_message(&config, train, message_parts).unwrap();

    let gif_data = write_frames_to_gif_in_memory(&config, &frames).unwrap();

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("image/gif"));

    (headers, gif_data).into_response()
}

async fn get_index() -> Markup {
    html! {
        body {
            h1 { "MTA Sign GIF Generator" }
            p { "This is a simple web application that generates GIFs that simulate an MTA subway sign." }
            img src="/gif/Welcome to the A train line! Enjoy your ride." alt="Sample MTA Sign GIF";
        }
    }
}

async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
    }))
}

fn write_frames_to_gif_at_path(config: &BulbDisplayConfig, frames: &Vec<BulbDisplay>, path: std::path::PathBuf) -> Result<(), Box<dyn Error>> {
    let mut image_file = File::create(path)?;
    write_frames_to_gif(config, frames, &mut image_file)
}

fn write_frames_to_gif_in_memory(config: &BulbDisplayConfig, frames: &Vec<BulbDisplay>) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut buffer: Vec<u8> = vec![];
    write_frames_to_gif(config, frames, &mut buffer)?;
    Ok(buffer)
}

fn write_frames_to_gif(config: &BulbDisplayConfig, frames: &Vec<BulbDisplay>, output_gif: &mut dyn Write) -> Result<(), Box<dyn Error>> {
    let mut encoder = Encoder::new(
         output_gif,
        config.img_width(),
        config.img_height(),
        &[],
    )?;
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
        let frame = Frame::from_rgb_speed(config.img_width(), config.img_height(), &img.into_raw(), 30);

        let frame_duration = 10;
        for _ in 0..frame_duration {
            encoder.write_frame(&frame)?;
        }
    })
}

fn generate_frames_for_message(config: &BulbDisplayConfig, train: types::Train, message_parts: Vec<String>) -> Result<Vec<BulbDisplay>, Box<dyn Error>> {
    let mut frames = vec![];
    for msg in &message_parts {
        let mut bulb_array: BulbDisplay = vec![vec![Rgb([0, 0, 0]); config.num_bulb_cols.into()]; config.num_bulb_rows.into()];

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

fn split_message_into_parts(config: &BulbDisplayConfig, message: &str) -> Vec<String> {
    let mut message_parts = vec![];

    let mut words = message.split_whitespace().peekable();
    while let Some(_) = words.peek() {
        let mut message_part = String::new();

        while let Some(next_word) = words.peek() {
            if message_part.len() + next_word.len() <= config.max_chars_per_row() as usize {
                message_part.push_str(next_word);
                message_part.push(' ');
                words.next();
            } else {
                // current message part is full
                break;
            }
        }

        message_parts.push(message_part);
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
    let bullet_width = 20;
    let max_chars = (num_cols - bullet_width) / pattern::LETTER_PATTERN_SLOT_WIDTH as usize;
    // todo split on words when possible
    let left_pad = if message.len() < max_chars {
        bullet_width
            + ((max_chars - message.len()) * pattern::LETTER_PATTERN_SLOT_WIDTH as usize) / 2
    } else {
        bullet_width
    };

    'CHARS: for (i, c) in message.chars().enumerate() {
        let char_pattern = pattern::pattern_for_letter(c);

        for (row_num, row) in char_pattern.iter().enumerate() {
            for (col_num, &rgb) in row.iter().enumerate() {
                let target_row = row_num;
                let target_col =
                    left_pad + col_num + (i as usize * pattern::LETTER_PATTERN_SLOT_WIDTH as usize);
                if target_row >= bulb_array.len() || target_col >= bulb_array[0].len() {
                    break 'CHARS;
                }
                bulb_array[target_row][target_col] = rgb;
            }
        }

        ret += 1;
    }
    Ok(ret)
}
