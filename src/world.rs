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
    pub bg_color: Color,
    pub entities: Vec<Box<dyn Entity>>,
    pub player: Player,
    pub placer: Placer,
    zoom: f32,
    view_offset_x: f32,
    view_offset_y: f32,
    camera: Camera2D,
    sky_material: Material,
    sky_top_color: Color,
    sky_bottom_color: Color,
}

impl World {
    pub fn new(
        blocks: HashMap<GridPos, Block>,
        entities: Vec<Box<dyn Entity>>,
        bg_color: Color,
    ) -> Self {
        // Load sky shader
        let sky_material = load_sky_shader().unwrap();

        let world = World {
            blocks,
            bg_color,
            entities,
            player: Player::new(SubGridPos { x: 28.0, y: 1.0 }),
            placer: Placer::new(),
            view_offset_x: 0.0,
            view_offset_y: 0.0,
            zoom: 6.0,
            camera: Camera2D {
                ..Default::default()
            },
            sky_material,
            sky_top_color: hex_color("#0b0108", 1.0),
            sky_bottom_color: hex_color("#1b1f27", 0.0),
        };

        return world;
    }

    pub fn update(&mut self) {
        self.player.update(&self.blocks);
    }

    pub fn render(&mut self) {
        clear_background(self.bg_color);
        self.draw_sky();
        self.set_camera_settings();

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

        self.player.render();

        // update and render placer
        self.placer.update(&self.camera, &mut self.blocks);

        set_default_camera();
        self.placer.render_hud();
    }

    fn set_camera_settings(&mut self) {
        let target_x = self.player.pos.x * BLOCK_SIZE;
        let target_y = self.player.pos.y * BLOCK_SIZE;
        self.view_offset_x += (target_x - self.view_offset_x) * 0.1;
        self.view_offset_y += (target_y - self.view_offset_y) * 0.02;
        self.camera.zoom = vec2(
            self.zoom / screen_width(),
            (screen_width() / screen_height() * self.zoom) / screen_width(),
        );
        self.camera.target = vec2(self.view_offset_x, self.view_offset_y);
        set_camera(&self.camera);
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

    fn draw_sky(&mut self) {
        let mat = &self.sky_material;
        gl_use_material(mat);
        mat.set_uniform("canvasSize", (screen_width(), screen_height()));
        mat.set_uniform("startColor", self.sky_top_color);
        mat.set_uniform("endColor", self.sky_bottom_color);
        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), WHITE);
        gl_use_default_material();
    }
}

pub fn generate_test_world() -> World {
    let mut blocks: HashMap<GridPos, Block> = HashMap::new();

    // generate grass
    for x in 0..64 {
        for y in 0..15 {
            let mut overlay = BlockOverlay::None;
            if y == 0 {
                overlay = BlockOverlay::None;
            }
            blocks.insert(
                GridPos::new(x, y + 12, false),
                Block {
                    block_type: BlockType::Solid,
                    color: hex_color("#1f3029", 1.0),
                    overlay: overlay,
                },
            );
        }
    }

    // place blue blocks
    let blue_block = Block::new(
        BlockType::Solid,
        hex_color("#5184c3", 1.0),
        BlockOverlay::None,
    );
    blocks.insert(GridPos::new(32, 10, false), blue_block.clone());
    blocks.insert(GridPos::new(32, 11, false), blue_block.clone());
    blocks.insert(GridPos::new(36, 11, false), blue_block.clone());

    return World::new(blocks, Vec::new(), hex_color("#15171c", 1.0));
}

pub fn load_sky_shader() -> Result<macroquad::material::Material, macroquad::Error> {
    return load_material(
        ShaderSource::Glsl {
            vertex: &include_str!("./assets/shaders/sky.vert"),
            fragment: &include_str!("./assets/shaders/sky.frag"),
        },
        MaterialParams {
            uniforms: vec![
                ("canvasSize".to_owned(), UniformType::Float2),
                ("startColor".to_owned(), UniformType::Float4),
                ("endColor".to_owned(), UniformType::Float4),
            ],
            ..Default::default()
        },
    );
}
