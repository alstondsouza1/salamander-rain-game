use bevy::prelude::*;

use crate::{
    constants::{FIREFLY_YELLOW, SALAMANDER_ORANGE, WIN_SCORE},
    game::{GameState, GameplayEntity, HighScore, Lives, Score},
};

#[derive(Component)]
struct ScreenEntity;

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct HighScoreText;

#[derive(Component)]
struct LivesText;

#[derive(Component)]
struct DifficultyText;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), show_menu)
            .add_systems(OnExit(GameState::Menu), clear_screen)
            .add_systems(OnEnter(GameState::Paused), show_pause)
            .add_systems(OnExit(GameState::Paused), clear_screen)
            .add_systems(OnEnter(GameState::Won), show_win)
            .add_systems(OnExit(GameState::Won), clear_screen)
            .add_systems(OnEnter(GameState::Lost), show_loss)
            .add_systems(OnExit(GameState::Lost), clear_screen)
            .add_systems(
                Update,
                (
                    menu_input.run_if(in_state(GameState::Menu)),
                    playing_input.run_if(in_state(GameState::Playing)),
                    pause_input.run_if(in_state(GameState::Paused)),
                    end_screen_input
                        .run_if(in_state(GameState::Won).or_else(in_state(GameState::Lost))),
                    update_hud,
                ),
            );
    }
}

pub fn spawn_hud(commands: &mut Commands, asset_server: &AssetServer, high_score: u32) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    spawn_text(
        commands,
        "SALAMANDER RAIN DASH",
        font.clone(),
        34.0,
        Color::rgb(0.92, 0.96, 0.90),
        Style {
            position_type: PositionType::Absolute,
            top: Val::Px(14.0),
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            ..default()
        },
        None,
    );

    let score = spawn_text(
        commands,
        &format!("FIREFLIES  0 / {WIN_SCORE}"),
        font.clone(),
        23.0,
        FIREFLY_YELLOW,
        Style {
            position_type: PositionType::Absolute,
            top: Val::Px(22.0),
            left: Val::Px(22.0),
            ..default()
        },
        None,
    );
    commands.entity(score).insert(ScoreText);

    let best = spawn_text(
        commands,
        &format!("BEST  {high_score}"),
        font.clone(),
        18.0,
        Color::rgb(0.62, 0.72, 0.69),
        Style {
            position_type: PositionType::Absolute,
            top: Val::Px(51.0),
            left: Val::Px(22.0),
            ..default()
        },
        None,
    );
    commands.entity(best).insert(HighScoreText);

    let lives = spawn_text(
        commands,
        "LIVES  \u{2665} \u{2665} \u{2665}",
        font.clone(),
        23.0,
        SALAMANDER_ORANGE,
        Style {
            position_type: PositionType::Absolute,
            top: Val::Px(22.0),
            right: Val::Px(22.0),
            ..default()
        },
        None,
    );
    commands.entity(lives).insert(LivesText);

    let difficulty = spawn_text(
        commands,
        "STORM  CALM",
        font.clone(),
        17.0,
        Color::rgb(0.45, 0.72, 0.92),
        Style {
            position_type: PositionType::Absolute,
            top: Val::Px(52.0),
            right: Val::Px(22.0),
            ..default()
        },
        None,
    );
    commands.entity(difficulty).insert(DifficultyText);

    spawn_text(
        commands,
        "WASD / ARROWS TO MOVE     P / ESC TO PAUSE",
        font,
        16.0,
        Color::rgb(0.54, 0.64, 0.62),
        Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            ..default()
        },
        None,
    );
}

fn spawn_text(
    commands: &mut Commands,
    value: &str,
    font: Handle<Font>,
    size: f32,
    color: Color,
    style: Style,
    marker: Option<ScreenEntity>,
) -> Entity {
    let mut entity = commands.spawn((
        TextBundle {
            text: Text::from_section(
                value,
                TextStyle {
                    font,
                    font_size: size,
                    color,
                },
            )
            .with_justify(JustifyText::Center),
            style,
            ..default()
        },
        GameplayEntity,
    ));
    if let Some(marker) = marker {
        entity.insert(marker);
    }
    entity.id()
}

