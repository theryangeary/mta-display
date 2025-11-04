use std::str::FromStr;

use image::Rgb;
use serde::{Deserialize, Serialize};
use sqlx::{Decode, Sqlite};

use crate::pattern;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::EnumString)]
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
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, strum::EnumString, strum::Display,
)]
#[derive(sqlx::Type)]
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
    fn test_train_from_str() {
        assert_eq!(Train::from_str("One").unwrap(), Train::One);
        assert_eq!(Train::from_str("A").unwrap(), Train::A);
        assert_eq!(Train::from_str("Z").unwrap(), Train::Z);
    }
}
