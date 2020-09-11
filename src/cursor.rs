use crate::window::LockCursor;
use bevy::prelude::*;
use bevy::window::WindowCreated;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<LockCursorState>()
            .add_system(lock_cursor_system.system());
    }
}

#[derive(Default)]
struct LockCursorState {
    window_created_event_reader: EventReader<WindowCreated>,
}

fn lock_cursor_system(
    mut state: ResMut<LockCursorState>,
    events: Res<Events<WindowCreated>>,
    mut lock_commands: ResMut<Events<LockCursor>>,
) {
    if let Some(event) = state.window_created_event_reader.latest(&events) {
        lock_commands.send(LockCursor(event.id));
    }
}
