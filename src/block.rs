use crate::cache::get_texture;
use crate::position::GridPos;
use crate::BLOCK_SIZE;
use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub struct Block {
    pub block_type: BlockType,
    pub color: Color,
    pub overlay: BlockOverlay,
}
impl Block {
    pub fn new(block_type: BlockType, color: Color, overlay: BlockOverlay) -> Self {
        Block {
            block_type,
            color,
            overlay,
        }
    }
}
#[derive(PartialEq, Clone, Copy)]
pub enum BlockType {
    Solid,
}

#[derive(PartialEq, Clone, Copy)]
pub enum BlockOverlay {
    None,
    Top,
}

pub fn get_overlay_texture(overlay: BlockOverlay) -> Texture2D {
    match overlay {
        BlockOverlay::Top => get_texture("./assets/overlays/top.png"),
        _ => panic!("get_overlay_texture called on none overlay"),
    }
}
pub fn render_block(block: Block, pos: GridPos) {
    let scaled_pos = pos * BLOCK_SIZE;
    draw_rectangle(
        scaled_pos.x as f32,
        scaled_pos.y as f32,
        BLOCK_SIZE,
        BLOCK_SIZE,
        block.color,
    );
}

pub fn render_block_overlay(block: Block, pos: GridPos) {
    let scaled_pos = pos * BLOCK_SIZE;
    draw_texture(
        &get_overlay_texture(block.overlay),
        scaled_pos.x as f32,
        scaled_pos.y as f32,
        Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: block.color.a,
        },
    )
}
