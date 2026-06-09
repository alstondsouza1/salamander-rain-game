use std::time::Duration;

use bevy::prelude::*;

use crate::{
    background::MainCamera,
    constants::{ATLAS_COLUMNS, ATLAS_FRAME_SIZE, FIREFLY_YELLOW},
    game::GameplayEntity,
    util::random_range,
};

#[derive(Message)]
pub struct CollectionEffect(pub Vec3);

#[derive(Message)]
pub struct PlayerHitEffect;

#[derive(Message)]
pub struct SplashEffect(pub Vec3);

#[derive(Component)]
struct Particle {
    velocity: Vec3,
    lifetime: Timer,
}

#[derive(Component)]
struct SplashAnimation(Timer);

#[derive(Resource)]
struct SplashAtlas(Handle<TextureAtlasLayout>);

#[derive(Resource, Default)]
struct CameraShake {
    timer: Timer,
    strength: f32,
}

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<CollectionEffect>()
            .add_message::<PlayerHitEffect>()
            .add_message::<SplashEffect>()
            .init_resource::<CameraShake>()
            .add_systems(Startup, load_splash_atlas)
            .add_systems(
                Update,
                (
                    spawn_collection_particles,
                    spawn_splashes,
                    trigger_camera_shake,
                    move_particles,
                    animate_splashes,
                    update_camera_shake,
                ),
            );
    }
}

fn load_splash_atlas(mut commands: Commands, mut layouts: ResMut<Assets<TextureAtlasLayout>>) {
    commands.insert_resource(SplashAtlas(layouts.add(TextureAtlasLayout::from_grid(
        ATLAS_FRAME_SIZE,
        ATLAS_COLUMNS,
        1,
        None,
        None,
    ))));
}

fn spawn_collection_particles(
    mut commands: Commands,
    mut messages: MessageReader<CollectionEffect>,
) {
    for message in messages.read() {
        for _ in 0..10 {
            let angle = random_range(0.0, std::f32::consts::TAU);
            let speed = random_range(45.0, 120.0);
            commands.spawn((
                Sprite::from_color(FIREFLY_YELLOW, Vec2::splat(random_range(3.0, 7.0))),
                Transform::from_translation(message.0 + Vec3::Z * 2.0),
                Particle {
                    velocity: Vec3::new(angle.cos(), angle.sin(), 0.0) * speed,
                    lifetime: Timer::from_seconds(0.45, TimerMode::Once),
                },
                GameplayEntity,
            ));
        }
    }
}

fn spawn_splashes(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    atlas: Res<SplashAtlas>,
    mut messages: MessageReader<SplashEffect>,
) {
    for message in messages.read() {
        commands.spawn((
            Sprite {
                image: asset_server.load("sprites/rain-splash.png"),
                texture_atlas: Some(TextureAtlas {
                    layout: atlas.0.clone(),
                    index: 0,
                }),
                custom_size: Some(Vec2::new(75.0, 100.0)),
                ..default()
            },
            Transform::from_translation(Vec3::new(message.0.x, message.0.y.max(-275.0), 2.5)),
            SplashAnimation(Timer::new(Duration::from_millis(70), TimerMode::Repeating)),
            GameplayEntity,
        ));
    }
}

fn move_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut particles: Query<(Entity, &mut Transform, &mut Sprite, &mut Particle)>,
) {
    for (entity, mut transform, mut sprite, mut particle) in &mut particles {
        particle.lifetime.tick(time.delta());
        transform.translation += particle.velocity * time.delta_secs();
        particle.velocity *= 0.92;
        sprite.color.set_alpha(1.0 - particle.lifetime.fraction());

        if particle.lifetime.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn animate_splashes(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut SplashAnimation, &mut Sprite)>,
) {
    for (entity, mut animation, mut sprite) in &mut query {
        animation.0.tick(time.delta());
        if animation.0.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index += 1;
                if atlas.index >= ATLAS_COLUMNS as usize {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}

fn trigger_camera_shake(
    mut messages: MessageReader<PlayerHitEffect>,
    mut shake: ResMut<CameraShake>,
) {
    if messages.read().next().is_some() {
        shake.timer = Timer::from_seconds(0.32, TimerMode::Once);
        shake.strength = 12.0;
    }
}

fn update_camera_shake(
    time: Res<Time>,
    mut shake: ResMut<CameraShake>,
    mut camera: Query<&mut Transform, With<MainCamera>>,
) {
    let Ok(mut transform) = camera.single_mut() else {
        return;
    };

    if shake.timer.is_finished() {
        transform.translation.x = 0.0;
        transform.translation.y = 0.0;
        return;
    }

    shake.timer.tick(time.delta());
    let fade = 1.0 - shake.timer.fraction();
    let phase = time.elapsed_secs() * 75.0;
    transform.translation.x = phase.sin() * shake.strength * fade;
    transform.translation.y = (phase * 1.7).cos() * shake.strength * fade;
}
