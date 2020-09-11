use crate::window::LockCursor;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::WindowCreated;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<LockCursorSystemState>()
            .add_system(lock_cursor_system.system())
            .add_stage_before(stage::EVENT_UPDATE, STAGE)
            .add_system_to_stage(STAGE, clear_cursor_moved_events_system.system())
            .init_resource::<PublishCursorMovedSystemState>()
            .add_system_to_stage(
                STAGE,
                publish_cursor_moved_events_within_bounds_system.system(),
            );
    }
}

#[derive(Default)]
struct LockCursorSystemState {
    window_created_event_reader: EventReader<WindowCreated>,
}

fn lock_cursor_system(
    mut state: ResMut<LockCursorSystemState>,
    events: Res<Events<WindowCreated>>,
    mut lock_commands: ResMut<Events<LockCursor>>,
) {
    if let Some(event) = state.window_created_event_reader.latest(&events) {
        lock_commands.send(LockCursor(event.id));
    }
}

const STAGE: &str = "cursor_stage";

fn clear_cursor_moved_events_system(mut events: ResMut<Events<CursorMoved>>) {
    events.clear();
}

#[derive(Default)]
struct PublishCursorMovedSystemState {
    mouse_motion_event_reader: EventReader<MouseMotion>,
    last_cursor_pos: Vec2,
    cursor_pos: Vec2,
}

fn publish_cursor_moved_events_within_bounds_system(
    mut state: ResMut<PublishCursorMovedSystemState>,
    mouse_events: Res<Events<MouseMotion>>,
    mut cursor_moved_events: ResMut<Events<CursorMoved>>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();

    for event in state.mouse_motion_event_reader.iter(&mouse_events) {
        let x = bevy::math::clamp(
            state.cursor_pos.x() + event.delta.x(),
            0.0,
            window.width as f32,
        );

        let y = bevy::math::clamp(
            state.cursor_pos.y() + -event.delta.y(),
            0.0,
            window.height as f32,
        );

        state.cursor_pos.set_x(x);
        state.cursor_pos.set_y(y);
    }

    if state.cursor_pos == state.last_cursor_pos {
        return;
    }

    cursor_moved_events.send(CursorMoved {
        id: bevy::window::WindowId::primary(),
        position: state.cursor_pos,
    });

    state.last_cursor_pos = state.cursor_pos;
}
