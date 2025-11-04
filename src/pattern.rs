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
        vec![ B0, B0, B0, B0, B0, OR, OR, OR, OR, OR, OR, B0, B0, B0, B0, B0, ],
        vec![ B0, B0, B0, OR, OR, OR, OR, OR, OR, OR, OR, OR, OR, B0, B0, B0, ],
        vec![ B0, B0, OR, OR, W1, W1, W1, W1, W1, W1, OR, OR, OR, OR, B0, B0, ],
        vec![ B0, OR, OR, OR, W1, W1, W1, W1, W1, W1, W1, OR, OR, OR, OR, B0, ],
        vec![ B0, OR, OR, OR, W1, W1, OR, OR, OR, OR, W1, W1, OR, OR, OR, B0, ],
        vec![ OR, OR, OR, OR, W1, W1, OR, OR, OR, OR, W1, W1, OR, OR, OR, OR, ],
        vec![ OR, OR, OR, OR, W1, W1, OR, OR, OR, OR, W1, W1, OR, OR, OR, OR, ],
        vec![ B0, OR, OR, OR, W1, W1, OR, OR, OR, OR, W1, W1, OR, OR, OR, B0, ],
        vec![ B0, OR, OR, OR, W1, W1, W1, W1, W1, W1, W1, OR, OR, OR, OR, B0, ],
        vec![ B0, B0, OR, OR, OR, W1, W1, W1, W1, W1, OR, OR, OR, OR, B0, B0, ],
        vec![ B0, B0, B0, OR, OR, OR, OR, OR, OR, OR, OR, OR, OR, B0, B0, B0, ],
        vec![ B0, B0, B0, B0, B0, OR, OR, OR, OR, OR, OR, B0, B0, B0, B0, B0, ],
    ];
    pattern
}

fn c_train_bullet_pattern() -> BulbDisplay {
    let pattern = vec![
        vec![ B0, B0, B0, B0, B0, BL, BL, BL, BL, BL, BL, B0, B0, B0, B0, B0, ],
        vec![ B0, B0, B0, BL, BL, BL, W1, W1, W1, BL, BL, BL, BL, B0, B0, B0, ],
        vec![ B0, B0, BL, BL, BL, W1, W1, W1, W1, W1, BL, BL, BL, BL, B0, B0, ],
        vec![ B0, BL, BL, BL, W1, W1, BL, BL, BL, W1, W1, BL, BL, BL, BL, B0, ],
        vec![ B0, BL, BL, BL, W1, W1, BL, BL, BL, BL, BL, BL, BL, BL, BL, B0, ],
        vec![ BL, BL, BL, W1, W1, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, ],
        vec![ BL, BL, BL, W1, W1, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, ],
        vec![ BL, BL, BL, W1, W1, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, ],
        vec![ BL, BL, BL, W1, W1, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, ],
        vec![ BL, BL, BL, W1, W1, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, ],
        vec![ BL, BL, BL, W1, W1, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, ],
        vec![ B0, BL, BL, BL, W1, W1, BL, BL, BL, BL, BL, BL, BL, BL, BL, B0, ],
        vec![ B0, BL, BL, BL, W1, W1, BL, BL, BL, W1, W1, BL, BL, BL, BL, B0, ],
        vec![ B0, B0, BL, BL, BL, W1, W1, W1, W1, W1, BL, BL, BL, BL, B0, B0, ],
        vec![ B0, B0, B0, BL, BL, BL, W1, W1, W1, BL, BL, BL, BL, B0, B0, B0, ],
        vec![ B0, B0, B0, B0, B0, BL, BL, BL, BL, BL, BL, B0, B0, B0, B0, B0, ],
    ];
    pattern
}