fn show_menu(mut commands: Commands, asset_server: Res<AssetServer>, high_score: Res<HighScore>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    spawn_screen_text(
        &mut commands,
        "SALAMANDER\nRAIN DASH",
        font.clone(),
        58.0,
        FIREFLY_YELLOW,
        110.0,
    );
    spawn_screen_text(
        &mut commands,
        &format!(
            "Collect all {WIN_SCORE} fireflies before the storm takes your lives.\n\n\
             WASD or Arrow Keys  -  Move\nP or Escape  -  Pause\n\n\
             BEST SCORE: {}\n\nPRESS ENTER TO START",
            high_score.0
        ),
        font,
        22.0,
        Color::rgb(0.88, 0.94, 0.90),
        285.0,
    );
}

fn show_pause(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_overlay(&mut commands);
    spawn_screen_text(
        &mut commands,
        "PAUSED\n\nP / ESC  -  RESUME\nM  -  MAIN MENU",
        asset_server.load("fonts/FiraSans-Bold.ttf"),
        30.0,
        Color::WHITE,
        210.0,
    );
}

fn show_win(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_overlay(&mut commands);
    spawn_screen_text(
        &mut commands,
        "THE FIREFLIES ARE SAFE!\n\nENTER / R  -  PLAY AGAIN\nM  -  MAIN MENU",
        asset_server.load("fonts/FiraSans-Bold.ttf"),
        30.0,
        FIREFLY_YELLOW,
        205.0,
    );
}

fn show_loss(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_overlay(&mut commands);
    spawn_screen_text(
        &mut commands,
        "THE STORM WON\n\nENTER / R  -  TRY AGAIN\nM  -  MAIN MENU",
        asset_server.load("fonts/FiraSans-Bold.ttf"),
        30.0,
        Color::rgb(1.0, 0.35, 0.25),
        205.0,
    );
}

fn spawn_overlay(commands: &mut Commands) {
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            background_color: BackgroundColor(Color::rgba(0.01, 0.02, 0.04, 0.82)),
            ..default()
        },
        ScreenEntity,
    ));
}

fn spawn_screen_text(
    commands: &mut Commands,
    value: &str,
    font: Handle<Font>,
    size: f32,
    color: Color,
    top: f32,
) {
    commands.spawn((
        TextBundle {
            text: Text::from_section(
                value,
                TextStyle {
                    font,
                    font_size: size,
                    color,
                },
            )
            .with_justify(JustifyText::Center),
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(top),
                width: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        },
        ScreenEntity,
    ));
}

fn clear_screen(mut commands: Commands, query: Query<Entity, With<ScreenEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn menu_input(input: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if input.just_pressed(KeyCode::Enter) || input.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Playing);
    }
}

fn playing_input(input: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if input.just_pressed(KeyCode::KeyP) || input.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Paused);
    }
}

fn pause_input(input: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if input.just_pressed(KeyCode::KeyP) || input.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Playing);
    } else if input.just_pressed(KeyCode::KeyM) {
        next_state.set(GameState::Menu);
    }
}

fn end_screen_input(
    input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if input.just_pressed(KeyCode::Enter) || input.just_pressed(KeyCode::KeyR) {
        next_state.set(GameState::Playing);
    } else if input.just_pressed(KeyCode::KeyM) {
        next_state.set(GameState::Menu);
    }
}

fn update_hud(
    score: Res<Score>,
    high_score: Res<HighScore>,
    lives: Res<Lives>,
    mut text_queries: ParamSet<(
        Query<&mut Text, With<ScoreText>>,
        Query<&mut Text, With<HighScoreText>>,
        Query<&mut Text, With<LivesText>>,
        Query<&mut Text, With<DifficultyText>>,
    )>,
) {
    if score.is_changed() {
        for mut text in text_queries.p0().iter_mut() {
            text.sections[0].value = format!("FIREFLIES  {} / {}", score.0, WIN_SCORE);
        }
        for mut text in text_queries.p3().iter_mut() {
            let label = match score.0 {
                0..=4 => "CALM",
                5..=9 => "RISING",
                10..=14 => "HEAVY",
                _ => "FIERCE",
            };
            text.sections[0].value = format!("STORM  {label}");
        }
    }

    if high_score.is_changed() {
        for mut text in text_queries.p1().iter_mut() {
            text.sections[0].value = format!("BEST  {}", high_score.0);
        }
    }

    if lives.is_changed() {
        for mut text in text_queries.p2().iter_mut() {
            let hearts = "\u{2665} ".repeat(lives.0 as usize);
            text.sections[0].value = format!("LIVES  {}", hearts.trim_end());
        }
    }
}
