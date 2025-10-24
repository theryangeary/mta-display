use gif::{Encoder, Frame, Repeat};
use image::{ImageBuffer, Rgb};
use std::fs::File;

struct BulbDisplayConfig {
    num_bulb_rows: u16,
    num_bulb_cols: u16,
    /// the number of pixels around the border of the display that do not have bulbs on them
    display_margin: u16,
    /// the resulting image height in pixels
    img_height: u16,
    /// the resulting image width in pixels
    img_width: u16,
    /// bulb_size_ratio is the ratio of the bulb diameter to the bounding box width
    bulb_size_ratio: f64,
}

impl BulbDisplayConfig {
    fn new(
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

    fn bulb_region_side_length(&self) -> u16 {
        (self.img_height - (2 * self.display_margin)) / self.num_bulb_rows
    }

    fn bulb_width(&self) -> u16 {
        (self.bulb_region_side_length() as f64 * self.bulb_size_ratio) as u16
    }

    fn img_width(&self) -> u16 {
        self.img_width
    }

    fn img_height(&self) -> u16 {
        self.img_height
    }
}

#[allow(non_snake_case)]
fn train_bullet_pattern() -> Vec<Vec<Rgb<u8>>> {
    let Bl = Rgb([0, 204, 255]);
    let W1 = Rgb([255, 255, 255]);
    let B0 = Rgb([0, 0, 0]);
    let pattern = vec![
        vec![
            B0, B0, B0, B0, B0, Bl, Bl, Bl, Bl, Bl, Bl, B0, B0, B0, B0, B0,
        ],
        vec![
            B0, B0, B0, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, B0, B0, B0,
        ],
        vec![
            B0, B0, Bl, Bl, Bl, Bl, Bl, W1, W1, Bl, Bl, Bl, Bl, Bl, B0, B0,
        ],
        vec![
            B0, Bl, Bl, Bl, Bl, Bl, Bl, W1, W1, Bl, Bl, Bl, Bl, Bl, Bl, B0,
        ],
        vec![
            B0, Bl, Bl, Bl, Bl, Bl, W1, W1, W1, W1, Bl, Bl, Bl, Bl, Bl, B0,
        ],
        vec![
            Bl, Bl, Bl, Bl, Bl, W1, W1, Bl, Bl, W1, W1, Bl, Bl, Bl, Bl, Bl,
        ],
        vec![
            Bl, Bl, Bl, Bl, Bl, W1, W1, Bl, Bl, W1, W1, Bl, Bl, Bl, Bl, Bl,
        ],
        vec![
            Bl, Bl, Bl, Bl, Bl, W1, W1, Bl, Bl, W1, W1, Bl, Bl, Bl, Bl, Bl,
        ],
        vec![
            Bl, Bl, Bl, Bl, W1, W1, Bl, Bl, Bl, Bl, W1, W1, Bl, Bl, Bl, Bl,
        ],
        vec![
            Bl, Bl, Bl, Bl, W1, W1, W1, W1, W1, W1, W1, W1, Bl, Bl, Bl, Bl,
        ],
        vec![
            Bl, Bl, Bl, Bl, W1, W1, W1, W1, W1, W1, W1, W1, Bl, Bl, Bl, Bl,
        ],
        vec![
            B0, Bl, Bl, W1, W1, Bl, Bl, Bl, Bl, Bl, Bl, W1, W1, Bl, Bl, B0,
        ],
        vec![
            B0, Bl, Bl, W1, W1, Bl, Bl, Bl, Bl, Bl, Bl, W1, W1, Bl, Bl, B0,
        ],
        vec![
            B0, B0, Bl, W1, W1, Bl, Bl, Bl, Bl, Bl, Bl, W1, W1, Bl, B0, B0,
        ],
        vec![
            B0, B0, B0, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, Bl, B0, B0, B0,
        ],
        vec![
            B0, B0, B0, B0, B0, Bl, Bl, Bl, Bl, Bl, Bl, B0, B0, B0, B0, B0,
        ],
    ];

    pattern
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // config setup
    let margin = 10;
    let bulb_rows = 16;
    let bulb_cols = 160;
    let bulb_bounding_box_size = 20;
    let bulb_size_ratio = 0.75;
    let config = BulbDisplayConfig::new(
        bulb_rows,
        bulb_cols,
        margin,
        bulb_bounding_box_size,
        bulb_size_ratio,
    );

    // Example bulb color array (all bulbs set to a warm yellow)
    let mut bulb_array = vec![vec![Rgb([0, 0, 0]); bulb_cols.into()]; bulb_rows.into()];
    // draw an A train bullet in the left edge of the bulb array
    let train_bullet = train_bullet_pattern();
    for (row_num, row) in train_bullet.iter().enumerate() {
        for (col_num, &rgb) in row.iter().enumerate() {
            bulb_array[row_num][col_num] = rgb;
        }
    }

    // 1. Set up GIF encoder
    let num_frames = 1;
    let mut image_file = File::create("output.gif")?;
    let mut encoder = Encoder::new(
        &mut image_file,
        config.img_width(),
        config.img_height(),
        &[],
    )?;
    encoder.set_repeat(Repeat::Infinite)?;

    // 2. Generate frames
    for i in 0..num_frames {
        let mut img: ImageBuffer<Rgb<_>, Vec<u8>> =
            ImageBuffer::new(config.img_width().into(), config.img_height().into());

        // Draw your graphics here
        // ... manipulate pixels in img ...
        for (row_num, row) in bulb_array.iter().enumerate() {
            for (col_num, rgb) in row.iter().enumerate() {
                draw_bulb(&mut img, &config, row_num as u16, col_num as u16, *rgb)
            }
        }

        // 3. Add frame to GIF
        let frame = Frame::from_rgb(config.img_width(), config.img_height(), &img.into_raw());
        encoder.write_frame(&frame)?;
    }

    Ok(())
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
