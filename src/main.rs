use bevy::prelude::*;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Bug;

#[derive(Component)]
struct Raindrop;

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct GameOverText;

#[derive(Component)]
struct InstructionText;

#[derive(Resource)]
struct Score(u32);

#[derive(Resource)]
struct RainTimer(Timer);

#[derive(Resource)]
struct GameOverTimer(Timer);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.05, 0.08, 0.2)))
        .add_plugins(DefaultPlugins)
        .insert_resource(Score(0))
        .insert_resource(RainTimer(Timer::from_seconds(0.2, TimerMode::Repeating)))
        .insert_resource(GameOverTimer(Timer::from_seconds(2.0, TimerMode::Once)))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                player_movement,
                collect_bug,
                spawn_raindrops,
                move_raindrops,
                check_rain_collision,
                update_score_text,
                update_game_over_text,
            ),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        },
        Player,
    ));

    let bug_positions = [
        Vec3::new(200.0, 0.0, 1.0),
        Vec3::new(-200.0, 100.0, 1.0),
        Vec3::new(0.0, -150.0, 1.0),
        Vec3::new(150.0, 200.0, 1.0),
        Vec3::new(-150.0, -200.0, 1.0),
    ];

    for position in bug_positions {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::YELLOW,
                    custom_size: Some(Vec2::new(30.0, 30.0)),
                    ..default()
                },
                transform: Transform::from_translation(position),
                ..default()
            },
            Bug,
        ));
    }

    commands.spawn((
        TextBundle {
            text: Text::from_section(
                "Score: 0",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ),
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                ..default()
            },
            ..default()
        },
        ScoreText,
    ));

    commands.spawn((
        TextBundle {
            text: Text::from_section(
                "Game Over! Avoid the rain!",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::RED,
                },
            ),
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(100.0),
                left: Val::Px(200.0),
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        },
        GameOverText,
    ));

    commands.spawn((
        TextBundle {
            text: Text::from_section(
                "WASD to move • collect bugs • avoid rain",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 22.0,
                    color: Color::GRAY,
                },
            ),
            style: Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(10.0),
                left: Val::Px(100.0),
                ..default()
            },
            ..default()
        },
        InstructionText,
    ));
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut player_transform = player_query.single_mut();
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
    player_transform.translation += direction.normalize_or_zero() * speed * time.delta_seconds();

    player_transform.translation.x = player_transform.translation.x.clamp(-380.0, 380.0);
    player_transform.translation.y = player_transform.translation.y.clamp(-280.0, 280.0);
}

fn collect_bug(
    mut commands: Commands,
    mut score: ResMut<Score>,
    player_query: Query<&Transform, With<Player>>,
    bug_query: Query<(Entity, &Transform), With<Bug>>,
) {
    let player_transform = player_query.single();

    for (bug_entity, bug_transform) in bug_query.iter() {
        let distance = player_transform.translation.distance(bug_transform.translation);

        if distance < 45.0 {
            commands.entity(bug_entity).despawn();
            score.0 += 1;
            println!("Score: {}", score.0);
        }
    }
}

fn update_score_text(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    if score.is_changed() {
        let mut text = query.single_mut();
        text.sections[0].value = format!("Score: {}", score.0);
    }
}

fn spawn_raindrops(
    mut commands: Commands,
    time: Res<Time>,
    mut rain_timer: ResMut<RainTimer>,
) {
    rain_timer.0.tick(time.delta());

    if rain_timer.0.just_finished() {
        let x = time.elapsed_seconds().sin() * 350.0;

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(0.3, 0.5, 1.0, 1.0),
                    custom_size: Some(Vec2::new(12.0, 25.0)),
                    ..default()
                },
                transform: Transform::from_xyz(x, 320.0, 1.0),
                ..default()
            },
            Raindrop,
        ));
    }
}

fn move_raindrops(
    mut commands: Commands,
    mut rain_query: Query<(Entity, &mut Transform), With<Raindrop>>,
    time: Res<Time>,
) {
    for (rain_entity, mut rain_transform) in rain_query.iter_mut() {
        rain_transform.translation.y -= 400.0 * time.delta_seconds();

        if rain_transform.translation.y < -350.0 {
            commands.entity(rain_entity).despawn();
        }
    }
}

fn check_rain_collision(
    mut player_query: Query<&mut Transform, (With<Player>, Without<Raindrop>)>,
    rain_query: Query<&Transform, (With<Raindrop>, Without<Player>)>,
    mut text_query: Query<&mut Visibility, With<GameOverText>>,
    mut game_over_timer: ResMut<GameOverTimer>,
) {
    let mut player_transform = player_query.single_mut();

    for rain_transform in rain_query.iter() {
        let distance = player_transform.translation.distance(rain_transform.translation);

        if distance < 35.0 {
            println!("Game over!");
            player_transform.translation = Vec3::ZERO;
            *text_query.single_mut() = Visibility::Visible;
            game_over_timer.0.reset();
            break;
        }
    }
}

fn update_game_over_text(
    time: Res<Time>,
    mut timer: ResMut<GameOverTimer>,
    mut query: Query<&mut Visibility, With<GameOverText>>,
) {
    timer.0.tick(time.delta());

    if timer.0.finished() {
        *query.single_mut() = Visibility::Hidden;
    }
}