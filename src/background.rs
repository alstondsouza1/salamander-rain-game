use bevy::prelude::*;

use crate::{
    constants::{DEEP_FOREST, MOSS},
    util::random_range,
};

#[derive(Component)]
pub struct MainCamera;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_background);
    }
}

fn setup_background(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: DEEP_FOREST,
            custom_size: Some(Vec2::new(960.0, 190.0)),
            ..default()
        },
        transform: Transform::from_xyz(0.0, -225.0, -4.0),
        ..default()
    });

    for i in 0..18 {
        let x = random_range(i as f32 * 12.7, -470.0, 470.0);
        let y = random_range(i as f32 * 31.1, -305.0, -135.0);
        let size = random_range(i as f32 * 8.3, 45.0, 110.0);

        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: MOSS,
                custom_size: Some(Vec2::splat(size)),
                ..default()
            },
            transform: Transform::from_xyz(x, y, -3.0)
                .with_rotation(Quat::from_rotation_z(i as f32 * 0.7)),
            ..default()
        });
    }

    for i in 0..32 {
        let x = random_range(i as f32 * 17.9, -460.0, 460.0);
        let y = random_range(i as f32 * 43.7, -80.0, 280.0);
        let size = random_range(i as f32 * 5.4, 1.0, 3.0);

        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.70, 0.83, 0.88, 0.38),
                custom_size: Some(Vec2::splat(size)),
                ..default()
            },
            transform: Transform::from_xyz(x, y, -3.0),
            ..default()
        });
    }
}
