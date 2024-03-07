use comfy::HashMap;
use macroquad::prelude::*;

use crate::{
    block::{render_block, render_block_overlay, Block, BlockOverlay, BlockType},
    position::GridPos,
    utils::hex_color,
    BLOCK_SIZE,
};
pub struct Placer {
    selected_block: Block,
    last_x: f32,
    last_y: f32,
}

impl Placer {
    pub fn new() -> Self {
        Placer {
            selected_block: Block::new(
                BlockType::Solid,
                hex_color("#E755FF", 1.0),
                BlockOverlay::Box,
            ),
            last_x: 0.0,
            last_y: 0.0,
        }
    }
    pub fn update(&mut self, camera: &Camera2D, blocks: &mut HashMap<GridPos, Block>) {
        // get mouse pos in world space
        let mouse_pos = mouse_position();
        let screen_mouse_pos = camera.screen_to_world(Vec2::new(mouse_pos.0, mouse_pos.1));

        // draw a block there
        let block_pos = screen_mouse_pos / BLOCK_SIZE;
        let block_grid_pos = GridPos::new(block_pos.x.floor() as i32, block_pos.y.floor() as i32);

        // we want to draw it transparently so clone the block
        let mut ghost_block = self.selected_block.clone();
        ghost_block.color.a = 0.5;
        render_block(ghost_block, block_grid_pos);
        render_block_overlay(ghost_block, block_grid_pos);
        // check for input to place
        if is_mouse_button_down(MouseButton::Left) {
            // todo: down? up?
            if block_grid_pos.x as f32 == self.last_x && block_grid_pos.y as f32 == self.last_y {
                return;
            }
            if blocks.contains_key(&block_grid_pos) {
                return;
            }
            blocks.insert(block_grid_pos, self.selected_block);
        }

        self.last_x = block_pos.x;
        self.last_y = block_pos.y;
    }
}
