use image::Rgb;
#[allow(non_snake_case)]
pub(crate) fn train_bullet_pattern() -> Vec<Vec<Rgb<u8>>> {
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

// Patterns for letters A-Z and space
// TODO check if they actually match MTA display fonts

pub(crate) fn space_pattern() -> Vec<Vec<Rgb<u8>>> {
    let B0 = Rgb([0, 0, 0]);
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

pub(crate) fn letter_a_pattern() -> Vec<Vec<Rgb<u8>>> {
    let W1 = Rgb([255, 255, 255]);
    let B0 = Rgb([0, 0, 0]);
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

pub(crate) fn letter_b_pattern() -> Vec<Vec<Rgb<u8>>> {
    let W1 = Rgb([255, 255, 255]);
    let B0 = Rgb([0, 0, 0]);
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

pub(crate) fn letter_c_pattern() -> Vec<Vec<Rgb<u8>>> {
    let W1 = Rgb([255, 255, 255]);
    let B0 = Rgb([0, 0, 0]);
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

pub(crate) fn letter_d_pattern() -> Vec<Vec<Rgb<u8>>> {
    let W1 = Rgb([255, 255, 255]);
    let B0 = Rgb([0, 0, 0]);
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

pub(crate) fn letter_e_pattern() -> Vec<Vec<Rgb<u8>>> {
    let W1 = Rgb([255, 255, 255]);
    let B0 = Rgb([0, 0, 0]);
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

pub(crate) fn letter_f_pattern() -> Vec<Vec<Rgb<u8>>> {
    let W1 = Rgb([255, 255, 255]);
    let B0 = Rgb([0, 0, 0]);
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

pub(crate) fn letter_g_pattern() -> Vec<Vec<Rgb<u8>>> {
    let W1 = Rgb([255, 255, 255]);
    let B0 = Rgb([0, 0, 0]);
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

pub(crate) fn letter_h_pattern() -> Vec<Vec<Rgb<u8>>> {
    let W1 = Rgb([255, 255, 255]);
    let B0 = Rgb([0, 0, 0]);
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

pub(crate) fn letter_i_pattern() -> Vec<Vec<Rgb<u8>>> {
    let W1 = Rgb([255, 255, 255]);
    let B0 = Rgb([0, 0, 0]);
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

pub(crate) fn letter_j_pattern() -> Vec<Vec<Rgb<u8>>> {
    let W1 = Rgb([255, 255, 255]);
    let B0 = Rgb([0, 0, 0]);
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

pub(crate) fn letter_k_pattern() -> Vec<Vec<Rgb<u8>>> {
    let W1 = Rgb([255, 255, 255]);
    let B0 = Rgb([0, 0, 0]);
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

pub(crate) fn letter_l_pattern() -> Vec<Vec<Rgb<u8>>> {
    let W1 = Rgb([255, 255, 255]);
    let B0 = Rgb([0, 0, 0]);
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

pub(crate) fn letter_m_pattern() -> Vec<Vec<Rgb<u8>>> {
    let W1 = Rgb([255, 255, 255]);
    let B0 = Rgb([0, 0, 0]);
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

pub(crate) fn letter_n_pattern() -> Vec<Vec<Rgb<u8>>> {
    let W1 = Rgb([255, 255, 255]);
    let B0 = Rgb([0, 0, 0]);
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

pub(crate) fn letter_o_pattern() -> Vec<Vec<Rgb<u8>>> {
    let W1 = Rgb([255, 255, 255]);
    let B0 = Rgb([0, 0, 0]);
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

pub(crate) fn letter_p_pattern() -> Vec<Vec<Rgb<u8>>> {
    let W1 = Rgb([255, 255, 255]);
    let B0 = Rgb([0, 0, 0]);
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

pub(crate) fn letter_q_pattern() -> Vec<Vec<Rgb<u8>>> {
    let W1 = Rgb([255, 255, 255]);
    let B0 = Rgb([0, 0, 0]);
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

pub(crate) fn letter_r_pattern() -> Vec<Vec<Rgb<u8>>> {
    let W1 = Rgb([255, 255, 255]);
    let B0 = Rgb([0, 0, 0]);
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

pub(crate) fn letter_s_pattern() -> Vec<Vec<Rgb<u8>>> {
    let W1 = Rgb([255, 255, 255]);
    let B0 = Rgb([0, 0, 0]);
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

pub(crate) fn letter_t_pattern() -> Vec<Vec<Rgb<u8>>> {
    let W1 = Rgb([255, 255, 255]);
    let B0 = Rgb([0, 0, 0]);
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

pub(crate) fn letter_u_pattern() -> Vec<Vec<Rgb<u8>>> {
    let W1 = Rgb([255, 255, 255]);
    let B0 = Rgb([0, 0, 0]);
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

pub(crate) fn letter_v_pattern() -> Vec<Vec<Rgb<u8>>> {
    let W1 = Rgb([255, 255, 255]);
    let B0 = Rgb([0, 0, 0]);
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

pub(crate) fn letter_w_pattern() -> Vec<Vec<Rgb<u8>>> {
    let W1 = Rgb([255, 255, 255]);
    let B0 = Rgb([0, 0, 0]);
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

pub(crate) fn letter_x_pattern() -> Vec<Vec<Rgb<u8>>> {
    let W1 = Rgb([255, 255, 255]);
    let B0 = Rgb([0, 0, 0]);
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

pub(crate) fn letter_y_pattern() -> Vec<Vec<Rgb<u8>>> {
    let W1 = Rgb([255, 255, 255]);
    let B0 = Rgb([0, 0, 0]);
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

pub(crate) fn letter_z_pattern() -> Vec<Vec<Rgb<u8>>> {
    let W1 = Rgb([255, 255, 255]);
    let B0 = Rgb([0, 0, 0]);
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