fn d_train_bullet_pattern() -> BulbDisplay {
    let pattern = vec![
        vec![ B0, B0, B0, B0, B0, OR, OR, OR, OR, OR, OR, B0, B0, B0, B0, B0, ],
        vec![ B0, B0, B0, OR, OR, OR, OR, OR, OR, OR, OR, OR, OR, B0, B0, B0, ],
        vec![ B0, B0, OR, OR, W1, W1, W1, W1, W1, W1, OR, OR, OR, OR, B0, B0, ],
        vec![ B0, OR, OR, OR, W1, W1, W1, W1, W1, W1, W1, OR, OR, OR, OR, B0, ],
        vec![ B0, OR, OR, OR, W1, W1, OR, OR, OR, OR, W1, W1, OR, OR, OR, B0, ],
        vec![ OR, OR, OR, OR, W1, W1, OR, OR, OR, OR, W1, W1, OR, OR, OR, OR, ],
        vec![ OR, OR, OR, OR, W1, W1, OR, OR, OR, OR, W1, W1, OR, OR, OR, OR, ],
        vec![ OR, OR, OR, OR, W1, W1, OR, OR, OR, OR, W1, W1, OR, OR, OR, OR, ],
        vec![ OR, OR, OR, OR, W1, W1, OR, OR, OR, OR, W1, W1, OR, OR, OR, OR, ],
        vec![ OR, OR, OR, OR, W1, W1, OR, OR, OR, OR, W1, W1, OR, OR, OR, OR, ],
        vec![ OR, OR, OR, OR, W1, W1, OR, OR, OR, OR, W1, W1, OR, OR, OR, OR, ],
        vec![ B0, OR, OR, OR, W1, W1, OR, OR, OR, OR, W1, W1, OR, OR, OR, B0, ],
        vec![ B0, OR, OR, OR, W1, W1, W1, W1, W1, W1, W1, OR, OR, OR, OR, B0, ],
        vec![ B0, B0, OR, OR, OR, W1, W1, W1, W1, W1, OR, OR, OR, OR, B0, B0, ],
        vec![ B0, B0, B0, OR, OR, OR, OR, OR, OR, OR, OR, OR, OR, B0, B0, B0, ],
        vec![ B0, B0, B0, B0, B0, OR, OR, OR, OR, OR, OR, B0, B0, B0, B0, B0, ],
    ];
    pattern
}

fn e_train_bullet_pattern() -> BulbDisplay {
    let pattern = vec![
        vec![ B0, B0, B0, B0, B0, BL, BL, BL, BL, BL, BL, B0, B0, B0, B0, B0, ],
        vec![ B0, B0, B0, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, B0, B0, B0, ],
        vec![ B0, B0, BL, BL, W1, W1, W1, W1, W1, W1, W1, W1, BL, BL, B0, B0, ],
        vec![ B0, BL, BL, BL, W1, W1, W1, W1, W1, W1, W1, W1, BL, BL, BL, B0, ],
        vec![ B0, BL, BL, BL, W1, W1, BL, BL, BL, BL, BL, BL, BL, BL, BL, B0, ],
        vec![ BL, BL, BL, BL, W1, W1, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, ],
        vec![ BL, BL, BL, BL, W1, W1, W1, W1, W1, W1, BL, BL, BL, BL, BL, BL, ],
        vec![ BL, BL, BL, BL, W1, W1, W1, W1, W1, W1, BL, BL, BL, BL, BL, BL, ],
        vec![ BL, BL, BL, BL, W1, W1, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, ],
        vec![ BL, BL, BL, BL, W1, W1, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, ],
        vec![ BL, BL, BL, BL, W1, W1, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, ],
        vec![ B0, BL, BL, BL, W1, W1, BL, BL, BL, BL, BL, BL, BL, BL, BL, B0, ],
        vec![ B0, BL, BL, BL, W1, W1, W1, W1, W1, W1, W1, W1, BL, BL, BL, B0, ],
        vec![ B0, B0, BL, BL, W1, W1, W1, W1, W1, W1, W1, W1, BL, BL, B0, B0, ],
        vec![ B0, B0, B0, BL, BL, BL, BL, BL, BL, BL, BL, BL, BL, B0, B0, B0, ],
        vec![ B0, B0, B0, B0, B0, BL, BL, BL, BL, BL, BL, B0, B0, B0, B0, B0, ],
    ];
    pattern
}

