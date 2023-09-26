mod pig;
use crate::pig::PigPlugin;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct Player {
    speed: f32,
    rot_speed: f32,
}

#[derive(Resource)]
pub struct Money(f32);

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "joe's new boids eventually".into(),
                        resolution: (640.0, 480.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .insert_resource(Money(100.0))
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
        )
        .register_type::<Player>()
        .add_plugins(PigPlugin)
        //.add_systems(Schedule, (system, system, system ...))
        .add_systems(Startup, setup)
        .add_systems(Update, (player_movement))
        .run();
}
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 256.0,
        min_height: 144.0,
    };

    commands.spawn(camera);

    let texture = asset_server.load("character.png");

    commands.spawn((
        SpriteBundle {
            texture, // == texture: texture
            sprite: Sprite {
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            ..default()
        },
        Player {
            speed: 300.0,
            rot_speed: 0.01
        },
        Name::new("Playeruwu"),
    ));
}
fn player_movement(
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, player) = characters.single_mut();
    let mut v = Vec2::new(0.0, 0.0);
    let mut r = 0.0;
    if input.pressed(KeyCode::W) {
        v.y += 1.0;
    }
    if input.pressed(KeyCode::A) {
        v.x -= 1.0;
    }
    if input.pressed(KeyCode::S) {
        v.y -= 1.0;
    }
    if input.pressed(KeyCode::D) {
        v.x += 1.0;
    }
    if input.pressed(KeyCode::Up) {
        v.y += 1.0;
    }
    if input.pressed(KeyCode::Down) {
        v.y -= 1.0;
    }
    if input.pressed(KeyCode::Left) {
        v.x -= 1.0;
    }
    if input.pressed(KeyCode::Right) {
        v.x += 1.0;
    }
    if input.just_pressed(KeyCode::E) {
        r -= 1.0;
    }
    if input.just_pressed(KeyCode::Q) {
        r += 1.0
    }
    transform.rotate_z(r * std::f32::consts::PI);
    match v.try_normalize() {
        None => (),
        Some(val) => {
            transform.translation.x += val.x * player.speed * time.delta_seconds();
            transform.translation.y += val.y * player.speed * time.delta_seconds();
        }
    }
}
