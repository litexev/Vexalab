use crate::block::Block;
use crate::include_texture2d;
use crate::position::GridPos;
use crate::position::SubGridPos;
use crate::utils::aabb;
use crate::utils::clamp;
use crate::BLOCK_SIZE;
use macroquad::prelude::*;
use std::collections::HashMap;

const NEIGHBORS: [(i32, i32); 16] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (2, -1),
    (3, -1),
    (-1, 0),
    (3, 0),
    (-1, 1),
    (3, -1),
    (-1, 2),
    (3, 2),
    (-1, 3),
    (0, 3),
    (1, 3),
    (2, 3),
    (3, 3),
];
const GRAVITY: f32 = 0.012;

pub struct Player {
    pub pos: SubGridPos,
    vel_x: f32,
    vel_y: f32,
    grounded: bool,
    sprite: Option<Texture2D>,
    flip: bool,
}

impl Player {
    pub fn new(pos: SubGridPos) -> Self {
        let mut player = Player {
            pos,
            vel_x: 0.0,
            vel_y: 0.0,
            grounded: false,
            sprite: None,
            flip: false,
        };
        player.sprite = Some(include_texture2d!("./assets/player.png"));
        return player;
    }

    pub fn render(&self) {
        if let Some(sprite) = &self.sprite {
            let sprite_w = sprite.width();
            let sprite_h = sprite.height();
            draw_texture_ex(
                sprite,
                (self.pos.x * BLOCK_SIZE) - (sprite_w / 2.0) + (BLOCK_SIZE * (3.0 / 2.0)),
                ((self.pos.y) * BLOCK_SIZE) - sprite_h + (BLOCK_SIZE * 3.0),
                WHITE,
                DrawTextureParams {
                    flip_x: self.flip,
                    ..Default::default()
                },
            )
        }
        /*draw_rectangle(
            self.pos.x * BLOCK_SIZE,
            self.pos.y * BLOCK_SIZE,
            BLOCK_SIZE * 3.0,
            BLOCK_SIZE * 3.0,
            hex_color("#c34f51", 0.5),
        );*/
    }

    pub fn update(&mut self, blocks: &HashMap<GridPos, Block>) {
        // INPUT
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            self.flip = true;
            self.vel_x = -0.3;
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            self.flip = false;
            self.vel_x = 0.3;
        }
        if is_key_down(KeyCode::Space) && self.grounded {
            self.vel_y = -0.35;
        }

        // new position we'll try to move to
        let mut new_x = self.pos.x;
        let mut new_y = self.pos.y;
        // can we move in which axis?
        let mut blocked_x = false;
        let mut blocked_y = false;
        // if we can't move, what is blocking us?
        let mut x_blocker = GridPos {
            x: 0,
            y: 0,
            bg: false,
        };
        let mut y_blocker = GridPos {
            x: 0,
            y: 0,
            bg: false,
        };

        // slow existing velocity over time
        self.vel_x *= 1.0 - (0.2);
        self.vel_y = clamp(self.vel_y, -1.0, 1.0);

        // apply gravity
        self.vel_y += GRAVITY;

        // calculate new position
        new_x += self.vel_x;
        new_y += self.vel_y;

        // calculate fake grid position for the new pos
        let new_grid_x = new_x.round() as i32;
        let new_grid_y = new_y.round() as i32;

        for (x, y) in NEIGHBORS {
            let neighbor_grid_x = new_grid_x + x;
            let neighbor_grid_y = new_grid_y + y;

            if let Some(_) = blocks.get(&GridPos::new(neighbor_grid_x, neighbor_grid_y, false)) {
                if !blocked_x {
                    blocked_x = aabb(
                        new_x,
                        self.pos.y,
                        3.0,
                        3.0,
                        neighbor_grid_x as f32,
                        neighbor_grid_y as f32,
                        1.0,
                        1.0,
                    );
                    x_blocker = GridPos::new(neighbor_grid_x, neighbor_grid_y, false);
                }
                if !blocked_y {
                    blocked_y = aabb(
                        self.pos.x,
                        new_y,
                        3.0,
                        3.0,
                        neighbor_grid_x as f32,
                        neighbor_grid_y as f32,
                        1.0,
                        1.0,
                    );
                    y_blocker = GridPos::new(neighbor_grid_x, neighbor_grid_y, false);
                };
            }
        }

        if blocked_x {
            self.vel_x = 0.0;
            // snap to left/right of blocking block
            if self.pos.x < x_blocker.x as f32 {
                self.pos.x = x_blocker.x as f32 - 3.0; // left
            } else if self.pos.x > x_blocker.x as f32 {
                self.pos.x = x_blocker.x as f32 + 1.0; // right
            }
        } else {
            // apply position
            self.pos.x = new_x;
        }

        if blocked_y {
            self.vel_y = 0.0;
            // blocker debug
            // snap to top/bottom of blocking block
            if self.pos.y < y_blocker.y as f32 {
                self.pos.y = y_blocker.y as f32 - 3.0; // top
            } else {
                self.pos.y = y_blocker.y as f32 + 1.0; // bottom
            }
            // grounded check
            self.grounded = y_blocker.y as f32 > self.pos.y;
        } else {
            // apply position
            self.pos.y = new_y;
            self.grounded = false;
        }
    }

    pub fn get_vel(&self) -> (f32, f32) {
        return (self.vel_x, self.vel_y);
    }

    pub fn get_pos(&self) -> SubGridPos {
        return self.pos;
    }
}
