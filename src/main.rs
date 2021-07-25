use bevy::{input::keyboard::KeyboardInput, prelude::*};

mod controller;
mod controls_setup;

const SCR_W: f32 = 1000.;
const SCR_H: f32 = 800.;
const SCR_NAME: &str = "RE-FLEX";

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: SCR_NAME.to_string(),
            width: SCR_W,
            height: SCR_H,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(engine_setup.system())
        .add_system(print_keyboard_event_system.system())
        .run();
}

struct Name(String);

struct LocalPlayer;

type UnitPos = Vec2;

// struct Stats {
//     speed: f32,
//     hp: i32,
// }

fn engine_setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // player
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.75, 0.25, 0.25).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(30.0, 30.0)),
            ..Default::default()
        })
        .insert(LocalPlayer)
        .insert(Name("@pl".to_string()))
        .insert(UnitPos::new(0.0, 0.0));
        // .insert(Stats {speed: 1.0, hp: 10});
}

/// This system prints out all keyboard events as they come in
fn print_keyboard_event_system(mut keyboard_input_events: EventReader<KeyboardInput>) {
    for event in keyboard_input_events.iter() {
        println!("{:?}", event);
    }
}

struct LogicTickEvent {}
struct PhysicsTickEvent {}
struct VisualTickEvent {}

struct LogicEventTriggerState {
    event_timer: Timer,
}

struct PhysicsEventTriggerState {
    event_timer: Timer,
}

struct VisualEventTriggerState {
    event_timer: Timer,
}
