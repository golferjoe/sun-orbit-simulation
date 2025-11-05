use bevy::math::DVec2;

#[derive(Clone, Copy, PartialEq)]
pub struct DRect {
    pub min: DVec2, // top-left
    pub max: DVec2, // bottom-right
}

impl DRect {
    pub fn new(min: DVec2, max: DVec2) -> Self {
        Self { min, max }
    }

    pub fn contains(&self, p: DVec2) -> bool {
        p.x >= self.min.x && p.x < self.max.x && p.y >= self.max.y && p.y < self.min.y
    }
}
