use bevy::prelude::*;

pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(WindowDescriptor {
            title: "zVille".to_string(),
            width: 2880,
            height: 1800,
            ..Default::default()
        })
        .add_resource(Msaa { samples: 4 });
    }
}
