use bevy::prelude::*;

use crate::{
    effects::PlayerHitEffect,
    game::{GameState, GameplayEntity, HitCooldown, Lives, Score, SessionActive},
    player::{HitFlash, Player},
    util::random_range,
};

#[derive(Component)]
pub struct Raindrop {
    speed_offset: f32,
}

#[derive(Resource)]
pub struct RainTimer(pub Timer);

pub struct RainPlugin;

impl Plugin for RainPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RainTimer(Timer::from_seconds(0.22, TimerMode::Repeating)))
            .add_systems(
                Update,
                (spawn_raindrops, move_raindrops, check_rain_collision)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

fn spawn_raindrops(mut commands: Commands, time: Res<Time>, mut rain_timer: ResMut<RainTimer>) {
    rain_timer.0.tick(time.delta());

    if rain_timer.0.just_finished() {
        let seed = time.elapsed_seconds() * 91.7;
        let x = random_range(seed, -450.0, 450.0);
        let length = random_range(seed + 3.4, 20.0, 38.0);

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(0.35, 0.68, 1.0, 0.82),
                    custom_size: Some(Vec2::new(5.0, length)),
                    ..default()
                },
                transform: Transform::from_xyz(x, 330.0, 2.0)
                    .with_rotation(Quat::from_rotation_z(-0.08)),
                ..default()
            },
            Raindrop {
                speed_offset: random_range(seed + 8.2, -35.0, 85.0),
            },
            GameplayEntity,
        ));
    }
}

fn move_raindrops(
    mut commands: Commands,
    mut rain_query: Query<(Entity, &mut Transform, &Raindrop)>,
    time: Res<Time>,
    score: Res<Score>,
) {
    let base_speed = 390.0 + score.0 as f32 * 24.0;

    for (entity, mut transform, raindrop) in rain_query.iter_mut() {
        transform.translation.y -= (base_speed + raindrop.speed_offset) * time.delta_seconds();
        transform.translation.x -= 24.0 * time.delta_seconds();

        if transform.translation.y < -340.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn check_rain_collision(
    mut commands: Commands,
    mut hit_effects: EventWriter<PlayerHitEffect>,
    mut player_query: Query<(Entity, &mut Transform), With<Player>>,
    rain_query: Query<(Entity, &Transform), (With<Raindrop>, Without<Player>)>,
    mut lives: ResMut<Lives>,
    mut cooldown: ResMut<HitCooldown>,
    mut session: ResMut<SessionActive>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if !cooldown.0.finished() {
        return;
    }

    let Ok((player_entity, mut player)) = player_query.get_single_mut() else {
        return;
    };

    for (rain_entity, rain) in rain_query.iter() {
        let difference = player.translation - rain.translation;
        if difference.x.abs() < 30.0 && difference.y.abs() < 28.0 {
            commands.entity(rain_entity).despawn();
            commands.entity(player_entity).insert(HitFlash {
                timer: Timer::from_seconds(1.0, TimerMode::Once),
            });
            player.translation = Vec3::ZERO;
            lives.0 = lives.0.saturating_sub(1);
            cooldown.0.reset();
            hit_effects.send(PlayerHitEffect);

            if lives.0 == 0 {
                session.0 = false;
                next_state.set(GameState::Lost);
            }
            break;
        }
    }
}
