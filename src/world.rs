use crate::block::render_block;
use crate::block::render_block_overlay;
use crate::block::Block;
use crate::block::BlockOverlay;
use crate::block::BlockType;
use crate::entity::Entity;
use crate::placer::Placer;
use crate::player::Player;
use crate::position::GridPos;
use crate::position::SubGridPos;
use crate::utils::hex_color;
use crate::vis::VisRange;
use crate::BLOCK_SIZE;
use macroquad::prelude::*;
use std::collections::HashMap;
pub struct World {
    pub blocks: HashMap<GridPos, Block>,
    pub entities: Vec<Box<dyn Entity>>,
    pub bg_color: Color,
    pub player: Player,
    pub placer: Placer,
    zoom: f32,
    view_offset_x: f32,
    view_offset_y: f32,
    camera: Camera2D,
}
impl World {
    pub fn new(
        blocks: HashMap<GridPos, Block>,
        entities: Vec<Box<dyn Entity>>,
        bg_color: Color,
    ) -> Self {
        let mut world = World {
            blocks,
            entities,
            bg_color,
            player: Player::new(SubGridPos { x: 1.0, y: 1.0 }),
            placer: Placer::new(),
            view_offset_x: 0.0,
            view_offset_y: 0.0,
            zoom: 4.0,
            camera: Camera2D {
                ..Default::default()
            },
        };
        world.player.spawn();
        return world;
    }
    pub fn update(&mut self) {
        self.player.update(&self.blocks);
        self.lerp_view_offset_to_player();
    }

    pub fn render(&mut self, vis_block_count: &mut i32) {
        clear_background(self.bg_color);
        self.set_camera_settings();
        set_camera(&self.camera);

        // calculate how much we can see on screen for culling
        let world_vis = self.calc_screen_vis();
        let mut blocks_to_render: Vec<(&GridPos, &Block)> = Vec::new();
        // get all blocks to be rendered
        for (pos, block) in &self.blocks {
            if block.block_type == BlockType::Solid {
                let scaled_pos = *pos * BLOCK_SIZE;
                // cull blocks that aren't visible to the camera
                if !world_vis.contains_coord(scaled_pos.x as f32, scaled_pos.y as f32) {
                    continue;
                }
                blocks_to_render.push((pos, block));
                *vis_block_count += 1;
            }
        }
        // these need to be rendered seperately to not fuck up draw calls(?)
        for (pos, block) in &blocks_to_render {
            render_block(**block, **pos);
        }
        for (pos, block) in &blocks_to_render {
            let block = **block;
            if block.overlay != BlockOverlay::None {
                render_block_overlay(block, **pos);
            }
        }

        // render all entities
        for entity in &mut self.entities {
            entity.render();
        }
        self.player.render();

        // update and render placer
        self.placer.update(&self.camera, &mut self.blocks);

        set_default_camera();
    }

    fn lerp_view_offset_to_player(&mut self) {
        let target_x = self.player.pos.x * BLOCK_SIZE;
        let target_y = self.player.pos.y * BLOCK_SIZE;
        self.view_offset_x += (target_x - self.view_offset_x) * 0.1;
        self.view_offset_y += (target_y - self.view_offset_y) * 0.02;
    }
    fn set_camera_settings(&mut self) {
        self.camera.zoom = vec2(
            self.zoom / screen_width(),
            (screen_width() / screen_height() * self.zoom) / screen_width(),
        );
        self.camera.target = vec2(self.view_offset_x.round(), self.view_offset_y.round());
    }
    fn calc_screen_vis(&self) -> VisRange {
        let min = self.camera.screen_to_world(Vec2::new(0.0, 0.0));
        let max = self
            .camera
            .screen_to_world(Vec2::new(screen_width(), screen_height()));
        // adding padding to avoid visible culling near screen sides
        return VisRange::new(
            min.x - BLOCK_SIZE,
            min.y - BLOCK_SIZE,
            max.x + BLOCK_SIZE,
            max.y + BLOCK_SIZE,
        );
    }
}

pub fn generate_test_world() -> World {
    let mut blocks: HashMap<GridPos, Block> = HashMap::new();
    for x in 0..48 {
        for y in 0..6 {
            blocks.insert(
                GridPos { x: x, y: y + 12 },
                Block {
                    block_type: BlockType::Solid,
                    color: hex_color("#1f3029", 1.0),
                    overlay: BlockOverlay::Grass,
                },
            );
        }
    }
    let blue_block = Block {
        block_type: BlockType::Solid,
        color: hex_color("#5184c3", 1.0),
        overlay: BlockOverlay::Box,
    };
    blocks.insert(GridPos { x: 5, y: 10 }, blue_block.clone());
    blocks.insert(GridPos { x: 5, y: 11 }, blue_block.clone());
    blocks.insert(GridPos { x: 9, y: 11 }, blue_block.clone());
    let mut world = World::new(blocks, Vec::new(), hex_color("#15171c", 1.0));
    world.player.spawn();
    return world;
}
