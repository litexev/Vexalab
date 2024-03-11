use crate::world::{generate_test_world, World};

pub struct Game {
    pub world: World,
}

impl Game {
    pub fn new() -> Self {
        Game {
            world: generate_test_world(),
        }
    }

    pub fn update(&mut self) {
        self.world.update();
    }
    pub fn render(&mut self) {
        self.world.render();
    }
}
