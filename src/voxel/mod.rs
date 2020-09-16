use bevy::prelude::*;

mod matrix;
mod qb;

pub use matrix::*;

#[derive(Default)]
pub struct VoxelPlugin;

impl Plugin for VoxelPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<matrix::Matrix>()
            .add_asset_loader::<matrix::Matrix, qb::QubicleBinaryLoader>();
    }
}
