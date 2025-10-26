use image::Rgb;

use crate::pattern;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    Z,
}
