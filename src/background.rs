use bevy::prelude::*;

use crate::{game::GameState, level::CurrentLevel};

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
struct WetlandBackground;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_background)
            .add_systems(OnEnter(GameState::Playing), tint_for_level);
    }
}

fn setup_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2d, MainCamera));
    commands.spawn((
        Sprite {
            image: asset_server.load("backgrounds/wetland-night.png"),
            custom_size: Some(Vec2::new(960.0, 640.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -10.0),
        WetlandBackground,
    ));
}

fn tint_for_level(
    level: Res<CurrentLevel>,
    mut background: Query<&mut Sprite, With<WetlandBackground>>,
) {
    if let Ok(mut sprite) = background.single_mut() {
        sprite.color = level.definition().tint;
    }
}
