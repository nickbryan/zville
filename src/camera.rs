use bevy::input::mouse;
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .init_resource::<MoveSystemState>()
            .add_system(move_system.system())
            .init_resource::<ZoomSystemState>()
            .add_system(zoom_system.system());
    }
}

struct CameraComponent;

const STARTING_YAW: f32 = 45.0;
const STARTING_PITCH: f32 = -15.0;

fn setup(mut commands: Commands) {
    commands
        .spawn(Camera3dComponents {
            translation: Translation::new(25.0, 20.0, 25.0),
            rotation: Rotation::from_rotation_yxz(
                STARTING_YAW.to_radians(),
                STARTING_PITCH.to_radians(),
                0.0,
            ),
            ..Default::default()
        })
        .with(CameraComponent);
}

const SCROLL_SPEED: f32 = 100.0;
const SCROLL_MARGIN: f32 = 0.1;

#[derive(Default)]
struct MoveSystemState {
    cursor_moved_event_reader: EventReader<CursorMoved>,
    cursor_position: Vec2,
}

fn move_system(
    mut state: ResMut<MoveSystemState>,
    events: Res<Events<CursorMoved>>,
    windows: Res<Windows>,
    time: Res<Time>,
    mut camera_query: Query<(&CameraComponent, &mut Translation, &Rotation)>,
) {
    let window = windows.get_primary().unwrap();
    let screen_width = window.width as f32;
    let screen_height = window.height as f32;

    for event in state.cursor_moved_event_reader.iter(&events) {
        state
            .cursor_position
            .set_x(event.position.x() / screen_width);
        state
            .cursor_position
            .set_y(event.position.y() / screen_height);
    }

    let position = state.cursor_position;
    let mut movement = Vec3::zero();

    if position.x() < SCROLL_MARGIN {
        *movement.x_mut() -= (SCROLL_MARGIN - position.x()) * SCROLL_SPEED;
    } else if position.x() > (1.0 - SCROLL_MARGIN) {
        *movement.x_mut() += (position.x() - (1.0 - SCROLL_MARGIN)) * SCROLL_SPEED;
    }

    if position.y() < SCROLL_MARGIN {
        *movement.z_mut() += (SCROLL_MARGIN - position.y()) * SCROLL_SPEED;
    } else if position.y() > (1.0 - SCROLL_MARGIN) {
        *movement.z_mut() -= (position.y() - (1.0 - SCROLL_MARGIN)) * SCROLL_SPEED;
    }

    for (_, mut translation, rotation) in &mut camera_query.iter() {
        // Account for camera direction.
        movement = rotation.mul_vec3(movement);

        *translation.x_mut() += movement.x() * time.delta_seconds;
        *translation.z_mut() += movement.z() * time.delta_seconds;
    }
}

const ZOOM_SPEED: f32 = 0.5;
const MAX_ZOOM: f32 = 30.0;
const MIN_ZOOM: f32 = 10.0;

#[derive(Default)]
struct ZoomSystemState {
    mouse_wheel_event_reader: EventReader<mouse::MouseWheel>,
}

fn zoom_system(
    mut state: ResMut<ZoomSystemState>,
    events: Res<Events<mouse::MouseWheel>>,
    time: Res<Time>,
    mut camera_query: Query<(&CameraComponent, &mut Translation)>,
) {
    for event in state.mouse_wheel_event_reader.iter(&events) {
        if let mouse::MouseScrollUnit::Pixel = event.unit {
            for (_, mut translation) in &mut camera_query.iter() {
                let current = translation.y();
                let movement = (event.y * ZOOM_SPEED) * time.delta_seconds;
                let new_value = current + movement;

                if new_value < MIN_ZOOM {
                    translation.set_y(MIN_ZOOM);
                    return;
                }

                if new_value > MAX_ZOOM {
                    translation.set_y(MAX_ZOOM);
                    return;
                }

                translation.set_y(new_value);
            }
        } else {
            panic!("we currently only deal with pixel units on mouse scroll");
        }
    }
}
