use crate::world::{generate_test_world, World};

pub struct Game {
    pub world: World,
    pub debug_vis_block_count: i32,
}

impl Game {
    pub fn new() -> Self {
        Game {
            world: generate_test_world(),
            debug_vis_block_count: 0,
        }
    }
    pub fn update(&mut self) {
        self.debug_vis_block_count = 0;
        self.world.update();
        self.world.render(&mut self.debug_vis_block_count);
    }
}
