use bevy::prelude::*;

use crate::{
    constants::{FIREFLY_YELLOW, SALAMANDER_ORANGE},
    game::{GameState, GameplayEntity, HighScore, Lives, Score},
    level::{CurrentLevel, LevelDefinition},
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
                    end_screen_input.run_if(end_screen_active),
                    update_hud,
                ),
            );
    }
}

pub fn spawn_hud(
    commands: &mut Commands,
    asset_server: &AssetServer,
    high_score: u32,
    level_index: usize,
    level: LevelDefinition,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    spawn_text(
        commands,
        &format!(
            "LEVEL {}  -  {}",
            level_index + 1,
            level.name.to_uppercase()
        ),
        font.clone(),
        30.0,
        Color::srgb(0.92, 0.96, 0.90),
        centered_top(14.0),
        false,
    );

    let score = spawn_text(
        commands,
        &format!("FIREFLIES  0 / {}", level.firefly_goal),
        font.clone(),
        23.0,
        FIREFLY_YELLOW,
        Node {
            position_type: PositionType::Absolute,
            top: px(22),
            left: px(22),
            ..default()
        },
        false,
    );
    commands.entity(score).insert(ScoreText);

    let best = spawn_text(
        commands,
        &format!("BEST  {high_score}"),
        font.clone(),
        18.0,
        Color::srgb(0.62, 0.72, 0.69),
        Node {
            position_type: PositionType::Absolute,
            top: px(51),
            left: px(22),
            ..default()
        },
        false,
    );
    commands.entity(best).insert(HighScoreText);

    let lives = spawn_text(
        commands,
        &format!(
            "LIVES  {}",
            "\u{2665} ".repeat(level.starting_lives as usize)
        ),
        font.clone(),
        23.0,
        SALAMANDER_ORANGE,
        Node {
            position_type: PositionType::Absolute,
            top: px(22),
            right: px(22),
            ..default()
        },
        false,
    );
    commands.entity(lives).insert(LivesText);

    let difficulty = spawn_text(
        commands,
        "STORM  CALM",
        font.clone(),
        17.0,
        Color::srgb(0.45, 0.72, 0.92),
        Node {
            position_type: PositionType::Absolute,
            top: px(52),
            right: px(22),
            ..default()
        },
        false,
    );
    commands.entity(difficulty).insert(DifficultyText);

    spawn_text(
        commands,
        "WASD / ARROWS TO MOVE     P / ESC TO PAUSE",
        font,
        16.0,
        Color::srgb(0.68, 0.76, 0.73),
        Node {
            position_type: PositionType::Absolute,
            bottom: px(12),
            width: percent(100),
            justify_content: JustifyContent::Center,
            ..default()
        },
        false,
    );
}

fn spawn_text(
    commands: &mut Commands,
    value: &str,
    font: Handle<Font>,
    size: f32,
    color: Color,
    node: Node,
    screen_entity: bool,
) -> Entity {
    let mut entity = commands.spawn((
        Text::new(value),
        TextFont {
            font,
            font_size: size,
            ..default()
        },
        TextColor(color),
        TextLayout::new_with_justify(Justify::Center),
        node,
    ));
    if screen_entity {
        entity.insert(ScreenEntity);
    } else {
        entity.insert(GameplayEntity);
    }
    entity.id()
}

fn centered_top(top: f32) -> Node {
    Node {
        position_type: PositionType::Absolute,
        top: px(top),
        width: percent(100),
        justify_content: JustifyContent::Center,
        ..default()
    }
}

fn show_menu(mut commands: Commands, asset_server: Res<AssetServer>, high_score: Res<HighScore>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    spawn_text(
        &mut commands,
        "SALAMANDER\nRAIN DASH",
        font.clone(),
        58.0,
        FIREFLY_YELLOW,
        centered_top(92.0),
        true,
    );
    spawn_text(
        &mut commands,
        &format!(
            "Three wetlands. One growing storm.\nCollect every firefly to advance.\n\n\
             WASD or Arrow Keys  -  Move\nP or Escape  -  Pause\n\n\
             BEST SCORE: {}\n\nPRESS ENTER TO START",
            high_score.0
        ),
        font,
        21.0,
        Color::srgb(0.88, 0.94, 0.90),
        centered_top(275.0),
        true,
    );
}

