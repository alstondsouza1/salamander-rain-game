use bevy::prelude::*;

// Components
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Bug;

#[derive(Component)]
struct Raindrop;

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct LivesText;

#[derive(Component)]
struct GameOverText;

#[derive(Component)]
struct InstructionText;

#[derive(Component)]
struct TitleText;

// Resources
#[derive(Resource)]
struct Score(u32);

#[derive(Resource)]
struct Lives(u32);

#[derive(Resource)]
struct RainTimer(Timer);

#[derive(Resource)]
struct GameOverTimer(Timer);

// prevents rapid multiple hits
#[derive(Resource)]
struct HitCooldown(Timer);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.05, 0.08, 0.2)))
        .add_plugins(DefaultPlugins)
        .insert_resource(Score(0))
        .insert_resource(Lives(3))
        .insert_resource(RainTimer(Timer::from_seconds(0.2, TimerMode::Repeating)))
        .insert_resource(GameOverTimer(Timer::from_seconds(2.0, TimerMode::Once)))
        .insert_resource(HitCooldown(Timer::from_seconds(1.0, TimerMode::Once))) // ✅ NEW
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
                update_lives_text,        
                update_game_over_text,
                tick_hit_cooldown,        
            ),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // Title
    commands.spawn((
        TextBundle {
            text: Text::from_section(
                "Salamander Rain Dash",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            ),
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(220.0),
                ..default()
            },
            ..default()
        },
        TitleText,
    ));

    // Player
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

    // Bugs
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

    // Score
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
                top: Val::Px(60.0),
                left: Val::Px(10.0),
                ..default()
            },
            ..default()
        },
        ScoreText,
    ));

    // Lives UI
    commands.spawn((
        TextBundle {
            text: Text::from_section(
                "Lives: 3",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ),
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(100.0),
                left: Val::Px(10.0),
                ..default()
            },
            ..default()
        },
        LivesText,
    ));

    // Game over text
    commands.spawn((
        TextBundle {
            text: Text::from_section(
                "Game Over! Avoid the rain!",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 36.0,
                    color: Color::RED,
                },
            ),
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(150.0),
                left: Val::Px(180.0),
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        },
        GameOverText,
    ));

    // Instructions
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
                left: Val::Px(120.0),
                ..default()
            },
            ..default()
        },
        InstructionText,
    ));
}

// Movement (unchanged)
fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut t = query.single_mut();
    let mut dir = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::KeyW) {
        dir.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        dir.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        dir.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        dir.x += 1.0;
    }

    t.translation += dir.normalize_or_zero() * 300.0 * time.delta_seconds();
    t.translation.x = t.translation.x.clamp(-380.0, 380.0);
    t.translation.y = t.translation.y.clamp(-280.0, 280.0);
}

// ✅ UPDATED: cooldown logic
fn check_rain_collision(
    mut player_query: Query<&mut Transform, (With<Player>, Without<Raindrop>)>,
    rain_query: Query<&Transform, With<Raindrop>>,
    mut text_query: Query<&mut Text, With<GameOverText>>,
    mut visibility_query: Query<&mut Visibility, With<GameOverText>>,
    mut game_over_timer: ResMut<GameOverTimer>,
    mut lives: ResMut<Lives>,
    mut cooldown: ResMut<HitCooldown>,
) {
    // Ignore hits if still cooling down
    if !cooldown.0.finished() {
        return;
    }

    let mut player_transform = player_query.single_mut();

    for rain_transform in rain_query.iter() {
        if player_transform
            .translation
            .distance(rain_transform.translation)
            < 35.0
        {
            player_transform.translation = Vec3::ZERO;

            if lives.0 > 0 {
                lives.0 -= 1;
            }

            let mut text = text_query.single_mut();
            let mut visibility = visibility_query.single_mut();

            if lives.0 > 0 {
                text.sections[0].value = "Game Over! Avoid the rain!".to_string();
            } else {
                text.sections[0].value =
                    "You lost! Restart the app to try again.".to_string();
            }

            *visibility = Visibility::Visible;
            game_over_timer.0.reset();

            // Start cooldown
            cooldown.0.reset();

            break;
        }
    }
}

// tick cooldown timer
fn tick_hit_cooldown(time: Res<Time>, mut cooldown: ResMut<HitCooldown>) {
    cooldown.0.tick(time.delta());
}

// Lives UI update
fn update_lives_text(lives: Res<Lives>, mut q: Query<&mut Text, With<LivesText>>) {
    if lives.is_changed() {
        q.single_mut().sections[0].value = format!("Lives: {}", lives.0);
    }
}

// (all other systems unchanged below)

fn collect_bug(
    mut commands: Commands,
    mut score: ResMut<Score>,
    player_query: Query<&Transform, With<Player>>,
    bug_query: Query<(Entity, &Transform), With<Bug>>,
) {
    let player_transform = player_query.single();

    for (entity, transform) in bug_query.iter() {
        if player_transform.translation.distance(transform.translation) < 45.0 {
            commands.entity(entity).despawn();
            score.0 += 1;
        }
    }
}

fn update_score_text(score: Res<Score>, mut q: Query<&mut Text, With<ScoreText>>) {
    if score.is_changed() {
        q.single_mut().sections[0].value = format!("Score: {}", score.0);
    }
}

fn spawn_raindrops(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<RainTimer>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
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
    mut query: Query<(Entity, &mut Transform), With<Raindrop>>,
    time: Res<Time>,
) {
    for (e, mut t) in query.iter_mut() {
        t.translation.y -= 400.0 * time.delta_seconds();
        if t.translation.y < -350.0 {
            commands.entity(e).despawn();
        }
    }
}

fn update_game_over_text(
    time: Res<Time>,
    mut timer: ResMut<GameOverTimer>,
    mut query: Query<&mut Visibility, With<GameOverText>>,
    lives: Res<Lives>,
) {
    timer.0.tick(time.delta());

    if timer.0.finished() && lives.0 > 0 {
        *query.single_mut() = Visibility::Hidden;
    }
}