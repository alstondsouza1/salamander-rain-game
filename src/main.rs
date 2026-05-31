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
#[derive(Resource)] struct GameWon(bool);

// 10 bug positions
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
        TextBundle::from_section(
            "Salamander Rain Dash",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 40.0,
                color: Color::WHITE,
            },
        ).with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(220.0),
            ..default()
        }),
        TitleText,
    ));

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

    commands.spawn((
        TextBundle::from_section(
            "Score: 0",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 30.0,
                color: Color::WHITE,
            },
        ).with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(60.0),
            left: Val::Px(10.0),
            ..default()
        }),
        ScoreText,
    ));

    commands.spawn((
        TextBundle::from_section(
            "Lives: 3",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 30.0,
                color: Color::WHITE,
            },
        ).with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(100.0),
            left: Val::Px(10.0),
            ..default()
        }),
        LivesText,
    ));

    commands.spawn((
        TextBundle {
            text: Text::from_section(
                "",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 36.0,
                    color: Color::WHITE,
                },
            ),
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(150.0),
                left: Val::Px(140.0),
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        },
        GameOverText,
    ));

    commands.spawn((
        TextBundle::from_section(
            "WASD to move • collect bugs • avoid rain • Press R",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 22.0,
                color: Color::GRAY,
            },
        ).with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            left: Val::Px(80.0),
            ..default()
        }),
        InstructionText,
    ));
}

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
    mut q: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut t = q.single_mut();
    let mut dir = Vec3::ZERO;

    if input.pressed(KeyCode::KeyW) { dir.y += 1.0; }
    if input.pressed(KeyCode::KeyS) { dir.y -= 1.0; }
    if input.pressed(KeyCode::KeyA) { dir.x -= 1.0; }
    if input.pressed(KeyCode::KeyD) { dir.x += 1.0; }

    t.translation += dir.normalize_or_zero() * 300.0 * time.delta_seconds();
}

// Collect bugs
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

fn check_win_condition(
    score: Res<Score>,
    mut text: Query<&mut Text, With<GameOverText>>,
    mut vis: Query<&mut Visibility, With<GameOverText>>,
    mut won: ResMut<GameWon>,
) {
    if score.0 >= 10 && !won.0 {
        let mut t = text.single_mut();
        t.sections[0].value = "You saved the salamander! Press R.".into();
        t.sections[0].style.color = Color::GREEN;
        *vis.single_mut() = Visibility::Visible;
        won.0 = true;
    }
}

// ✅ UPDATED: speed scales with score
fn move_raindrops(
    mut commands: Commands,
    mut q: Query<(Entity, &mut Transform), With<Raindrop>>,
    time: Res<Time>,
    score: Res<Score>, // ✅ NEW
) {
    let speed = 400.0 + score.0 as f32 * 30.0;

    for (e, mut t) in q.iter_mut() {
        t.translation.y -= speed * time.delta_seconds();

        if t.translation.y < -350.0 {
            commands.entity(e).despawn();
        }
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

fn check_rain_collision(
    mut player: Query<&mut Transform, With<Player>>,
    rain: Query<&Transform, (With<Raindrop>, Without<Player>)>,
    mut text: Query<&mut Text, With<GameOverText>>,
    mut vis: Query<&mut Visibility, With<GameOverText>>,
    mut lives: ResMut<Lives>,
    mut cooldown: ResMut<HitCooldown>,
    mut timer: ResMut<GameOverTimer>,
    won: Res<GameWon>,
) {
    if !cooldown.0.finished() || won.0 { return; }

    let mut pt = player.single_mut();

    for rt in rain.iter() {
        if pt.translation.distance(rt.translation) < 35.0 {
            pt.translation = Vec3::ZERO;

            if lives.0 > 0 { lives.0 -= 1; }

            if lives.0 == 0 {
                let mut t = text.single_mut();
                t.sections[0].value = "You lost! Press R.".into();
                t.sections[0].style.color = Color::RED;
                *vis.single_mut() = Visibility::Visible;
            }

            timer.0.reset();
            cooldown.0.reset();
            break;
        }
    }
}

fn update_score_text(score: Res<Score>, mut q: Query<&mut Text, With<ScoreText>>) {
    if score.is_changed() {
        q.single_mut().sections[0].value = format!("Score: {}", score.0);
    }
}

fn update_lives_text(lives: Res<Lives>, mut q: Query<&mut Text, With<LivesText>>) {
    if lives.is_changed() {
        q.single_mut().sections[0].value = format!("Lives: {}", lives.0);
    }
}

fn tick_hit_cooldown(time: Res<Time>, mut cooldown: ResMut<HitCooldown>) {
    cooldown.0.tick(time.delta());
}

fn update_game_over_text(
    time: Res<Time>,
    mut timer: ResMut<GameOverTimer>,
    mut vis: Query<&mut Visibility, With<GameOverText>>,
    lives: Res<Lives>,
    won: Res<GameWon>,
) {
    timer.0.tick(time.delta());
    if timer.0.finished() && lives.0 > 0 && !won.0 {
        *vis.single_mut() = Visibility::Hidden;
    }
}

// Restart
fn handle_restart(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut score: ResMut<Score>,
    mut lives: ResMut<Lives>,
    mut won: ResMut<GameWon>,
    mut player: Query<&mut Transform, With<Player>>,
    mut vis: Query<&mut Visibility, With<GameOverText>>,
    rain: Query<Entity, With<Raindrop>>,
    bugs: Query<Entity, With<Bug>>,
) {
    if input.just_pressed(KeyCode::KeyR) && (lives.0 == 0 || won.0) {
        score.0 = 0;
        lives.0 = 3;
        won.0 = false;

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