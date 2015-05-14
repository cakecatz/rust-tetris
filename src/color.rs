pub fn get_color(color_name: &str) -> [f32; 4] {
    return match color_name {
        "black" => [0.0, 0.0, 0.0, 1.0],
        _ => [0.0, 0.0, 0.0, 0.0],
    };
}

pub fn hex_color(color_code: &str) -> [f32; 4] {
    let mut rgb_chars = color_code.chars();
    let r: f32 = (hex_to_dec( rgb_chars.nth(0).unwrap() ) * 16
        + hex_to_dec( rgb_chars.nth(0).unwrap() ) ) as f32 / 255f32;
    let g: f32 = (hex_to_dec( rgb_chars.nth(0).unwrap() ) * 16
        + hex_to_dec( rgb_chars.nth(0).unwrap() ) ) as f32 / 255f32;
    let b: f32 = (hex_to_dec( rgb_chars.nth(0).unwrap() ) * 16
        + hex_to_dec( rgb_chars.nth(0).unwrap() ) ) as f32 / 255f32;
    return [r, g, b, 1.0];
}

pub fn hex_to_dec(c: char) -> i32 {
    return match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'a' => 10,
        'b' => 11,
        'c' => 12,
        'd' => 13,
        'e' => 14,
        'f' => 15,
        _ => 0,
    };
}