pub struct VisRange {
    pub min_x: f32,
    pub min_y: f32,
    pub max_x: f32,
    pub max_y: f32,
}

impl VisRange {
    pub fn new(min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> Self {
        VisRange {
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }
    /*pub fn new_from_rect(x: f32, y: f32, w: f32, h: f32) -> Self {
        VisRange::new(x, y, x + w, y + h)
    }
    pub fn overlaps(&self, other: &VisRange) -> bool {
        self.min_x <= other.max_x
            && self.max_x >= other.min_x
            && self.min_y <= other.max_y
            && self.max_y >= other.min_y
    }*/
    pub fn contains_coord(&self, x: f32, y: f32) -> bool {
        self.min_x <= x && self.max_x >= x && self.min_y <= y && self.max_y >= y
    }
    /*pub fn contains_x(&self, x: f32) -> bool {
        self.min_x <= x && self.max_x >= x
    }
    pub fn contains_y(&self, y: f32) -> bool {
        self.min_y <= y && self.max_y >= y
    }*/
}
