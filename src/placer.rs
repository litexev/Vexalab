use crate::{
    block::{render_block, render_block_overlay, Block, BlockOverlay, BlockType},
    position::GridPos,
    BLOCK_SIZE,
};
use comfy::HashMap;
use hsl::HSL;
use macroquad::prelude::*;

pub struct Placer {
    last_x: f32,
    last_y: f32,
    color: f64,
    brightness: f64,
    overlay: BlockOverlay,
}

const COLOR_HUD_WIDTH: f32 = 5.0;

impl Placer {
    pub fn new() -> Self {
        Placer {
            last_x: 0.0,
            last_y: 0.0,
            color: 0.0,
            brightness: 0.67,
            overlay: BlockOverlay::None,
        }
    }
    pub fn calc_hsl_color(&self) -> Color {
        let hsl_color = HSL {
            h: self.color,
            s: self.brightness,
            l: self.brightness,
        }
        .to_rgb();
        return Color::from_rgba(hsl_color.0, hsl_color.1, hsl_color.2, 255);
    }
    pub fn update(&mut self, camera: &Camera2D, blocks: &mut HashMap<GridPos, Block>) {
        self.placer_input_update();

        // get mouse pos in world space
        let mouse_pos = mouse_position();
        let screen_mouse_pos = camera.screen_to_world(Vec2::new(mouse_pos.0, mouse_pos.1));

        // draw a block there
        let block_pos = screen_mouse_pos / BLOCK_SIZE;
        let block_grid_pos = GridPos::new(block_pos.x.floor() as i32, block_pos.y.floor() as i32);
        if blocks.contains_key(&block_grid_pos) {
            self.select_loop(block_pos, block_grid_pos, blocks);
        } else {
            self.place_loop(block_pos, block_grid_pos, blocks);
        }
        self.last_x = block_pos.x;
        self.last_y = block_pos.y;
    }
    fn select_loop(
        &mut self,
        _: Vec2,
        block_grid_pos: GridPos,
        blocks: &mut HashMap<GridPos, Block>,
    ) {
        // let block = blocks.get(&block_grid_pos).unwrap();

        let screen_grid_x = block_grid_pos.x as f32 * BLOCK_SIZE;
        let screen_grid_y = block_grid_pos.y as f32 * BLOCK_SIZE;
        draw_rectangle_lines(
            screen_grid_x,
            screen_grid_y,
            BLOCK_SIZE,
            BLOCK_SIZE,
            1.0,
            WHITE,
        );

        if is_mouse_button_down(MouseButton::Right) {
            if block_grid_pos.x as f32 == self.last_x && block_grid_pos.y as f32 == self.last_y {
                return;
            }
            blocks.remove(&block_grid_pos);
        }
    }
    fn place_loop(
        &mut self,
        _: Vec2,
        block_grid_pos: GridPos,
        blocks: &mut HashMap<GridPos, Block>,
    ) {
        // calculate the correct color using hsl
        let color = self.calc_hsl_color();

        // we want to draw it transparently so clone the block
        let mut block = Block::new(BlockType::Solid, color, self.overlay);
        block.color.a = 0.5;
        render_block(block, block_grid_pos);
        if self.overlay != BlockOverlay::None {
            render_block_overlay(block, block_grid_pos);
        }
        block.color.a = 1.0;

        // check for input to place
        if is_mouse_button_down(MouseButton::Left) {
            // todo: down? up?
            if block_grid_pos.x as f32 == self.last_x && block_grid_pos.y as f32 == self.last_y {
                return;
            }
            /*if blocks.contains_key(&block_grid_pos) {
                return;
            }*/
            blocks.insert(block_grid_pos, block.clone());
        }
    }
    fn placer_input_update(&mut self) {
        let (_, mouse_wheel_y) = mouse_wheel();
        let color_modifier = is_key_down(KeyCode::LeftControl);
        let bright_modifier = is_key_down(KeyCode::LeftAlt);

        if mouse_wheel_y != 0.0 {
            if color_modifier {
                self.color += 20.0 * -mouse_wheel_y.signum() as f64;
            }

            if bright_modifier {
                self.brightness += 0.025 * -mouse_wheel_y.signum() as f64;
            }
        }

        self.color = (self.color + 360.0) % 360.0;
        self.brightness = clamp(self.brightness, 0.0, 1.0);
    }
    pub fn render_hud(&self) {
        if is_key_down(KeyCode::LeftControl) {
            self.render_color_hud();
        } else if is_key_down(KeyCode::LeftAlt) {
            self.render_brightness_hud();
        }
    }
    pub fn render_color_hud(&self) {
        let estimated_size = COLOR_HUD_WIDTH * 36.0;
        let start_x = screen_width() / 2.0 - estimated_size / 2.0;

        for i in 0..36 {
            let hsl_color = HSL {
                h: (i * 10) as f64,
                s: 0.67,
                l: 0.67,
            }
            .to_rgb();
            let color = Color::from_rgba(hsl_color.0, hsl_color.1, hsl_color.2, 255);
            draw_rectangle(
                start_x + ((i as f32 * COLOR_HUD_WIDTH) as f32),
                8.0,
                COLOR_HUD_WIDTH,
                16.0,
                color,
            );
        }

        // preview rectangle
        let color = self.calc_hsl_color();
        draw_rectangle(start_x as f32, 32.0, estimated_size, 16.0, color);

        // draw selected color
        let estimated_pos = start_x + ((self.color as f32 / 10.0) * COLOR_HUD_WIDTH);
        draw_rectangle(
            estimated_pos,
            8.0,
            COLOR_HUD_WIDTH,
            16.0,
            Color::new(1.0, 1.0, 1.0, 0.5),
        );
    }

    pub fn render_brightness_hud(&self) {
        let estimated_size = COLOR_HUD_WIDTH * 36.0;
        let start_x = screen_width() / 2.0 - estimated_size / 2.0;

        for i in 0..36 {
            let hsl_color = HSL {
                h: self.color,
                s: (i as f64 / 35.0),
                l: (i as f64 / 35.0),
            }
            .to_rgb();
            let color = Color::from_rgba(hsl_color.0, hsl_color.1, hsl_color.2, 255);
            draw_rectangle(
                start_x + ((i as f32 * COLOR_HUD_WIDTH) as f32),
                8.0,
                COLOR_HUD_WIDTH,
                16.0,
                color,
            );
        }

        // preview rectangle
        let color = self.calc_hsl_color();
        draw_rectangle(start_x as f32, 32.0, estimated_size, 16.0, color);

        // draw selected color
        let estimated_pos = start_x + ((self.brightness as f32 * 36.0) * COLOR_HUD_WIDTH);
        draw_rectangle(
            estimated_pos,
            8.0,
            COLOR_HUD_WIDTH,
            16.0,
            Color::new(1.0, 1.0, 1.0, 0.5),
        );
    }
}