fn f_train_bullet_pattern() -> BulbDisplay {
    let pattern = vec![
        vec![ B0, B0, B0, B0, B0, OR, OR, OR, OR, OR, OR, B0, B0, B0, B0, B0, ],
        vec![ B0, B0, B0, OR, OR, OR, OR, OR, OR, OR, OR, OR, OR, B0, B0, B0, ],
        vec![ B0, B0, OR, OR, W1, W1, W1, W1, W1, W1, W1, W1, OR, OR, B0, B0, ],
        vec![ B0, OR, OR, OR, W1, W1, W1, W1, W1, W1, W1, W1, OR, OR, OR, B0, ],
        vec![ B0, OR, OR, OR, W1, W1, OR, OR, OR, OR, OR, OR, OR, OR, OR, B0, ],
        vec![ OR, OR, OR, OR, W1, W1, OR, OR, OR, OR, OR, OR, OR, OR, OR, OR, ],
        vec![ OR, OR, OR, OR, W1, W1, W1, W1, W1, W1, OR, OR, OR, OR, OR, OR, ],
        vec![ OR, OR, OR, OR, W1, W1, W1, W1, W1, W1, OR, OR, OR, OR, OR, OR, ],
        vec![ OR, OR, OR, OR, W1, W1, OR, OR, OR, OR, OR, OR, OR, OR, OR, OR, ],
        vec![ OR, OR, OR, OR, W1, W1, OR, OR, OR, OR, OR, OR, OR, OR, OR, OR, ],
        vec![ OR, OR, OR, OR, W1, W1, OR, OR, OR, OR, OR, OR, OR, OR, OR, OR, ],
        vec![ B0, OR, OR, OR, W1, W1, OR, OR, OR, OR, OR, OR, OR, OR, OR, B0, ],
        vec![ B0, OR, OR, OR, W1, W1, OR, OR, OR, OR, OR, OR, OR, OR, OR, B0, ],
        vec![ B0, B0, OR, OR, W1, W1, OR, OR, OR, OR, OR, OR, OR, OR, B0, B0, ],
        vec![ B0, B0, B0, OR, OR, OR, OR, OR, OR, OR, OR, OR, OR, B0, B0, B0, ],
        vec![ B0, B0, B0, B0, B0, OR, OR, OR, OR, OR, OR, B0, B0, B0, B0, B0, ],
    ];
    pattern
}

fn g_train_bullet_pattern() -> BulbDisplay {
    let pattern = vec![
        vec![ B0, B0, B0, B0, B0, LG, LG, LG, LG, LG, LG, B0, B0, B0, B0, B0, ],
        vec![ B0, B0, B0, LG, LG, LG, W1, W1, W1, LG, LG, LG, LG, B0, B0, B0, ],
        vec![ B0, B0, LG, LG, LG, W1, W1, W1, W1, W1, LG, LG, LG, LG, B0, B0, ],
        vec![ B0, LG, LG, LG, W1, W1, LG, LG, LG, W1, W1, LG, LG, LG, LG, B0, ],
        vec![ B0, LG, LG, LG, W1, W1, LG, LG, LG, LG, LG, LG, LG, LG, LG, B0, ],
        vec![ LG, LG, LG, W1, W1, LG, LG, LG, LG, LG, LG, LG, LG, LG, LG, LG, ],
        vec![ LG, LG, LG, W1, W1, LG, LG, LG, LG, LG, LG, LG, LG, LG, LG, LG, ],
        vec![ LG, LG, LG, W1, W1, LG, LG, W1, W1, W1, W1, LG, LG, LG, LG, LG, ],
        vec![ LG, LG, LG, W1, W1, LG, LG, LG, LG, W1, W1, LG, LG, LG, LG, LG, ],
        vec![ LG, LG, LG, W1, W1, LG, LG, LG, LG, W1, W1, LG, LG, LG, LG, LG, ],
        vec![ LG, LG, LG, W1, W1, LG, LG, LG, LG, W1, W1, LG, LG, LG, LG, LG, ],
        vec![ B0, LG, LG, LG, W1, W1, LG, LG, LG, W1, W1, LG, LG, LG, LG, B0, ],
        vec![ B0, LG, LG, LG, W1, W1, W1, W1, W1, W1, LG, LG, LG, LG, LG, B0, ],
        vec![ B0, B0, LG, LG, LG, W1, W1, W1, W1, LG, LG, LG, LG, LG, B0, B0, ],
        vec![ B0, B0, B0, LG, LG, LG, LG, LG, LG, LG, LG, LG, LG, B0, B0, B0, ],
        vec![ B0, B0, B0, B0, B0, LG, LG, LG, LG, LG, LG, B0, B0, B0, B0, B0, ],
    ];
    pattern
}

