use bevy::render::color;

#[derive(Debug, Copy, Clone)]
pub enum Voxel {
    Empty,
    Solid(color::Color),
}

impl PartialEq for Voxel {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&Voxel::Empty, &Voxel::Empty) => true,
            (&Voxel::Solid(a), &Voxel::Solid(b)) => a == b,
            _ => false,
        }
    }
}
