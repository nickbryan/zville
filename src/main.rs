use bevy::prelude::*;

mod cursor;
mod window;

fn main() {
    App::build()
        .add_plugin(window::WindowPlugin)
        .add_default_plugins()
        .add_plugin(cursor::CursorPlugin)
        .add_system(quit_system.system())
        .run();
}

fn quit_system(
    input: Res<Input<KeyCode>>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        app_exit_events.send(bevy::app::AppExit);
    }
}
