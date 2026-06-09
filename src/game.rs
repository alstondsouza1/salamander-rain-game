use std::{fs, time::Duration};

use bevy::prelude::*;

use crate::{
    firefly::{spawn_fireflies, FireflyAtlas},
    level::CurrentLevel,
    player::{spawn_player, PlayerAtlas},
    rain::RainTimer,
    ui::spawn_hud,
};

const HIGH_SCORE_FILE: &str = "high_score.txt";

#[derive(States, Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    Menu,
    Playing,
    Paused,
    Won,
    Lost,
}

#[derive(Component)]
pub struct GameplayEntity;

#[derive(Resource, Default)]
pub struct Score(pub u32);

#[derive(Resource)]
pub struct HighScore(pub u32);

#[derive(Resource)]
pub struct Lives(pub u32);

#[derive(Resource, Default)]
pub struct SessionActive(pub bool);

#[derive(Resource)]
pub struct HitCooldown(pub Timer);

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        let mut hit_cooldown = Timer::from_seconds(1.0, TimerMode::Once);
        hit_cooldown.set_elapsed(Duration::from_secs_f32(1.0));

        app.init_resource::<Score>()
            .init_resource::<SessionActive>()
            .insert_resource(Lives(3))
            .insert_resource(HitCooldown(hit_cooldown))
            .add_systems(OnEnter(GameState::Playing), start_round_if_needed)
            .add_systems(OnEnter(GameState::Menu), leave_session)
            .add_systems(
                Update,
                tick_hit_cooldown.run_if(in_state(GameState::Playing)),
            );
    }
}

pub fn load_high_score() -> u32 {
    fs::read_to_string(HIGH_SCORE_FILE)
        .ok()
        .and_then(|value| value.trim().parse().ok())
        .unwrap_or(0)
}

pub fn save_high_score(score: u32) {
    if let Err(error) = fs::write(HIGH_SCORE_FILE, score.to_string()) {
        warn!("Could not save high score: {error}");
    }
}

#[allow(clippy::too_many_arguments)]
fn start_round_if_needed(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut session: ResMut<SessionActive>,
    mut score: ResMut<Score>,
    mut lives: ResMut<Lives>,
    mut cooldown: ResMut<HitCooldown>,
    mut rain_timer: ResMut<RainTimer>,
    high_score: Res<HighScore>,
    level: Res<CurrentLevel>,
    player_atlas: Res<PlayerAtlas>,
    firefly_atlas: Res<FireflyAtlas>,
    old_entities: Query<Entity, With<GameplayEntity>>,
) {
    if session.0 {
        return;
    }

    for entity in &old_entities {
        commands.entity(entity).despawn();
    }

    let definition = level.definition();
    score.0 = 0;
    lives.0 = definition.starting_lives;
    rain_timer.0 = Timer::from_seconds(definition.rain_interval, TimerMode::Repeating);
    let cooldown_duration = cooldown.0.duration();
    cooldown.0.set_elapsed(cooldown_duration);

    spawn_hud(
        &mut commands,
        &asset_server,
        high_score.0,
        level.0,
        definition,
    );
    spawn_player(&mut commands, &asset_server, player_atlas.0.clone());
    spawn_fireflies(
        &mut commands,
        &asset_server,
        firefly_atlas.0.clone(),
        definition.firefly_goal,
    );
    session.0 = true;
}

fn leave_session(
    mut commands: Commands,
    mut session: ResMut<SessionActive>,
    mut level: ResMut<CurrentLevel>,
    entities: Query<Entity, With<GameplayEntity>>,
) {
    session.0 = false;
    level.0 = 0;
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}

fn tick_hit_cooldown(time: Res<Time>, mut cooldown: ResMut<HitCooldown>) {
    cooldown.0.tick(time.delta());
}
