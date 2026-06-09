mod background;
mod constants;
mod effects;
mod firefly;
mod game;
mod level;
mod player;
mod rain;
mod ui;
mod util;

use background::BackgroundPlugin;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use constants::NIGHT_SKY;
use effects::EffectsPlugin;
use firefly::FireflyPlugin;
use game::{load_high_score, GamePlugin, GameState, HighScore};
use level::CurrentLevel;
use player::PlayerPlugin;
use rain::RainPlugin;
use ui::UiPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(NIGHT_SKY))
        .insert_resource(HighScore(load_high_score()))
        .init_resource::<CurrentLevel>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Salamander Rain Dash".into(),
                resolution: WindowResolution::new(960, 640),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .add_plugins((
            GamePlugin,
            BackgroundPlugin,
            PlayerPlugin,
            FireflyPlugin,
            RainPlugin,
            EffectsPlugin,
            UiPlugin,
        ))
        .run();
}
