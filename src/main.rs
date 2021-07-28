// #![allow(dead_code)]

use bevy::{input::keyboard::KeyboardInput, prelude::*};

mod controller;
use controller::{Controller, ControllerAction, KeyEvent, KeyEventType, KeyPressMode};

const SCR_W: f32 = 1000.;
const SCR_H: f32 = 800.;
const SCR_NAME: &str = "RE-FLEX";


fn player_move(player: &mut LocalPlayer, _args: &CtrlArgs, key_mode: KeyPressMode, bind_arg: i8) {
    if key_mode == KeyPressMode::RELEASED {
        return
    }

    // if key_mode == KeyPressMode::PRESSED {
    //     println!("pressed {} | mouse pos {}", bind_arg, args.mouse_pos);
    // }

    match bind_arg {
        1 => {player.pos += Vec3::new(0.,  5., 0.);}
        2 => {player.pos += Vec3::new(-5., 0., 0.);}
        3 => {player.pos += Vec3::new(0., -5., 0.);}
        4 => {player.pos += Vec3::new( 5., 0., 0.);}
        _ => {}
    }
}


fn main() {
    let mut ctrl: Controller<LocalPlayer, CtrlArgs, KeyCode> = Controller::new();
    
    ctrl.bind_key(KeyCode::W, ControllerAction {func: player_move, bind_arg: 1})
        .bind_key(KeyCode::A, ControllerAction {func: player_move, bind_arg: 2})
        .bind_key(KeyCode::S, ControllerAction {func: player_move, bind_arg: 3})
        .bind_key(KeyCode::D, ControllerAction {func: player_move, bind_arg: 4});

    App::build()
        .insert_resource(WindowDescriptor {
            title: SCR_NAME.to_string(),
            width: SCR_W,
            height: SCR_H,
            vsync: true,
            ..Default::default()
        })
        .insert_resource(ctrl)
        .add_plugins(DefaultPlugins)
        .add_startup_system(engine_setup.system())
        .add_system(keyboard_event_system.system().label("keyboard_input"))
        .add_system(controll_local_player_system.system().after("keyboard_input"))
        .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
        .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        .run();
}

struct Name(String);

struct LocalPlayer {
    pos: Vec3,
}

struct CtrlArgs {
    mouse_pos: Vec2,
}

// struct Stats {
//     speed: f32,
//     hp: i32,
// }

fn engine_setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    // commands.spawn_bundle(UiCameraBundle::default());

    // player
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.75, 0.25, 0.25).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(30.0, 30.0)),
            ..Default::default()
        })
        .insert(LocalPlayer {pos: Vec3::new(0.0, 0.0, 0.0)})
        .insert(Name("@pl".to_string()));
        // .insert(Stats {speed: 1.0, hp: 10});
}

/// This system prints out all keyboard events as they come in
fn keyboard_event_system(mut controller: ResMut<Controller<LocalPlayer, CtrlArgs, KeyCode>>, mut keyboard_input_events: EventReader<KeyboardInput>, mut query: Query<(&mut LocalPlayer, &mut Transform)>) {
    for event in keyboard_input_events.iter() {
        let key = event.key_code.expect("no KeyCode inside a keyboard event");

        // if key == KeyCode::W && event.state == bevy::input::ElementState::Pressed {
        //     for (mut entity, mut transform) in query.iter_mut() {
        //         entity.pos.y += 5.;
        //         transform.translation = entity.pos;
        //     }
        // }

        match event.state {
            bevy::input::ElementState::Pressed  => { controller.key_event(KeyEvent {key: key, etype: KeyEventType::PRESSED }) }
            bevy::input::ElementState::Released => { controller.key_event(KeyEvent {key: key, etype: KeyEventType::RELEASED}) }
        }
    }
}

fn controll_local_player_system(mut controller: ResMut<Controller<LocalPlayer, CtrlArgs, KeyCode>>, windows: ResMut<Windows>, mut query: Query<(&mut LocalPlayer, &mut Transform)>) {
    controller.controller_tick();

    let window = windows.get_primary().unwrap();

    let args: CtrlArgs = CtrlArgs {mouse_pos: match window.cursor_position() {Some(pos) => pos, None => Vec2::new(0., 0.)} };

    for (mut entity, mut transform) in query.iter_mut() {
        controller.bindings_tick(&mut entity, &args);
        transform.translation = entity.pos;
    }
}
