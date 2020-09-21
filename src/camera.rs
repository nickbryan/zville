use bevy::input::mouse;
use bevy::prelude::*;
use bevy_mod_picking::{PickingGroup, PickingMethod, PickingSource};

pub const STARTUP_STAGE: &str = "camera_startup_stage";

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_stage(STARTUP_STAGE)
            .add_startup_system_to_stage(STARTUP_STAGE, setup.system())
            .init_resource::<MoveSystemState>()
            .add_system(move_system.system())
            .init_resource::<ZoomSystemState>()
            .add_system(zoom_system.system())
            .init_resource::<RotateSystemState>()
            .add_system(rotate_system.system());
    }
}

pub struct CameraPickingGroup(pub PickingGroup);

struct CameraComponent;

const STARTING_YAW: f32 = 45.0;
const STARTING_PITCH: f32 = -15.0;

fn setup(mut commands: Commands) {
    let picking_group = PickingGroup::Group(0);

    commands
        .spawn(Camera3dComponents {
            transform: Transform::from_translation_rotation(
                Vec3::new(25.0, 20.0, 25.0),
                Quat::from_rotation_ypr(
                    STARTING_YAW.to_radians(),
                    STARTING_PITCH.to_radians(),
                    0.0,
                ),
            ),
            ..Default::default()
        })
        .with(CameraComponent)
        .with(PickingSource::new(picking_group, PickingMethod::Center));

    commands.insert_resource(CameraPickingGroup(picking_group));
}

const SCROLL_SPEED: f32 = 125.0;
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
    mut camera_query: Query<(&CameraComponent, &mut Transform)>,
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

    for (_, mut transform) in &mut camera_query.iter() {
        // Account for camera direction.
        movement = transform.rotation().mul_vec3(movement);

        let translation = Vec3::new(
            transform.translation().x() + movement.x() * time.delta_seconds,
            transform.translation().y(),
            transform.translation().z() + movement.z() * time.delta_seconds,
        );

        transform.set_translation(translation);
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
    mut camera_query: Query<(&CameraComponent, &mut Transform)>,
) {
    for event in state.mouse_wheel_event_reader.iter(&events) {
        if let mouse::MouseScrollUnit::Pixel = event.unit {
            for (_, mut transform) in &mut camera_query.iter() {
                let current = transform.translation().y();
                let movement = (event.y * ZOOM_SPEED) * time.delta_seconds;
                let new_value = current + movement;

                let mut translation = transform.translation();
                translation.set_y(new_value);

                // TODO: bevy::math::clamp ??
                if new_value < MIN_ZOOM {
                    translation.set_y(MIN_ZOOM);
                }

                if new_value > MAX_ZOOM {
                    translation.set_y(MAX_ZOOM);
                }

                transform.set_translation(translation);
            }
        } else {
            panic!("we currently only deal with pixel units on mouse scroll");
        }
    }
}

#[derive(Default)]
struct RotateSystemState {
    mouse_motion_event_reader: EventReader<mouse::MouseMotion>,
    focus: Option<Vec3>,
}

fn rotate_system(
    mut state: ResMut<RotateSystemState>,
    keyboard_input: Res<Input<KeyCode>>,
    (mouse_button_input, mouse_events): (Res<Input<MouseButton>>, Res<Events<mouse::MouseMotion>>),
    (pick_state, pick_group): (Res<bevy_mod_picking::PickState>, Res<CameraPickingGroup>),
    windows: Res<Windows>,
    mut camera_query: Query<(&CameraComponent, &mut Transform)>,
) {
    if keyboard_input.pressed(KeyCode::LShift) && mouse_button_input.pressed(MouseButton::Left) {
        let mut rotation_move = Vec2::zero();

        for event in state.mouse_motion_event_reader.iter(&mouse_events) {
            rotation_move -= event.delta;
        }

        match state.focus {
            Some(focus) => {
                let window = windows.get_primary().unwrap();
                let screen_width = window.width as f32;
                let screen_height = window.height as f32;

                // Link virtual sphere rotation relative to window to make it feel nicer
                let delta_x = rotation_move.x() / screen_width * std::f32::consts::PI * 2.0;
                let delta_y = rotation_move.y() / screen_height * std::f32::consts::PI;

                let delta_yaw = Quat::from_rotation_y(delta_x);
                let delta_pitch = Quat::from_rotation_x(delta_y);

                for (_, mut transform) in &mut camera_query.iter() {
                    let translation =
                        delta_yaw * delta_pitch * (transform.translation() - focus) + focus;
                    transform.set_translation(translation);

                    let look = Mat4::face_toward(transform.translation(), focus, Vec3::unit_y());
                    transform.set_rotation(look.to_scale_rotation_translation().1);
                }
            }
            None => {
                if let Some(pick) = pick_state.top(pick_group.0) {
                    state.focus = Some(*pick.position());
                }
            }
        }

        return;
    }

    if state.focus.is_some() {
        state.focus = None;
    }
}
