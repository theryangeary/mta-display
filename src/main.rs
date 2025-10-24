use gif::{Encoder, Frame, Repeat};
use image::{ImageBuffer, Rgb};
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let width = 100;
    let height = 100;
    let num_frames = 1;

    // 1. Set up GIF encoder
    let mut image_file = File::create("output.gif")?;
    let mut encoder = Encoder::new(&mut image_file, width, height, &[])?;
    encoder.set_repeat(Repeat::Infinite)?;

    // 2. Generate frames
    for i in 0..num_frames {
        let mut img: ImageBuffer<Rgb<_>, Vec<u8>> = ImageBuffer::new(width.into(), height.into());

        // Draw your graphics here
        // ... manipulate pixels in img ...
        let (w, h) = img.dimensions();
        for row_num in 0..h {
            for col_num in 0..w {
                let row_from_center = u32::abs_diff(h / 2, row_num);
                let col_from_center = u32::abs_diff(w / 2, col_num);
                if ((row_from_center * row_from_center) + (col_from_center * col_from_center))
                    .isqrt()
                    < 50
                {
                    img[(row_num, col_num)] = Rgb([24, 48, 96]);
                };
            }
        }

        // 3. Add frame to GIF
        let frame = Frame::from_rgb(width, height, &img.into_raw());
        encoder.write_frame(&frame)?;
    }

    Ok(())
}
