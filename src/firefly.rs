use std::time::Duration;

use bevy::prelude::*;

use crate::{
    constants::{ATLAS_COLUMNS, ATLAS_FRAME_SIZE},
    effects::CollectionEffect,
    game::{save_high_score, GameState, GameplayEntity, HighScore, Score, SessionActive},
    level::CurrentLevel,
    player::Player,
    util::random_range,
};

#[derive(Component)]
pub struct Firefly;

#[derive(Component)]
struct FireflyAnimation(Timer);

#[derive(Resource)]
pub struct FireflyAtlas(pub Handle<TextureAtlasLayout>);

pub struct FireflyPlugin;

impl Plugin for FireflyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_firefly_atlas).add_systems(
            Update,
            (collect_firefly, animate_fireflies).run_if(in_state(GameState::Playing)),
        );
    }
}

fn load_firefly_atlas(mut commands: Commands, mut layouts: ResMut<Assets<TextureAtlasLayout>>) {
    commands.insert_resource(FireflyAtlas(layouts.add(TextureAtlasLayout::from_grid(
        ATLAS_FRAME_SIZE,
        ATLAS_COLUMNS,
        1,
        None,
        None,
    ))));
}

pub fn spawn_fireflies(
    commands: &mut Commands,
    asset_server: &AssetServer,
    atlas: Handle<TextureAtlasLayout>,
    count: u32,
) {
    let image = asset_server.load("sprites/firefly-flap.png");
    for _ in 0..count {
        commands.spawn((
            Sprite {
                image: image.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: atlas.clone(),
                    index: rand::random_range(0..ATLAS_COLUMNS as usize),
                }),
                custom_size: Some(Vec2::new(62.0, 82.0)),
                ..default()
            },
            Transform::from_xyz(
                random_range(-410.0, 410.0),
                random_range(-235.0, 230.0),
                1.0,
            ),
            Firefly,
            FireflyAnimation(Timer::new(
                Duration::from_millis(rand::random_range(90..160)),
                TimerMode::Repeating,
            )),
            GameplayEntity,
        ));
    }
}

fn collect_firefly(
    mut commands: Commands,
    mut effects: MessageWriter<CollectionEffect>,
    mut score: ResMut<Score>,
    mut high_score: ResMut<HighScore>,
    mut session: ResMut<SessionActive>,
    level: Res<CurrentLevel>,
    mut next_state: ResMut<NextState<GameState>>,
    player_query: Query<&Transform, With<Player>>,
    firefly_query: Query<(Entity, &Transform), With<Firefly>>,
) {
    let Ok(player) = player_query.single() else {
        return;
    };

    for (entity, transform) in &firefly_query {
        if player.translation.distance(transform.translation) < 38.0 {
            commands.entity(entity).despawn();
            effects.write(CollectionEffect(transform.translation));
            score.0 += 1;

            if score.0 > high_score.0 {
                high_score.0 = score.0;
                save_high_score(high_score.0);
            }

            if score.0 >= level.definition().firefly_goal {
                session.0 = false;
                next_state.set(GameState::Won);
            }
        }
    }
}

fn animate_fireflies(time: Res<Time>, mut query: Query<(&mut FireflyAnimation, &mut Sprite)>) {
    for (mut animation, mut sprite) in &mut query {
        animation.0.tick(time.delta());
        if animation.0.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = (atlas.index + 1) % ATLAS_COLUMNS as usize;
            }
        }
    }
}