fn j_train_bullet_pattern() -> BulbDisplay {
    let pattern = vec![
        vec![ B0, B0, B0, B0, B0, BR, BR, BR, BR, BR, BR, B0, B0, B0, B0, B0, ],
        vec![ B0, B0, B0, BR, BR, BR, BR, BR, BR, BR, BR, BR, BR, B0, B0, B0, ],
        vec![ B0, B0, BR, BR, BR, BR, BR, BR, BR, W1, W1, BR, BR, BR, B0, B0, ],
        vec![ B0, BR, BR, BR, BR, BR, BR, BR, BR, W1, W1, BR, BR, BR, BR, B0, ],
        vec![ B0, BR, BR, BR, BR, BR, BR, BR, BR, W1, W1, BR, BR, BR, BR, B0, ],
        vec![ BR, BR, BR, BR, BR, BR, BR, BR, BR, W1, W1, BR, BR, BR, BR, BR, ],
        vec![ BR, BR, BR, BR, BR, BR, BR, BR, BR, W1, W1, BR, BR, BR, BR, BR, ],
        vec![ BR, BR, BR, BR, BR, BR, BR, BR, BR, W1, W1, BR, BR, BR, BR, BR, ],
        vec![ BR, BR, BR, BR, BR, BR, BR, BR, BR, W1, W1, BR, BR, BR, BR, BR, ],
        vec![ BR, BR, BR, BR, BR, BR, BR, BR, BR, W1, W1, BR, BR, BR, BR, BR, ],
        vec![ BR, BR, BR, BR, BR, BR, BR, BR, BR, W1, W1, BR, BR, BR, BR, BR, ],
        vec![ B0, BR, BR, BR, W1, W1, BR, BR, BR, W1, W1, BR, BR, BR, BR, B0, ],
        vec![ B0, BR, BR, BR, W1, W1, W1, W1, W1, W1, BR, BR, BR, BR, BR, B0, ],
        vec![ B0, B0, BR, BR, BR, W1, W1, W1, W1, BR, BR, BR, BR, BR, B0, B0, ],
        vec![ B0, B0, B0, BR, BR, BR, BR, BR, BR, BR, BR, BR, BR, B0, B0, B0, ],
        vec![ B0, B0, B0, B0, B0, BR, BR, BR, BR, BR, BR, B0, B0, B0, B0, B0, ],
    ];
    pattern
}

fn l_train_bullet_pattern() -> BulbDisplay {
    let pattern = vec![
        vec![ B0, B0, B0, B0, B0, GY, GY, GY, GY, GY, GY, B0, B0, B0, B0, B0, ],
        vec![ B0, B0, B0, GY, GY, GY, GY, GY, GY, GY, GY, GY, GY, B0, B0, B0, ],
        vec![ B0, B0, GY, GY, W1, W1, GY, GY, GY, GY, GY, GY, GY, GY, B0, B0, ],
        vec![ B0, GY, GY, GY, W1, W1, GY, GY, GY, GY, GY, GY, GY, GY, GY, B0, ],
        vec![ B0, GY, GY, GY, W1, W1, GY, GY, GY, GY, GY, GY, GY, GY, GY, B0, ],
        vec![ GY, GY, GY, GY, W1, W1, GY, GY, GY, GY, GY, GY, GY, GY, GY, GY, ],
        vec![ GY, GY, GY, GY, W1, W1, GY, GY, GY, GY, GY, GY, GY, GY, GY, GY, ],
        vec![ GY, GY, GY, GY, W1, W1, GY, GY, GY, GY, GY, GY, GY, GY, GY, GY, ],
        vec![ GY, GY, GY, GY, W1, W1, GY, GY, GY, GY, GY, GY, GY, GY, GY, GY, ],
        vec![ GY, GY, GY, GY, W1, W1, GY, GY, GY, GY, GY, GY, GY, GY, GY, GY, ],
        vec![ GY, GY, GY, GY, W1, W1, GY, GY, GY, GY, GY, GY, GY, GY, GY, GY, ],
        vec![ B0, GY, GY, GY, W1, W1, GY, GY, GY, GY, GY, GY, GY, GY, GY, B0, ],
        vec![ B0, GY, GY, GY, W1, W1, W1, W1, W1, W1, W1, W1, GY, GY, GY, B0, ],
        vec![ B0, B0, GY, GY, W1, W1, W1, W1, W1, W1, W1, W1, GY, GY, B0, B0, ],
        vec![ B0, B0, B0, GY, GY, GY, GY, GY, GY, GY, GY, GY, GY, B0, B0, B0, ],
        vec![ B0, B0, B0, B0, B0, GY, GY, GY, GY, GY, GY, B0, B0, B0, B0, B0, ],
    ];
    pattern
}

