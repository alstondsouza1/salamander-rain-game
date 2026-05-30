use bevy::prelude::*;

// Components
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Bug;

#[derive(Component)]
struct Raindrop;

// Resource to track score
#[derive(Resource)]
struct Score(u32);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Score(0))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                player_movement,
                collect_bug,
                spawn_raindrops,
                move_raindrops,
                check_rain_collision,
            ),
        )
        .run();
}

// Setup scene
fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Player
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

    // Bug
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::YELLOW,
                custom_size: Some(Vec2::new(30.0, 30.0)),
                ..default()
            },
            transform: Transform::from_xyz(200.0, 0.0, 0.0),
            ..default()
        },
        Bug,
    ));
}

// PLAYER MOVEMENT
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

// BUG COLLECTION
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

        if distance < 40.0 {
            commands.entity(entity).despawn();
            score.0 += 1;
            println!("Score: {}", score.0);
        }
    }
}

// SPAWN RAIN
fn spawn_raindrops(mut commands: Commands, time: Res<Time>) {
    // simple timer using elapsed time
    if (time.elapsed_seconds() * 5.0) as i32 % 10 == 0 {
        let x = (time.elapsed_seconds().sin() * 300.0) as f32;

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(0.3, 0.5, 1.0, 1.0),
                    custom_size: Some(Vec2::new(10.0, 20.0)),
                    ..default()
                },
                transform: Transform::from_xyz(x, 300.0, 0.0),
                ..default()
            },
            Raindrop,
        ));
    }
}

// MOVE RAIN DOWN
fn move_raindrops(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform), With<Raindrop>>,
    time: Res<Time>,
) {
    for (entity, mut transform) in query.iter_mut() {
        transform.translation.y -= 400.0 * time.delta_seconds();

        // Remove if off screen
        if transform.translation.y < -350.0 {
            commands.entity(entity).despawn();
        }
    }
}

// COLLISION WITH PLAYER
fn check_rain_collision(
    mut player_query: Query<&mut Transform, (With<Player>, Without<Raindrop>)>,
    rain_query: Query<&Transform, (With<Raindrop>, Without<Player>)>,
) {
    let mut player_transform = player_query.single_mut();

    for rain_transform in rain_query.iter() {
        let distance = player_transform
            .translation
            .distance(rain_transform.translation);

        if distance < 30.0 {
            println!("Game over!");

            // Reset player to center
            player_transform.translation = Vec3::ZERO;

            break;
        }
    }
}