use bevy::prelude::*;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Bug;

#[derive(Component)]
struct Glow;

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

#[derive(Resource)]
struct Score(u32);

#[derive(Resource)]
struct Lives(u32);

#[derive(Resource)]
struct RainTimer(Timer);

#[derive(Resource)]
struct GameOverTimer(Timer);

#[derive(Resource)]
struct HitCooldown(Timer);

#[derive(Resource)]
struct GameWon(bool);

const BUG_POSITIONS: [Vec3; 10] = [
    Vec3::new(-300.0, 200.0, 1.0),
    Vec3::new(-150.0, 220.0, 1.0),
    Vec3::new(0.0, 220.0, 1.0),
    Vec3::new(150.0, 220.0, 1.0),
    Vec3::new(300.0, 200.0, 1.0),
    Vec3::new(-250.0, 0.0, 1.0),
    Vec3::new(-100.0, -50.0, 1.0),
    Vec3::new(100.0, -50.0, 1.0),
    Vec3::new(250.0, 0.0, 1.0),
    Vec3::new(0.0, -220.0, 1.0),
];

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.05, 0.08, 0.2)))
        .add_plugins(DefaultPlugins)
        .insert_resource(Score(0))
        .insert_resource(Lives(3))
        .insert_resource(GameWon(false))
        .insert_resource(RainTimer(Timer::from_seconds(0.2, TimerMode::Repeating)))
        .insert_resource(GameOverTimer(Timer::from_seconds(2.0, TimerMode::Once)))
        .insert_resource(HitCooldown(Timer::from_seconds(1.0, TimerMode::Once)))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                player_movement,
                collect_bug,
                spawn_raindrops,
                move_raindrops,
                check_rain_collision,
                check_win_condition,
                update_score_text,
                update_lives_text,
                update_game_over_text,
                tick_hit_cooldown,
                handle_restart,
            ),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

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
                left: Val::Px(200.0),
                ..default()
            },
            ..default()
        },
        TitleText,
    ));

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

    commands.spawn((
        TextBundle {
            text: Text::from_section(
                "",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 34.0,
                    color: Color::WHITE,
                },
            ),
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(150.0),
                left: Val::Px(130.0),
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
                "WASD to move • collect 10 fireflies • avoid rain • Press R to restart",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::GRAY,
                },
            ),
            style: Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(10.0),
                left: Val::Px(45.0),
                ..default()
            },
            ..default()
        },
        InstructionText,
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 2.0),
            ..default()
        },
        Player,
    ));

    spawn_bugs(&mut commands);
}

fn spawn_bugs(commands: &mut Commands) {
    for pos in BUG_POSITIONS {
        commands
            .spawn((
                TransformBundle::from_transform(Transform::from_translation(pos)),
                Bug,
            ))
            .with_children(|parent| {
                parent.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgba(1.0, 1.0, 0.3, 0.25),
                            custom_size: Some(Vec2::new(60.0, 60.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(0.0, 0.0, 0.0),
                        ..default()
                    },
                    Glow,
                ));

                parent.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::YELLOW,
                        custom_size: Some(Vec2::new(30.0, 30.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 0.0, 1.0),
                    ..default()
                });
            });
    }
}

fn player_movement(
    input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    lives: Res<Lives>,
    won: Res<GameWon>,
) {
    if lives.0 == 0 || won.0 {
        return;
    }

    let mut player = player_query.single_mut();
    let mut direction = Vec3::ZERO;

    if input.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    player.translation += direction.normalize_or_zero() * 300.0 * time.delta_seconds();
    player.translation.x = player.translation.x.clamp(-380.0, 380.0);
    player.translation.y = player.translation.y.clamp(-280.0, 280.0);
}

fn collect_bug(
    mut commands: Commands,
    mut score: ResMut<Score>,
    player_query: Query<&Transform, With<Player>>,
    bug_query: Query<(Entity, &Transform), With<Bug>>,
    lives: Res<Lives>,
    won: Res<GameWon>,
) {
    if lives.0 == 0 || won.0 {
        return;
    }

    let player = player_query.single();

    for (bug_entity, bug_transform) in bug_query.iter() {
        if player.translation.distance(bug_transform.translation) < 45.0 {
            commands.entity(bug_entity).despawn_recursive();
            score.0 += 1;
        }
    }
}

