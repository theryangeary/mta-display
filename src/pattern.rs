use image::Rgb;

pub const LETTER_PATTERN_WIDTH: u16 = 8;
#[allow(dead_code)]
pub const LETTER_PATTERN_HEIGHT: u16 = 16;
pub const LETTER_PATTERN_SPACING: u16 = 2;
pub const LETTER_PATTERN_SLOT_WIDTH: u16 = LETTER_PATTERN_WIDTH + LETTER_PATTERN_SPACING;
pub const TRAIN_BULLET_PATTERN_WIDTH: u16 = 16;
#[allow(dead_code)]
pub const TRAIN_BULLET_PATTERN_HEIGHT: u16 = 16;


// short names for colors used in patterns, makes patterns easier to read as they becomes somewhat 1:1 aspect ratio
/// White
const W1: Rgb<u8> = Rgb([255, 255, 255]);
/// Black
const B0: Rgb<u8> = Rgb([0, 0, 0]);
/// ACE blue
const BL: Rgb<u8> = Rgb([0, 204, 255]);

pub fn train_bullet_pattern() -> Vec<Vec<Rgb<u8>>> {
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

pub fn pattern_for_letter(letter: char) -> Vec<Vec<Rgb<u8>>> {
    match letter {
        ' ' => space_pattern(),
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
        _ => space_pattern(),
    }
}

fn space_pattern() -> Vec<Vec<Rgb<u8>>> {
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

fn letter_a_pattern() -> Vec<Vec<Rgb<u8>>> {
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

fn letter_b_pattern() -> Vec<Vec<Rgb<u8>>> {
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

fn letter_c_pattern() -> Vec<Vec<Rgb<u8>>> {
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

fn letter_d_pattern() -> Vec<Vec<Rgb<u8>>> {
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

fn letter_e_pattern() -> Vec<Vec<Rgb<u8>>> {
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

fn letter_f_pattern() -> Vec<Vec<Rgb<u8>>> {
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

fn letter_g_pattern() -> Vec<Vec<Rgb<u8>>> {
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

fn letter_h_pattern() -> Vec<Vec<Rgb<u8>>> {
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

fn letter_i_pattern() -> Vec<Vec<Rgb<u8>>> {
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

fn letter_j_pattern() -> Vec<Vec<Rgb<u8>>> {
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

fn letter_k_pattern() -> Vec<Vec<Rgb<u8>>> {
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

fn letter_l_pattern() -> Vec<Vec<Rgb<u8>>> {
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

fn letter_m_pattern() -> Vec<Vec<Rgb<u8>>> {
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

fn letter_n_pattern() -> Vec<Vec<Rgb<u8>>> {
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

fn letter_o_pattern() -> Vec<Vec<Rgb<u8>>> {
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

fn letter_p_pattern() -> Vec<Vec<Rgb<u8>>> {
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

fn letter_q_pattern() -> Vec<Vec<Rgb<u8>>> {
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

fn letter_r_pattern() -> Vec<Vec<Rgb<u8>>> {
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

fn letter_s_pattern() -> Vec<Vec<Rgb<u8>>> {
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

fn letter_t_pattern() -> Vec<Vec<Rgb<u8>>> {
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

fn letter_u_pattern() -> Vec<Vec<Rgb<u8>>> {
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

fn letter_v_pattern() -> Vec<Vec<Rgb<u8>>> {
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

fn letter_w_pattern() -> Vec<Vec<Rgb<u8>>> {
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

fn letter_x_pattern() -> Vec<Vec<Rgb<u8>>> {
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

fn letter_y_pattern() -> Vec<Vec<Rgb<u8>>> {
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

fn letter_z_pattern() -> Vec<Vec<Rgb<u8>>> {
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
