use crate::game::Game;
use crate::utils::draw_text_ex_shadow;
use crate::BUILD_VERSION;
use macroquad::prelude::*;

pub fn draw_debug_text(game: &Game, font: macroquad::text::Font) {
    let mut messages: Vec<String> = Vec::new();

    messages.push(format!("Vexalab {}", BUILD_VERSION));
    messages.push(format!("FPS: {}", get_fps()));

    let player_pos = game.world.player.get_pos();
    messages.push(format!(
        "player pos - x: {:.3}, y: {:.3}",
        player_pos.x, player_pos.y
    ));

    let player_vel = game.world.player.get_vel();
    messages.push(format!(
        "player vel - x: {:.3}, y: {:.3}",
        player_vel.0, player_vel.1
    ));

    for (index, message) in messages.iter().enumerate() {
        draw_text_ex_shadow(
            &message,
            10.0,
            20.0 * (index + 1) as f32,
            macroquad::text::TextParams {
                font: Some(&font),
                font_size: 16,
                color: WHITE,
                ..Default::default()
            },
        );
    }
}
