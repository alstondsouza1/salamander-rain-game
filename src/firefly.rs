use bevy::prelude::*;

use crate::{
    constants::{FIREFLY_COUNT, FIREFLY_YELLOW, WIN_SCORE},
    effects::CollectionEffect,
    game::{save_high_score, GameState, GameplayEntity, HighScore, Score, SessionActive},
    player::Player,
    util::random_range,
};

#[derive(Component)]
pub struct Firefly;

#[derive(Component)]
struct Glow {
    phase: f32,
}

pub struct FireflyPlugin;

impl Plugin for FireflyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (collect_firefly, animate_fireflies).run_if(in_state(GameState::Playing)),
        );
    }
}

pub fn spawn_fireflies(commands: &mut Commands, seed: f32) {
    for i in 0..FIREFLY_COUNT {
        let x = random_range(seed + i as f32 * 12.989, -410.0, 410.0);
        let y = random_range(seed + i as f32 * 78.233, -235.0, 230.0);
        let phase = random_range(seed + i as f32 * 4.17, 0.0, std::f32::consts::TAU);

        commands
            .spawn((
                SpatialBundle {
                    transform: Transform::from_xyz(x, y, 1.0),
                    ..default()
                },
                Firefly,
                GameplayEntity,
            ))
            .with_children(|parent| {
                parent.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgba(1.0, 0.82, 0.12, 0.17),
                            custom_size: Some(Vec2::splat(44.0)),
                            ..default()
                        },
                        ..default()
                    },
                    Glow { phase },
                ));

                parent.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: FIREFLY_YELLOW,
                        custom_size: Some(Vec2::splat(12.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 0.0, 1.0)
                        .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_4)),
                    ..default()
                });
                parent.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.22, 0.18, 0.08),
                        custom_size: Some(Vec2::new(4.0, 10.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 0.0, 2.0),
                    ..default()
                });
            });
    }
}

fn collect_firefly(
    mut commands: Commands,
    mut effects: EventWriter<CollectionEffect>,
    mut score: ResMut<Score>,
    mut high_score: ResMut<HighScore>,
    mut session: ResMut<SessionActive>,
    mut next_state: ResMut<NextState<GameState>>,
    player_query: Query<&Transform, With<Player>>,
    firefly_query: Query<(Entity, &Transform), With<Firefly>>,
) {
    let Ok(player) = player_query.get_single() else {
        return;
    };

    for (entity, transform) in firefly_query.iter() {
        if player.translation.distance(transform.translation) < 34.0 {
            commands.entity(entity).despawn_recursive();
            effects.send(CollectionEffect(transform.translation));
            score.0 += 1;

            if score.0 > high_score.0 {
                high_score.0 = score.0;
                save_high_score(high_score.0);
            }

            if score.0 >= WIN_SCORE {
                session.0 = false;
                next_state.set(GameState::Won);
            }
        }
    }
}

fn animate_fireflies(time: Res<Time>, mut glow_query: Query<(&Glow, &mut Transform, &mut Sprite)>) {
    for (glow, mut transform, mut sprite) in glow_query.iter_mut() {
        let pulse = (time.elapsed_seconds() * 3.0 + glow.phase).sin() * 0.5 + 0.5;
        transform.scale = Vec3::splat(0.82 + pulse * 0.28);
        sprite.color.set_a(0.10 + pulse * 0.16);
    }
}
