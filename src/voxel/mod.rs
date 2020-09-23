mod matrix;
mod qb;
mod vox;

pub use matrix::*;
pub use vox::*;

use bevy::prelude::{Plugin as BevyPlugin, *};
use qb::QubicleBinaryLoader;

#[derive(Default)]
pub struct Plugin;

impl BevyPlugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<Matrix>()
            .add_asset_loader::<Matrix, QubicleBinaryLoader>();
    }
}
