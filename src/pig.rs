use crate::Money;
use crate::Player;
use bevy::prelude::*;

pub struct PigPlugin;

impl Plugin for PigPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_pig, pig_lifetime));
    }
}

#[derive(Component)]
pub struct Pig {
    lifetime: Timer,
}

fn spawn_pig(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    mut money: ResMut<Money>,
    player_t: Query<&Transform, With<Player>>,
) {
    if input.just_pressed(KeyCode::Space) {
        if money.0 >= 10.0 {
            money.0 -= 10.0;
            info!("spent $10 on pig. remaining money: ${}", money.0);

            let texture = asset_server.load("pig.png");

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(50.0, 50.0)),
                        ..default()
                    },
                    transform: Transform {
                        translation: player_t.single().translation,
                        ..default()
                    },
                    texture,
                    ..default()
                },
                Pig {
                    lifetime: Timer::from_seconds(2.0, TimerMode::Once),
                },
            ));
        }
    }
}
fn pig_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut pigs: Query<(Entity, &mut Pig)>,
    mut money: ResMut<Money>,
) {
    for (pig_entity, mut pig) in &mut pigs {
        pig.lifetime.tick(time.delta());

        if pig.lifetime.finished() {
            money.0 += 15.0;

            commands.entity(pig_entity).despawn();

            info!("sold pig for $15! Current money: {}", money.0);
        }
    }
}
