use bevy::prelude::*;

use crate::{
    constants::{
        PLAYER_SPEED, PLAYFIELD_HALF_HEIGHT, PLAYFIELD_HALF_WIDTH, SALAMANDER_DARK,
        SALAMANDER_ORANGE,
    },
    game::{GameState, GameplayEntity},
};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct HitFlash {
    pub timer: Timer,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (player_movement, animate_hit_flash).run_if(in_state(GameState::Playing)),
        );
    }
}

pub fn spawn_player(commands: &mut Commands) {
    commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_xyz(0.0, 0.0, 3.0),
                ..default()
            },
            Player,
            GameplayEntity,
        ))
        .with_children(|parent| {
            spawn_part(
                parent,
                Vec2::new(46.0, 27.0),
                Vec3::ZERO,
                SALAMANDER_ORANGE,
                0.0,
            );
            spawn_part(
                parent,
                Vec2::new(27.0, 25.0),
                Vec3::new(29.0, 1.0, 0.1),
                SALAMANDER_ORANGE,
                0.0,
            );
            spawn_part(
                parent,
                Vec2::new(31.0, 13.0),
                Vec3::new(-35.0, 0.0, 0.0),
                SALAMANDER_ORANGE,
                0.16,
            );
            spawn_part(
                parent,
                Vec2::new(25.0, 10.0),
                Vec3::new(-58.0, -3.0, 0.0),
                SALAMANDER_ORANGE,
                0.30,
            );

            for (x, y, angle) in [
                (-12.0, 20.0, 0.55),
                (12.0, 20.0, -0.55),
                (-12.0, -20.0, -0.55),
                (12.0, -20.0, 0.55),
            ] {
                spawn_part(
                    parent,
                    Vec2::new(22.0, 7.0),
                    Vec3::new(x, y, -0.1),
                    SALAMANDER_ORANGE,
                    angle,
                );
            }

            for x in [-12.0, 8.0] {
                spawn_part(
                    parent,
                    Vec2::splat(8.0),
                    Vec3::new(x, 0.0, 0.2),
                    SALAMANDER_DARK,
                    0.0,
                );
            }

            for y in [-7.0, 8.0] {
                spawn_part(
                    parent,
                    Vec2::splat(5.0),
                    Vec3::new(37.0, y, 0.3),
                    Color::WHITE,
                    0.0,
                );
                spawn_part(
                    parent,
                    Vec2::splat(2.5),
                    Vec3::new(38.0, y, 0.4),
                    SALAMANDER_DARK,
                    0.0,
                );
            }
        });
}

fn spawn_part(parent: &mut ChildBuilder, size: Vec2, position: Vec3, color: Color, rotation: f32) {
    parent.spawn(SpriteBundle {
        sprite: Sprite {
            color,
            custom_size: Some(size),
            ..default()
        },
        transform: Transform::from_translation(position)
            .with_rotation(Quat::from_rotation_z(rotation)),
        ..default()
    });
}

fn player_movement(
    input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let Ok(mut player) = player_query.get_single_mut() else {
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
    player.translation += direction * PLAYER_SPEED * time.delta_seconds();
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

fn animate_hit_flash(
    mut commands: Commands,
    time: Res<Time>,
    mut player_query: Query<(Entity, &mut Visibility, &mut HitFlash), With<Player>>,
) {
    let Ok((entity, mut visibility, mut flash)) = player_query.get_single_mut() else {
        return;
    };

    flash.timer.tick(time.delta());
    let blink = (flash.timer.elapsed_secs() * 14.0) as u32 % 2 == 0;
    *visibility = if blink {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };

    if flash.timer.finished() {
        *visibility = Visibility::Visible;
        commands.entity(entity).remove::<HitFlash>();
    }
}