fn m_train_bullet_pattern() -> BulbDisplay {
    let pattern = vec![
        vec![ B0, B0, B0, B0, B0, OR, OR, OR, OR, OR, OR, B0, B0, B0, B0, B0, ],
        vec![ B0, B0, B0, OR, OR, OR, OR, OR, OR, OR, OR, OR, OR, B0, B0, B0, ],
        vec![ B0, B0, OR, OR, W1, W1, OR, OR, OR, OR, W1, W1, OR, OR, B0, B0, ],
        vec![ B0, OR, OR, OR, W1, W1, OR, OR, OR, OR, W1, W1, OR, OR, OR, B0, ],
        vec![ B0, OR, OR, OR, W1, W1, W1, OR, OR, W1, W1, W1, OR, OR, OR, B0, ],
        vec![ OR, OR, OR, OR, W1, W1, W1, OR, OR, W1, W1, W1, OR, OR, OR, OR, ],
        vec![ OR, OR, OR, OR, W1, W1, W1, W1, W1, W1, W1, W1, OR, OR, OR, OR, ],
        vec![ OR, OR, OR, OR, W1, W1, OR, W1, W1, OR, W1, W1, OR, OR, OR, OR, ],
        vec![ OR, OR, OR, OR, W1, W1, OR, W1, W1, OR, W1, W1, OR, OR, OR, OR, ],
        vec![ OR, OR, OR, OR, W1, W1, OR, OR, OR, OR, W1, W1, OR, OR, OR, OR, ],
        vec![ OR, OR, OR, OR, W1, W1, OR, OR, OR, OR, W1, W1, OR, OR, OR, OR, ],
        vec![ B0, OR, OR, OR, W1, W1, OR, OR, OR, OR, W1, W1, OR, OR, OR, B0, ],
        vec![ B0, OR, OR, OR, W1, W1, OR, OR, OR, OR, W1, W1, OR, OR, OR, B0, ],
        vec![ B0, B0, OR, OR, W1, W1, OR, OR, OR, OR, W1, W1, OR, OR, B0, B0, ],
        vec![ B0, B0, B0, OR, OR, OR, OR, OR, OR, OR, OR, OR, OR, B0, B0, B0, ],
        vec![ B0, B0, B0, B0, B0, OR, OR, OR, OR, OR, OR, B0, B0, B0, B0, B0, ],
    ];
    pattern
}

fn n_train_bullet_pattern() -> BulbDisplay {
    let pattern = vec![
        vec![ B0, B0, B0, B0, B0, YE, YE, YE, YE, YE, YE, B0, B0, B0, B0, B0, ],
        vec![ B0, B0, B0, YE, YE, YE, YE, YE, YE, YE, YE, YE, YE, B0, B0, B0, ],
        vec![ B0, B0, YE, YE, W1, W1, YE, YE, YE, YE, W1, W1, YE, YE, B0, B0, ],
        vec![ B0, YE, YE, YE, W1, W1, YE, YE, YE, YE, W1, W1, YE, YE, YE, B0, ],
        vec![ B0, YE, YE, YE, W1, W1, W1, YE, YE, YE, W1, W1, YE, YE, YE, B0, ],
        vec![ YE, YE, YE, YE, W1, W1, W1, YE, YE, YE, W1, W1, YE, YE, YE, YE, ],
        vec![ YE, YE, YE, YE, W1, W1, W1, W1, YE, YE, W1, W1, YE, YE, YE, YE, ],
        vec![ YE, YE, YE, YE, W1, W1, YE, W1, W1, YE, W1, W1, YE, YE, YE, YE, ],
        vec![ YE, YE, YE, YE, W1, W1, YE, W1, W1, YE, W1, W1, YE, YE, YE, YE, ],
        vec![ YE, YE, YE, YE, W1, W1, YE, YE, W1, W1, W1, W1, YE, YE, YE, YE, ],
        vec![ YE, YE, YE, YE, W1, W1, YE, YE, YE, W1, W1, W1, YE, YE, YE, YE, ],
        vec![ B0, YE, YE, YE, W1, W1, YE, YE, YE, YE, W1, W1, YE, YE, YE, B0, ],
        vec![ B0, YE, YE, YE, W1, W1, YE, YE, YE, YE, W1, W1, YE, YE, YE, B0, ],
        vec![ B0, B0, YE, YE, W1, W1, YE, YE, YE, YE, W1, W1, YE, YE, B0, B0, ],
        vec![ B0, B0, B0, YE, YE, YE, YE, YE, YE, YE, YE, YE, YE, B0, B0, B0, ],
        vec![ B0, B0, B0, B0, B0, YE, YE, YE, YE, YE, YE, B0, B0, B0, B0, B0, ],
    ];
    pattern
}

