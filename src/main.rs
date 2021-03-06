use bevy::{prelude::*};

mod controller;
use controller::{Controller, ControllerAction, KeyEvent, KeyEventType, KeyPressMode, binding_args::BindArg};

const SCR_W:     f32 = 1000.;
const SCR_H:     f32 = 800.;
const SCR_NAME: &str = "RE-FLEX";

fn main() {
    let mut controller: Controller<LocalPlayer, CtrlArgs, KeyCode, BindArg> = Controller::new();
    
    controller.bind_key(KeyCode::W, ControllerAction {func: player_move, bind_arg: BindArg::Up   })
              .bind_key(KeyCode::A, ControllerAction {func: player_move, bind_arg: BindArg::Left })
              .bind_key(KeyCode::S, ControllerAction {func: player_move, bind_arg: BindArg::Down })
              .bind_key(KeyCode::D, ControllerAction {func: player_move, bind_arg: BindArg::Right});

    App::build()
        .insert_resource(WindowDescriptor {
            title: SCR_NAME.to_string(),
            width: SCR_W,
            height: SCR_H,
            vsync: true,
            ..Default::default()
        })
        .insert_resource(controller)
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(keyboard_event_system.system().label("keyboard_input"))
        .add_system(controller_local_player_system.system().after("keyboard_input"))
        // .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
        // .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        .run();
}

fn player_move(player: &mut LocalPlayer, _args: &CtrlArgs, key_mode: KeyPressMode, bind_arg: BindArg) {
    if key_mode == KeyPressMode::RELEASED {
        return
    }

    match bind_arg {
        BindArg::Up    => {player.pos += Vec3::new(0.,  5., 0.);}
        BindArg::Left  => {player.pos += Vec3::new(-5., 0., 0.);}
        BindArg::Down  => {player.pos += Vec3::new(0., -5., 0.);}
        BindArg::Right => {player.pos += Vec3::new( 5., 0., 0.);}
        _ => {}
    }
}

struct Name(String);
struct Position(Vec3);

struct LocalPlayer;

struct CtrlArgs {
    _mouse_pos: Vec2,
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
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
}

fn keyboard_event_system(mut controller: ResMut<Controller<LocalPlayer, CtrlArgs, KeyCode, BindArg>>, keys: Res<Input<KeyCode>>) {
    for &key in keys.get_just_pressed() {
        controller.key_event(KeyEvent {key: key, etype: KeyEventType::PRESSED });
    }

    for &key in keys.get_just_released() {
        controller.key_event(KeyEvent {key: key, etype: KeyEventType::RELEASED });
    }
}

fn controller_local_player_system(
    mut controller: ResMut<Controller<LocalPlayer, CtrlArgs, KeyCode, BindArg>>,
    windows: ResMut<Windows>,
    mut query: Query<(&mut LocalPlayer, &mut Transform)>
) {
    controller.controller_tick();

    let window = windows.get_primary().unwrap();

    let args: CtrlArgs = CtrlArgs {_mouse_pos: match window.cursor_position() {Some(pos) => pos, None => Vec2::new(0., 0.)} };

    for (mut entity, mut transform) in query.iter_mut() {
        controller.bindings_tick(&mut entity, &args);
        transform.translation = entity.pos;
    }
}
