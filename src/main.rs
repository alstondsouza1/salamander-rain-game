use bevy::prelude::*;

// Components
#[derive(Component)] struct Player;
#[derive(Component)] struct Bug;
#[derive(Component)] struct Raindrop;
#[derive(Component)] struct ScoreText;
#[derive(Component)] struct LivesText;
#[derive(Component)] struct GameOverText;
#[derive(Component)] struct InstructionText;
#[derive(Component)] struct TitleText;

// Resources
#[derive(Resource)] struct Score(u32);
#[derive(Resource)] struct Lives(u32);
#[derive(Resource)] struct RainTimer(Timer);
#[derive(Resource)] struct GameOverTimer(Timer);
#[derive(Resource)] struct HitCooldown(Timer);

// Bug positions (reused for restart)
const BUG_POSITIONS: [Vec3; 5] = [
    Vec3::new(200.0, 0.0, 1.0),
    Vec3::new(-200.0, 100.0, 1.0),
    Vec3::new(0.0, -150.0, 1.0),
    Vec3::new(150.0, 200.0, 1.0),
    Vec3::new(-150.0, -200.0, 1.0),
];

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.05, 0.08, 0.2)))
        .add_plugins(DefaultPlugins)
        .insert_resource(Score(0))
        .insert_resource(Lives(3))
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

    // Title
    commands.spawn((
        TextBundle::from_section(
            "Salamander Rain Dash",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 40.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(220.0),
            ..default()
        }),
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
            ..default()
        },
        Player,
    ));

    spawn_bugs(&mut commands);

    // Score
    commands.spawn((
        TextBundle::from_section(
            "Score: 0",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 30.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(60.0),
            left: Val::Px(10.0),
            ..default()
        }),
        ScoreText,
    ));

    // Lives
    commands.spawn((
        TextBundle::from_section(
            "Lives: 3",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 30.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(100.0),
            left: Val::Px(10.0),
            ..default()
        }),
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
        TextBundle::from_section(
            "WASD to move • collect bugs • avoid rain • Press R to restart",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 22.0,
                color: Color::GRAY,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            left: Val::Px(80.0),
            ..default()
        }),
        InstructionText,
    ));
}

// Spawn bugs helper
fn spawn_bugs(commands: &mut Commands) {
    for pos in BUG_POSITIONS {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::YELLOW,
                    custom_size: Some(Vec2::new(30.0, 30.0)),
                    ..default()
                },
                transform: Transform::from_translation(pos),
                ..default()
            },
            Bug,
        ));
    }
}

// Movement
fn player_movement(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut t = query.single_mut();
    let mut dir = Vec3::ZERO;

    if input.pressed(KeyCode::KeyW) { dir.y += 1.0; }
    if input.pressed(KeyCode::KeyS) { dir.y -= 1.0; }
    if input.pressed(KeyCode::KeyA) { dir.x -= 1.0; }
    if input.pressed(KeyCode::KeyD) { dir.x += 1.0; }

    t.translation += dir.normalize_or_zero() * 300.0 * time.delta_seconds();

    t.translation.x = t.translation.x.clamp(-380.0, 380.0);
    t.translation.y = t.translation.y.clamp(-280.0, 280.0);
}

// Bug collection
fn collect_bug(
    mut commands: Commands,
    mut score: ResMut<Score>,
    player: Query<&Transform, With<Player>>,
    bugs: Query<(Entity, &Transform), With<Bug>>,
) {
    let p = player.single();
    for (e, t) in bugs.iter() {
        if p.translation.distance(t.translation) < 45.0 {
            commands.entity(e).despawn();
            score.0 += 1;
        }
    }
}

// Score UI
fn update_score_text(score: Res<Score>, mut q: Query<&mut Text, With<ScoreText>>) {
    if score.is_changed() {
        q.single_mut().sections[0].value = format!("Score: {}", score.0);
    }
}

// Lives UI
fn update_lives_text(lives: Res<Lives>, mut q: Query<&mut Text, With<LivesText>>) {
    if lives.is_changed() {
        q.single_mut().sections[0].value = format!("Lives: {}", lives.0);
    }
}

// Rain spawning
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
                    color: Color::rgb(0.3, 0.5, 1.0),
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

// Rain movement
fn move_raindrops(
    mut commands: Commands,
    mut q: Query<(Entity, &mut Transform), With<Raindrop>>,
    time: Res<Time>,
) {
    for (e, mut t) in q.iter_mut() {
        t.translation.y -= 400.0 * time.delta_seconds();

        if t.translation.y < -350.0 {
            commands.entity(e).despawn();
        }
    }
}

// Collision (fixed conflict!)
fn check_rain_collision(
    mut player: Query<&mut Transform, With<Player>>,
    rain: Query<&Transform, (With<Raindrop>, Without<Player>)>,
    mut text: Query<&mut Text, With<GameOverText>>,
    mut vis: Query<&mut Visibility, With<GameOverText>>,
    mut lives: ResMut<Lives>,
    mut cooldown: ResMut<HitCooldown>,
    mut timer: ResMut<GameOverTimer>,
) {
    if !cooldown.0.finished() {
        return;
    }

    let mut pt = player.single_mut();

    for rt in rain.iter() {
        if pt.translation.distance(rt.translation) < 35.0 {
            pt.translation = Vec3::ZERO;

            if lives.0 > 0 {
                lives.0 -= 1;
            }

            let mut t = text.single_mut();

            if lives.0 > 0 {
                t.sections[0].value = "Game Over! Avoid the rain!".into();
            } else {
                t.sections[0].value = "You lost! Press R to restart.".into();
            }

            *vis.single_mut() = Visibility::Visible;
            timer.0.reset();
            cooldown.0.reset();

            break;
        }
    }
}

// Cooldown tick
fn tick_hit_cooldown(time: Res<Time>, mut cooldown: ResMut<HitCooldown>) {
    cooldown.0.tick(time.delta());
}

// Hide text
fn update_game_over_text(
    time: Res<Time>,
    mut timer: ResMut<GameOverTimer>,
    mut vis: Query<&mut Visibility, With<GameOverText>>,
    lives: Res<Lives>,
) {
    timer.0.tick(time.delta());
    if timer.0.finished() && lives.0 > 0 {
        *vis.single_mut() = Visibility::Hidden;
    }
}

// ✅ Restart system
fn handle_restart(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut score: ResMut<Score>,
    mut lives: ResMut<Lives>,
    mut player: Query<&mut Transform, With<Player>>,
    mut vis: Query<&mut Visibility, With<GameOverText>>,
    rain: Query<Entity, With<Raindrop>>,
    bugs: Query<Entity, With<Bug>>,
) {
    if lives.0 > 0 {
        return;
    }

    if keyboard.just_pressed(KeyCode::KeyR) {
        score.0 = 0;
        lives.0 = 3;

        player.single_mut().translation = Vec3::ZERO;
        *vis.single_mut() = Visibility::Hidden;

        for e in rain.iter() {
            commands.entity(e).despawn();
        }

        for e in bugs.iter() {
            commands.entity(e).despawn();
        }

        spawn_bugs(&mut commands);
    }
}