fn q_train_bullet_pattern() -> BulbDisplay {
    let pattern = vec![
        vec![ B0, B0, B0, B0, B0, YE, YE, YE, YE, YE, YE, B0, B0, B0, B0, B0, ],
        vec![ B0, B0, B0, YE, YE, YE, W1, W1, W1, YE, YE, YE, YE, B0, B0, B0, ],
        vec![ B0, B0, YE, YE, YE, W1, W1, W1, W1, W1, YE, YE, YE, YE, B0, B0, ],
        vec![ B0, YE, YE, YE, W1, W1, YE, YE, YE, W1, W1, YE, YE, YE, YE, B0, ],
        vec![ B0, YE, YE, YE, W1, W1, YE, YE, YE, W1, W1, YE, YE, YE, YE, B0, ],
        vec![ YE, YE, YE, W1, W1, YE, YE, YE, YE, YE, W1, W1, YE, YE, YE, YE, ],
        vec![ YE, YE, YE, W1, W1, YE, YE, YE, YE, YE, W1, W1, YE, YE, YE, YE, ],
        vec![ YE, YE, YE, W1, W1, YE, YE, YE, YE, YE, W1, W1, YE, YE, YE, YE, ],
        vec![ YE, YE, YE, W1, W1, YE, YE, W1, YE, YE, W1, W1, YE, YE, YE, YE, ],
        vec![ YE, YE, YE, W1, W1, YE, YE, YE, W1, W1, W1, YE, YE, YE, YE, YE, ],
        vec![ YE, YE, YE, W1, W1, YE, YE, YE, YE, W1, W1, W1, YE, YE, YE, YE, ],
        vec![ B0, YE, YE, YE, W1, W1, YE, YE, YE, W1, W1, YE, YE, YE, YE, B0, ],
        vec![ B0, YE, YE, YE, W1, W1, YE, YE, YE, W1, W1, YE, YE, YE, YE, B0, ],
        vec![ B0, B0, YE, YE, YE, W1, W1, W1, W1, W1, YE, YE, YE, YE, B0, B0, ],
        vec![ B0, B0, B0, YE, YE, YE, W1, W1, W1, YE, YE, YE, YE, B0, B0, B0, ],
        vec![ B0, B0, B0, B0, B0, YE, YE, YE, YE, YE, YE, B0, B0, B0, B0, B0, ],
    ];
    pattern
}