fn show_pause(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_overlay(&mut commands);
    spawn_text(
        &mut commands,
        "PAUSED\n\nP / ESC  -  RESUME\nM  -  MAIN MENU",
        asset_server.load("fonts/FiraSans-Bold.ttf"),
        30.0,
        Color::WHITE,
        centered_top(210.0),
        true,
    );
}

fn show_win(mut commands: Commands, asset_server: Res<AssetServer>, level: Res<CurrentLevel>) {
    spawn_overlay(&mut commands);
    let message = if level.is_final() {
        "ALL WETLANDS SAVED!\n\nENTER / R  -  PLAY FROM LEVEL 1\nM  -  MAIN MENU"
    } else {
        "LEVEL CLEAR!\n\nENTER / R  -  NEXT LEVEL\nM  -  MAIN MENU"
    };
    spawn_text(
        &mut commands,
        message,
        asset_server.load("fonts/FiraSans-Bold.ttf"),
        30.0,
        FIREFLY_YELLOW,
        centered_top(205.0),
        true,
    );
}

fn show_loss(mut commands: Commands, asset_server: Res<AssetServer>, level: Res<CurrentLevel>) {
    spawn_overlay(&mut commands);
    spawn_text(
        &mut commands,
        &format!(
            "THE STORM WON LEVEL {}\n\nENTER / R  -  TRY AGAIN\nM  -  MAIN MENU",
            level.0 + 1
        ),
        asset_server.load("fonts/FiraSans-Bold.ttf"),
        30.0,
        Color::srgb(1.0, 0.35, 0.25),
        centered_top(205.0),
        true,
    );
}

fn spawn_overlay(commands: &mut Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: percent(100),
            height: percent(100),
            ..default()
        },
        BackgroundColor(Color::srgba(0.01, 0.02, 0.04, 0.82)),
        ScreenEntity,
    ));
}

fn clear_screen(mut commands: Commands, query: Query<Entity, With<ScreenEntity>>) {
    for entity in &query {
        commands.entity(entity).despawn();
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

fn end_screen_active(state: Res<State<GameState>>) -> bool {
    matches!(state.get(), GameState::Won | GameState::Lost)
}

fn end_screen_input(
    input: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
    mut level: ResMut<CurrentLevel>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if input.just_pressed(KeyCode::Enter) || input.just_pressed(KeyCode::KeyR) {
        if *state.get() == GameState::Won {
            level.0 = if level.is_final() { 0 } else { level.0 + 1 };
        }
        next_state.set(GameState::Playing);
    } else if input.just_pressed(KeyCode::KeyM) {
        next_state.set(GameState::Menu);
    }
}

fn update_hud(
    score: Res<Score>,
    high_score: Res<HighScore>,
    lives: Res<Lives>,
    level: Res<CurrentLevel>,
    mut text_queries: ParamSet<(
        Query<&mut Text, With<ScoreText>>,
        Query<&mut Text, With<HighScoreText>>,
        Query<&mut Text, With<LivesText>>,
        Query<&mut Text, With<DifficultyText>>,
    )>,
) {
    if score.is_changed() {
        for mut text in &mut text_queries.p0() {
            *text = Text::new(format!(
                "FIREFLIES  {} / {}",
                score.0,
                level.definition().firefly_goal
            ));
        }
        for mut text in &mut text_queries.p3() {
            let progress = score.0 as f32 / level.definition().firefly_goal as f32;
            let label = if progress < 0.25 {
                "CALM"
            } else if progress < 0.5 {
                "RISING"
            } else if progress < 0.75 {
                "HEAVY"
            } else {
                "FIERCE"
            };
            *text = Text::new(format!("STORM  {label}"));
        }
    }

    if high_score.is_changed() {
        for mut text in &mut text_queries.p1() {
            *text = Text::new(format!("BEST  {}", high_score.0));
        }
    }

    if lives.is_changed() {
        for mut text in &mut text_queries.p2() {
            let hearts = "\u{2665} ".repeat(lives.0 as usize);
            *text = Text::new(format!("LIVES  {}", hearts.trim_end()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn final_level_is_last_definition() {
        let level = CurrentLevel(crate::level::LEVELS.len() - 1);
        assert!(level.is_final());
        assert_eq!(level.definition().name, "Tempest Grove");
    }
}
