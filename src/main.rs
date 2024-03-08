use macroquad::prelude::*;

mod block;
mod cache;
mod debug;
mod entity;
mod game;
mod placer;
mod player;
mod position;
mod utils;
mod vis;
mod world;
use debug::draw_debug_text;
use game::Game;

const BUILD_VERSION: &str = "PROTO3";
const BLOCK_SIZE: f32 = 8.0;

fn macroquad_conf() -> Conf {
    Conf {
        window_title: format!("Vexalab {}", BUILD_VERSION),
        high_dpi: true,
        window_width: 900,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(macroquad_conf)]
async fn main() {
    let font = load_ttf_font_from_bytes(include_bytes!("assets/consolas.ttf")).unwrap();
    let mut game = Game::new();
    loop {
        clear_background(BLACK);
        game.update();
        draw_debug_text(&game, font.clone());
        next_frame().await
    }
}