fn r_train_bullet_pattern() -> BulbDisplay {
    let pattern = vec![
        vec![ B0, B0, B0, B0, B0, YE, YE, YE, YE, YE, YE, B0, B0, B0, B0, B0, ],
        vec![ B0, B0, B0, YE, YE, YE, YE, YE, YE, YE, YE, YE, YE, B0, B0, B0, ],
        vec![ B0, B0, YE, YE, W1, W1, W1, W1, W1, W1, YE, YE, YE, YE, B0, B0, ],
        vec![ B0, YE, YE, YE, W1, W1, W1, W1, W1, W1, W1, YE, YE, YE, YE, B0, ],
        vec![ B0, YE, YE, YE, W1, W1, YE, YE, YE, YE, W1, W1, YE, YE, YE, B0, ],
        vec![ YE, YE, YE, YE, W1, W1, YE, YE, YE, YE, W1, W1, YE, YE, YE, YE, ],
        vec![ YE, YE, YE, YE, W1, W1, W1, W1, W1, W1, W1, YE, YE, YE, YE, YE, ],
        vec![ YE, YE, YE, YE, W1, W1, W1, W1, W1, W1, YE, YE, YE, YE, YE, YE, ],
        vec![ YE, YE, YE, YE, W1, W1, YE, YE, W1, W1, YE, YE, YE, YE, YE, YE, ],
        vec![ YE, YE, YE, YE, W1, W1, YE, YE, W1, W1, YE, YE, YE, YE, YE, YE, ],
        vec![ YE, YE, YE, YE, W1, W1, YE, YE, YE, W1, W1, YE, YE, YE, YE, YE, ],
        vec![ B0, YE, YE, YE, W1, W1, YE, YE, YE, W1, W1, YE, YE, YE, YE, B0, ],
        vec![ B0, YE, YE, YE, W1, W1, YE, YE, YE, YE, W1, W1, YE, YE, YE, B0, ],
        vec![ B0, B0, YE, YE, W1, W1, YE, YE, YE, YE, W1, W1, YE, YE, B0, B0, ],
        vec![ B0, B0, B0, YE, YE, YE, YE, YE, YE, YE, YE, YE, YE, B0, B0, B0, ],
        vec![ B0, B0, B0, B0, B0, YE, YE, YE, YE, YE, YE, B0, B0, B0, B0, B0, ],
    ];
    pattern
}

fn s_train_bullet_pattern() -> BulbDisplay {
    let pattern = vec![
        vec![ B0, B0, B0, B0, B0, GY, GY, GY, GY, GY, GY, B0, B0, B0, B0, B0, ],
        vec![ B0, B0, B0, GY, GY, GY, W1, W1, W1, GY, GY, GY, GY, B0, B0, B0, ],
        vec![ B0, B0, GY, GY, GY, W1, W1, W1, W1, W1, GY, GY, GY, GY, B0, B0, ],
        vec![ B0, GY, GY, GY, W1, W1, GY, GY, GY, W1, W1, GY, GY, GY, GY, B0, ],
        vec![ B0, GY, GY, GY, W1, W1, GY, GY, GY, GY, GY, GY, GY, GY, GY, B0, ],
        vec![ GY, GY, GY, GY, W1, W1, GY, GY, GY, GY, GY, GY, GY, GY, GY, GY, ],
        vec![ GY, GY, GY, GY, W1, W1, W1, W1, W1, GY, GY, GY, GY, GY, GY, GY, ],
        vec![ GY, GY, GY, GY, GY, W1, W1, W1, W1, W1, GY, GY, GY, GY, GY, GY, ],
        vec![ GY, GY, GY, GY, GY, GY, GY, GY, GY, W1, W1, GY, GY, GY, GY, GY, ],
        vec![ GY, GY, GY, GY, GY, GY, GY, GY, GY, W1, W1, GY, GY, GY, GY, GY, ],
        vec![ GY, GY, GY, GY, GY, GY, GY, GY, GY, W1, W1, GY, GY, GY, GY, GY, ],
        vec![ B0, GY, GY, GY, GY, GY, GY, GY, GY, W1, W1, GY, GY, GY, GY, B0, ],
        vec![ B0, GY, GY, GY, W1, W1, GY, GY, GY, W1, W1, GY, GY, GY, GY, B0, ],
        vec![ B0, B0, GY, GY, GY, W1, W1, W1, W1, W1, GY, GY, GY, GY, B0, B0, ],
        vec![ B0, B0, B0, GY, GY, GY, W1, W1, W1, GY, GY, GY, GY, B0, B0, B0, ],
        vec![ B0, B0, B0, B0, B0, GY, GY, GY, GY, GY, GY, B0, B0, B0, B0, B0, ],
    ];
    pattern
}

