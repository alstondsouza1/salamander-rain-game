use bevy::prelude::*;

// marker component for the player
#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, player_movement)
        .run();
}

// Runs once when the app starts
fn setup(mut commands: Commands) {
    // Camera (required to see anything)
    commands.spawn(Camera2dBundle::default());

    // Player (green square)
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
                custom_size: Some(Vec2::new(50.0, 50.0)), // 50x50 square
                ..default()
            },
            ..default()
        },
        Player,
    ));
}

// Runs every frame
fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut transform = query.single_mut();

    let mut direction = Vec3::ZERO;

    // WASD input
    if keyboard_input.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    let speed = 300.0;

    // Move player (frame-rate independent)
    transform.translation += direction.normalize_or_zero() * speed * time.delta_seconds();
}