fn spawn_raindrops(
    mut commands: Commands,
    time: Res<Time>,
    mut rain_timer: ResMut<RainTimer>,
    lives: Res<Lives>,
    won: Res<GameWon>,
) {
    if lives.0 == 0 || won.0 {
        return;
    }

    rain_timer.0.tick(time.delta());

    if rain_timer.0.just_finished() {
        let x = time.elapsed_seconds().sin() * 350.0;

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.3, 0.5, 1.0),
                    custom_size: Some(Vec2::new(12.0, 25.0)),
                    ..default()
                },
                transform: Transform::from_xyz(x, 320.0, 2.0),
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
    score: Res<Score>,
) {
    let speed = 400.0 + score.0 as f32 * 30.0;

    for (rain_entity, mut rain_transform) in rain_query.iter_mut() {
        rain_transform.translation.y -= speed * time.delta_seconds();

        if rain_transform.translation.y < -350.0 {
            commands.entity(rain_entity).despawn();
        }
    }
}

fn check_rain_collision(
    mut player_query: Query<&mut Transform, With<Player>>,
    rain_query: Query<&Transform, (With<Raindrop>, Without<Player>)>,
    mut text_query: Query<&mut Text, With<GameOverText>>,
    mut visibility_query: Query<&mut Visibility, With<GameOverText>>,
    mut lives: ResMut<Lives>,
    mut cooldown: ResMut<HitCooldown>,
    mut game_over_timer: ResMut<GameOverTimer>,
    won: Res<GameWon>,
) {
    if !cooldown.0.finished() || won.0 || lives.0 == 0 {
        return;
    }

    let mut player = player_query.single_mut();

    for rain in rain_query.iter() {
        if player.translation.distance(rain.translation) < 35.0 {
            player.translation = Vec3::ZERO;

            if lives.0 > 0 {
                lives.0 -= 1;
            }

            let mut text = text_query.single_mut();

            if lives.0 > 0 {
                text.sections[0].value = "Ouch! Avoid the rain!".to_string();
                text.sections[0].style.color = Color::ORANGE_RED;
            } else {
                text.sections[0].value = "You lost! Press R to restart.".to_string();
                text.sections[0].style.color = Color::RED;
            }

            *visibility_query.single_mut() = Visibility::Visible;
            game_over_timer.0.reset();
            cooldown.0.reset();

            break;
        }
    }
}

fn check_win_condition(
    score: Res<Score>,
    mut text_query: Query<&mut Text, With<GameOverText>>,
    mut visibility_query: Query<&mut Visibility, With<GameOverText>>,
    mut won: ResMut<GameWon>,
) {
    if score.0 >= 10 && !won.0 {
        let mut text = text_query.single_mut();
        text.sections[0].value = "You saved the salamander! Press R to play again.".to_string();
        text.sections[0].style.color = Color::GREEN;

        *visibility_query.single_mut() = Visibility::Visible;
        won.0 = true;
    }
}

fn update_score_text(score: Res<Score>, mut text_query: Query<&mut Text, With<ScoreText>>) {
    if score.is_changed() {
        text_query.single_mut().sections[0].value = format!("Score: {}", score.0);
    }
}

fn update_lives_text(lives: Res<Lives>, mut text_query: Query<&mut Text, With<LivesText>>) {
    if lives.is_changed() {
        text_query.single_mut().sections[0].value = format!("Lives: {}", lives.0);
    }
}

fn tick_hit_cooldown(time: Res<Time>, mut cooldown: ResMut<HitCooldown>) {
    cooldown.0.tick(time.delta());
}

fn update_game_over_text(
    time: Res<Time>,
    mut timer: ResMut<GameOverTimer>,
    mut visibility_query: Query<&mut Visibility, With<GameOverText>>,
    lives: Res<Lives>,
    won: Res<GameWon>,
) {
    timer.0.tick(time.delta());

    if timer.0.finished() && lives.0 > 0 && !won.0 {
        *visibility_query.single_mut() = Visibility::Hidden;
    }
}

fn handle_restart(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut score: ResMut<Score>,
    mut lives: ResMut<Lives>,
    mut won: ResMut<GameWon>,
    mut player_query: Query<&mut Transform, With<Player>>,
    mut visibility_query: Query<&mut Visibility, With<GameOverText>>,
    rain_query: Query<Entity, With<Raindrop>>,
    bug_query: Query<Entity, With<Bug>>,
) {
    if input.just_pressed(KeyCode::KeyR) && (lives.0 == 0 || won.0) {
        score.0 = 0;
        lives.0 = 3;
        won.0 = false;

        player_query.single_mut().translation = Vec3::ZERO;
        *visibility_query.single_mut() = Visibility::Hidden;

        for rain in rain_query.iter() {
            commands.entity(rain).despawn();
        }

        for bug in bug_query.iter() {
            commands.entity(bug).despawn_recursive();
        }

        spawn_bugs(&mut commands);
    }
}
