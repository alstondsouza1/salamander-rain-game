use std::time::Duration;

use bevy::prelude::*;

use crate::{
    constants::{
        ATLAS_COLUMNS, ATLAS_FRAME_SIZE, PLAYER_SPEED, PLAYFIELD_HALF_HEIGHT, PLAYFIELD_HALF_WIDTH,
    },
    game::{GameState, GameplayEntity},
};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct HitFlash {
    pub timer: Timer,
}

#[derive(Component)]
struct PlayerAnimation {
    timer: Timer,
}

#[derive(Resource)]
pub struct PlayerAtlas(pub Handle<TextureAtlasLayout>);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_player_atlas).add_systems(
            Update,
            (player_movement, animate_player, animate_hit_flash)
                .run_if(in_state(GameState::Playing)),
        );
    }
}

fn load_player_atlas(mut commands: Commands, mut layouts: ResMut<Assets<TextureAtlasLayout>>) {
    let layout = TextureAtlasLayout::from_grid(ATLAS_FRAME_SIZE, ATLAS_COLUMNS, 1, None, None);
    commands.insert_resource(PlayerAtlas(layouts.add(layout)));
}

pub fn spawn_player(
    commands: &mut Commands,
    asset_server: &AssetServer,
    atlas: Handle<TextureAtlasLayout>,
) {
    commands.spawn((
        Sprite {
            image: asset_server.load("sprites/salamander-run.png"),
            texture_atlas: Some(TextureAtlas {
                layout: atlas,
                index: 0,
            }),
            custom_size: Some(Vec2::new(105.0, 140.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 3.0),
        Player,
        PlayerAnimation {
            timer: Timer::new(Duration::from_millis(110), TimerMode::Repeating),
        },
        GameplayEntity,
    ));
}

fn player_movement(
    input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let Ok(mut player) = player_query.single_mut() else {
        return;
    };
    let mut direction = Vec3::ZERO;

    if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }

    let direction = direction.normalize_or_zero();
    player.translation += direction * PLAYER_SPEED * time.delta_secs();
    player.translation.x = player
        .translation
        .x
        .clamp(-PLAYFIELD_HALF_WIDTH, PLAYFIELD_HALF_WIDTH);
    player.translation.y = player
        .translation
        .y
        .clamp(-PLAYFIELD_HALF_HEIGHT, PLAYFIELD_HALF_HEIGHT);

    if direction.length_squared() > 0.0 {
        player.rotation = Quat::from_rotation_z(direction.y.atan2(direction.x));
    }
}

fn animate_player(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut PlayerAnimation, &mut Sprite), With<Player>>,
) {
    let moving = input.any_pressed([
        KeyCode::KeyW,
        KeyCode::KeyA,
        KeyCode::KeyS,
        KeyCode::KeyD,
        KeyCode::ArrowUp,
        KeyCode::ArrowDown,
        KeyCode::ArrowLeft,
        KeyCode::ArrowRight,
    ]);

    for (mut animation, mut sprite) in &mut query {
        let Some(atlas) = &mut sprite.texture_atlas else {
            continue;
        };
        if !moving {
            atlas.index = 0;
            continue;
        }
        animation.timer.tick(time.delta());
        if animation.timer.just_finished() {
            atlas.index = (atlas.index + 1) % ATLAS_COLUMNS as usize;
        }
    }
}

fn animate_hit_flash(
    mut commands: Commands,
    time: Res<Time>,
    mut player_query: Query<(Entity, &mut Visibility, &mut HitFlash), With<Player>>,
) {
    let Ok((entity, mut visibility, mut flash)) = player_query.single_mut() else {
        return;
    };

    flash.timer.tick(time.delta());
    *visibility = if (flash.timer.elapsed_secs() * 14.0) as u32 % 2 == 0 {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };

    if flash.timer.is_finished() {
        *visibility = Visibility::Visible;
        commands.entity(entity).remove::<HitFlash>();
    }
}
