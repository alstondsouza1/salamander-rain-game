use bevy::prelude::*;

// Components
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Bug;

// Resource to track score
#[derive(Resource)]
struct Score(u32);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Score(0))
        .add_systems(Startup, setup)
        .add_systems(Update, (player_movement, collect_bug))
        .run();
}

// Setup scene
fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Player (green square)
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            ..default()
        },
        Player,
    ));

    // Bug (yellow square)
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::YELLOW,
                custom_size: Some(Vec2::new(30.0, 30.0)),
                ..default()
            },
            transform: Transform::from_xyz(200.0, 0.0, 0.0), // placed to the right
            ..default()
        },
        Bug,
    ));
}

// Movement system
fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut transform = query.single_mut();

    let mut direction = Vec3::ZERO;

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

    transform.translation += direction.normalize_or_zero() * speed * time.delta_seconds();
}

// Collision + collect system
fn collect_bug(
    mut commands: Commands,
    mut score: ResMut<Score>,
    player_query: Query<&Transform, With<Player>>,
    bug_query: Query<(Entity, &Transform), With<Bug>>,
) {
    let player_transform = player_query.single();

    for (entity, bug_transform) in bug_query.iter() {
        let distance = player_transform
            .translation
            .distance(bug_transform.translation);

        // Simple collision check
        if distance < 40.0 {
            commands.entity(entity).despawn(); // remove bug
            score.0 += 1;

            println!("Score: {}", score.0);
        }
    }
}