use bevy::prelude::*;

use crate::{
    effects::{PlayerHitEffect, SplashEffect},
    game::{GameState, GameplayEntity, HitCooldown, Lives, Score, SessionActive},
    level::CurrentLevel,
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
        app.insert_resource(RainTimer(Timer::from_seconds(0.28, TimerMode::Repeating)))
            .add_systems(
                Update,
                (spawn_raindrops, move_raindrops, check_rain_collision)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

fn spawn_raindrops(
    mut commands: Commands,
    time: Res<Time>,
    mut rain_timer: ResMut<RainTimer>,
    level: Res<CurrentLevel>,
) {
    rain_timer
        .0
        .set_duration(std::time::Duration::from_secs_f32(
            level.definition().rain_interval,
        ));
    rain_timer.0.tick(time.delta());

    if rain_timer.0.just_finished() {
        commands.spawn((
            Sprite::from_color(
                Color::srgba(0.35, 0.68, 1.0, 0.82),
                Vec2::new(5.0, random_range(20.0, 38.0)),
            ),
            Transform::from_xyz(random_range(-450.0, 450.0), 330.0, 2.0)
                .with_rotation(Quat::from_rotation_z(-0.08)),
            Raindrop {
                speed_offset: random_range(-35.0, 85.0),
            },
            GameplayEntity,
        ));
    }
}

fn move_raindrops(
    mut commands: Commands,
    mut splashes: MessageWriter<SplashEffect>,
    mut rain_query: Query<(Entity, &mut Transform, &Raindrop)>,
    time: Res<Time>,
    score: Res<Score>,
    level: Res<CurrentLevel>,
) {
    let speed = level.definition().rain_speed + score.0 as f32 * 18.0;

    for (entity, mut transform, raindrop) in &mut rain_query {
        transform.translation.y -= (speed + raindrop.speed_offset) * time.delta_secs();
        transform.translation.x -= 24.0 * time.delta_secs();

        if transform.translation.y < -300.0 {
            splashes.write(SplashEffect(transform.translation));
            commands.entity(entity).despawn();
        }
    }
}

fn check_rain_collision(
    mut commands: Commands,
    mut hit_effects: MessageWriter<PlayerHitEffect>,
    mut splashes: MessageWriter<SplashEffect>,
    mut player_query: Query<(Entity, &mut Transform), With<Player>>,
    rain_query: Query<(Entity, &Transform), (With<Raindrop>, Without<Player>)>,
    mut lives: ResMut<Lives>,
    mut cooldown: ResMut<HitCooldown>,
    mut session: ResMut<SessionActive>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if !cooldown.0.is_finished() {
        return;
    }

    let Ok((player_entity, mut player)) = player_query.single_mut() else {
        return;
    };

    for (rain_entity, rain) in &rain_query {
        let difference = player.translation - rain.translation;
        if difference.x.abs() < 32.0 && difference.y.abs() < 30.0 {
            splashes.write(SplashEffect(rain.translation));
            commands.entity(rain_entity).despawn();
            commands.entity(player_entity).insert(HitFlash {
                timer: Timer::from_seconds(1.0, TimerMode::Once),
            });
            player.translation = Vec3::ZERO;
            lives.0 = lives.0.saturating_sub(1);
            cooldown.0.reset();
            hit_effects.write(PlayerHitEffect);

            if lives.0 == 0 {
                session.0 = false;
                next_state.set(GameState::Lost);
            }
            break;
        }
    }
}
