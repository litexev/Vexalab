use macroquad::prelude::*;

pub fn draw_text_ex_shadow(text: &str, x: f32, y: f32, params: TextParams) {
    let mut shadow_params = params.clone();
    shadow_params.color = Color::new(0.0, 0.0, 0.0, 255.0);
    draw_text_ex(text, x + 1.0, y + 1.0, shadow_params);
    draw_text_ex(text, x, y, params);
}

pub fn aabb(x1: f32, y1: f32, w1: f32, h1: f32, x2: f32, y2: f32, w2: f32, h2: f32) -> bool {
    x1 < x2 + w2 && x1 + w1 > x2 && y1 < y2 + h2 && y1 + h1 > y2
}

pub fn clamp(val: f32, min: f32, max: f32) -> f32 {
    return val.min(max).max(min);
}

pub fn hex_color(hex: &str, alpha: f32) -> Color {
    if hex.len() != 7 || !hex.starts_with("#") {
        panic!("Invalid hex color format");
    }

    let hex_digits = &hex[1..];
    let r_str = &hex_digits[0..2];
    let g_str = &hex_digits[2..4];
    let b_str = &hex_digits[4..6];

    let r = u8::from_str_radix(r_str, 16).expect("Invalid hex digit for red");
    let g = u8::from_str_radix(g_str, 16).expect("Invalid hex digit for green");
    let b = u8::from_str_radix(b_str, 16).expect("Invalid hex digit for blue");

    let r_normalized = r as f32 / 255.0;
    let g_normalized = g as f32 / 255.0;
    let b_normalized = b as f32 / 255.0;

    return Color::new(r_normalized, g_normalized, b_normalized, alpha);
}

#[macro_export]
macro_rules! load_pixel_texture {
    ($path:expr) => {{
        let sprite =
            Texture2D::from_file_with_format(include_bytes!($path), Some(ImageFormat::Png));
        sprite.set_filter(FilterMode::Nearest);
        sprite
    }};
}
