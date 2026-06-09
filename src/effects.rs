use bevy::prelude::*;

use crate::{
    background::MainCamera, constants::FIREFLY_YELLOW, game::GameplayEntity, util::random_range,
};

#[derive(Event)]
pub struct CollectionEffect(pub Vec3);

#[derive(Event)]
pub struct PlayerHitEffect;

#[derive(Component)]
struct Particle {
    velocity: Vec3,
    lifetime: Timer,
}

#[derive(Resource)]
struct CameraShake {
    timer: Timer,
    strength: f32,
}

impl Default for CameraShake {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.0, TimerMode::Once),
            strength: 0.0,
        }
    }
}

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollectionEffect>()
            .add_event::<PlayerHitEffect>()
            .init_resource::<CameraShake>()
            .add_systems(
                Update,
                (
                    spawn_collection_particles,
                    trigger_camera_shake,
                    move_particles,
                    update_camera_shake,
                ),
            );
    }
}

fn spawn_collection_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut events: EventReader<CollectionEffect>,
) {
    for event in events.read() {
        for i in 0..10 {
            let seed = time.elapsed_seconds() * 47.0 + i as f32 * 9.3;
            let angle = random_range(seed, 0.0, std::f32::consts::TAU);
            let speed = random_range(seed + 2.1, 45.0, 120.0);

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: FIREFLY_YELLOW,
                        custom_size: Some(Vec2::splat(random_range(seed + 4.0, 3.0, 7.0))),
                        ..default()
                    },
                    transform: Transform::from_translation(event.0 + Vec3::Z * 2.0),
                    ..default()
                },
                Particle {
                    velocity: Vec3::new(angle.cos(), angle.sin(), 0.0) * speed,
                    lifetime: Timer::from_seconds(0.45, TimerMode::Once),
                },
                GameplayEntity,
            ));
        }
    }
}

fn move_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut particles: Query<(Entity, &mut Transform, &mut Sprite, &mut Particle)>,
) {
    for (entity, mut transform, mut sprite, mut particle) in particles.iter_mut() {
        particle.lifetime.tick(time.delta());
        transform.translation += particle.velocity * time.delta_seconds();
        particle.velocity *= 0.92;
        let alpha = 1.0 - particle.lifetime.fraction();
        sprite.color.set_a(alpha);

        if particle.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn trigger_camera_shake(mut events: EventReader<PlayerHitEffect>, mut shake: ResMut<CameraShake>) {
    if events.read().next().is_some() {
        shake.timer = Timer::from_seconds(0.32, TimerMode::Once);
        shake.strength = 12.0;
    }
}

fn update_camera_shake(
    time: Res<Time>,
    mut shake: ResMut<CameraShake>,
    mut camera: Query<&mut Transform, With<MainCamera>>,
) {
    let Ok(mut transform) = camera.get_single_mut() else {
        return;
    };

    if shake.timer.finished() {
        transform.translation.x = 0.0;
        transform.translation.y = 0.0;
        return;
    }

    shake.timer.tick(time.delta());
    let fade = 1.0 - shake.timer.fraction();
    let phase = time.elapsed_seconds() * 75.0;
    transform.translation.x = phase.sin() * shake.strength * fade;
    transform.translation.y = (phase * 1.7).cos() * shake.strength * fade;
}
