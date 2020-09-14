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
        .add_resource(Msaa { samples: 4 })
        .add_event::<LockCursor>()
        .init_resource::<LockCursorState>()
        .add_system(lock_cursor_command_handler_system.system());
    }
}

pub struct LockCursor(pub bevy::window::WindowId);

#[derive(Default)]
struct LockCursorState {
    lock_cursor_command_reader: EventReader<LockCursor>,
}

fn lock_cursor_command_handler_system(
    mut state: ResMut<LockCursorState>,
    commands: Res<Events<LockCursor>>,
    windows: Res<bevy::winit::WinitWindows>,
) {
    if let Some(command) = state.lock_cursor_command_reader.latest(&commands) {
        let window = windows.get_window(command.0).unwrap();

        window.set_cursor_grab(true).unwrap();
        window.set_cursor_visible(false);
    }
}
