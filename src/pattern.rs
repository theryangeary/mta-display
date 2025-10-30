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
/// White
const W1: Rgb<u8> = Rgb([255, 255, 255]);
/// Black
const B0: Rgb<u8> = Rgb([0, 0, 0]);
/// ACE blue
const BL: Rgb<u8> = Rgb([0, 204, 255]);
/// 123 red
const RD: Rgb<u8> = Rgb([255, 0, 0]);

pub fn pattern_for_train(train: Train) -> BulbDisplay {
    match train {
        Train::One => one_train_bullet_pattern(),
        Train::Two => todo!(),
        Train::Three => todo!(),
        Train::Four => todo!(),
        Train::Five => todo!(),
        Train::Six => todo!(),
        Train::Seven => todo!(),
        Train::A => a_train_bullet_pattern(),
        Train::B => todo!(),
        Train::C => todo!(),
        Train::D => todo!(),
        Train::E => todo!(),
        Train::F => todo!(),
        Train::G => todo!(),
        Train::J => todo!(),
        Train::L => todo!(),
        Train::M => todo!(),
        Train::N => todo!(),
        Train::Q => todo!(),
        Train::R => todo!(),
        Train::S => todo!(),
        Train::Z => todo!(),
    }
}

fn one_train_bullet_pattern() -> BulbDisplay {
    let pattern = vec![
        vec![ B0, B0, B0, B0, B0, RD, RD, W1, W1, RD, RD, B0, B0, B0, B0, B0, ],
        vec![ B0, B0, B0, RD, RD, RD, W1, W1, W1, RD, RD, RD, RD, B0, B0, B0, ],
        vec![ B0, B0, RD, RD, RD, W1, W1, W1, W1, RD, RD, RD, RD, RD, B0, B0, ],
        vec![ B0, RD, RD, RD, W1, W1, RD, W1, W1, RD, RD, RD, RD, RD, RD, B0, ],
        vec![ B0, RD, RD, RD, RD, RD, RD, W1, W1, RD, RD, RD, RD, RD, RD, B0, ],
        vec![ RD, RD, RD, RD, RD, RD, RD, W1, W1, RD, RD, RD, RD, RD, RD, RD, ],
        vec![ RD, RD, RD, RD, RD, RD, RD, W1, W1, RD, RD, RD, RD, RD, RD, RD, ],
        vec![ RD, RD, RD, RD, RD, RD, RD, W1, W1, RD, RD, RD, RD, RD, RD, RD, ],
        vec![ RD, RD, RD, RD, RD, RD, RD, W1, W1, RD, RD, RD, RD, RD, RD, RD, ],
        vec![ RD, RD, RD, RD, RD, RD, RD, W1, W1, RD, RD, RD, RD, RD, RD, RD, ],
        vec![ RD, RD, RD, RD, RD, RD, RD, W1, W1, RD, RD, RD, RD, RD, RD, RD, ],
        vec![ B0, RD, RD, RD, RD, RD, RD, W1, W1, RD, RD, RD, RD, RD, RD, B0, ],
        vec![ B0, RD, RD, RD, RD, RD, RD, W1, W1, RD, RD, RD, RD, RD, RD, B0, ],
        vec![ B0, B0, RD, RD, RD, RD, RD, W1, W1, RD, RD, RD, RD, RD, B0, B0, ],
        vec![ B0, B0, B0, RD, RD, RD, RD, W1, W1, RD, RD, RD, RD, B0, B0, B0, ],
        vec![ B0, B0, B0, B0, B0, RD, RD, W1, W1, RD, RD, B0, B0, B0, B0, B0, ],
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