fn w_train_bullet_pattern() -> BulbDisplay {
    let pattern = vec![
        vec![ B0, B0, B0, B0, B0, YE, YE, YE, YE, YE, YE, B0, B0, B0, B0, B0, ],
        vec![ B0, B0, B0, YE, YE, YE, YE, YE, YE, YE, YE, YE, YE, B0, B0, B0, ],
        vec![ B0, B0, YE, YE, YE, YE, YE, YE, YE, YE, YE, YE, YE, YE, B0, B0, ],
        vec![ B0, YE, W1, YE, YE, YE, YE, W1, YE, YE, YE, YE, W1, YE, YE, B0, ],
        vec![ B0, YE, W1, W1, YE, YE, W1, W1, W1, YE, YE, W1, W1, YE, YE, B0, ],
        vec![ YE, YE, W1, W1, YE, W1, W1, YE, W1, W1, YE, W1, W1, YE, YE, YE, ],
        vec![ YE, YE, YE, W1, YE, W1, W1, YE, W1, W1, YE, W1, YE, YE, YE, YE, ],
        vec![ YE, YE, YE, YE, W1, W1, YE, YE, YE, W1, W1, YE, YE, YE, YE, YE, ],
        vec![ YE, YE, YE, YE, W1, W1, YE, YE, YE, W1, W1, YE, YE, YE, YE, YE, ],
        vec![ YE, YE, YE, YE, W1, YE, YE, YE, YE, YE, W1, YE, YE, YE, YE, YE, ],
        vec![ YE, YE, YE, YE, YE, YE, YE, YE, YE, YE, YE, YE, YE, YE, YE, YE, ],
        vec![ B0, YE, YE, YE, YE, YE, YE, YE, YE, YE, YE, YE, YE, YE, YE, B0, ],
        vec![ B0, YE, YE, YE, YE, YE, YE, YE, YE, YE, YE, YE, YE, YE, YE, B0, ],
        vec![ B0, B0, YE, YE, YE, YE, YE, YE, YE, YE, YE, YE, YE, YE, B0, B0, ],
        vec![ B0, B0, B0, YE, YE, YE, YE, YE, YE, YE, YE, YE, YE, B0, B0, B0, ],
        vec![ B0, B0, B0, B0, B0, YE, YE, YE, YE, YE, YE, B0, B0, B0, B0, B0, ],
    ];
    pattern
}

fn z_train_bullet_pattern() -> BulbDisplay {
    let pattern = vec![
        vec![ B0, B0, B0, B0, B0, BR, BR, BR, BR, BR, BR, B0, B0, B0, B0, B0, ],
        vec![ B0, B0, B0, BR, BR, BR, BR, BR, BR, BR, BR, BR, BR, B0, B0, B0, ],
        vec![ B0, B0, BR, BR, W1, W1, W1, W1, W1, W1, W1, W1, BR, BR, B0, B0, ],
        vec![ B0, BR, BR, BR, W1, W1, W1, W1, W1, W1, W1, W1, BR, BR, BR, B0, ],
        vec![ B0, BR, BR, BR, BR, BR, BR, BR, BR, W1, W1, BR, BR, BR, BR, B0, ],
        vec![ BR, BR, BR, BR, BR, BR, BR, BR, W1, W1, BR, BR, BR, BR, BR, BR, ],
        vec![ BR, BR, BR, BR, BR, BR, BR, W1, W1, BR, BR, BR, BR, BR, BR, BR, ],
        vec![ BR, BR, BR, BR, BR, BR, W1, W1, BR, BR, BR, BR, BR, BR, BR, BR, ],
        vec![ BR, BR, BR, BR, BR, W1, W1, BR, BR, BR, BR, BR, BR, BR, BR, BR, ],
        vec![ BR, BR, BR, BR, W1, W1, BR, BR, BR, BR, BR, BR, BR, BR, BR, BR, ],
        vec![ BR, BR, BR, W1, W1, BR, BR, BR, BR, BR, BR, BR, BR, BR, BR, BR, ],
        vec![ B0, BR, BR, W1, W1, BR, BR, BR, BR, BR, BR, BR, BR, BR, BR, B0, ],
        vec![ B0, BR, BR, W1, W1, W1, W1, W1, W1, W1, W1, W1, BR, BR, BR, B0, ],
        vec![ B0, B0, BR, BR, W1, W1, W1, W1, W1, W1, W1, W1, BR, BR, B0, B0, ],
        vec![ B0, B0, B0, BR, BR, BR, BR, BR, BR, BR, BR, BR, BR, B0, B0, B0, ],
        vec![ B0, B0, B0, B0, B0, BR, BR, BR, BR, BR, BR, B0, B0, B0, B0, B0, ],
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
