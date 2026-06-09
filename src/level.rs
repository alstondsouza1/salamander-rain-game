use bevy::prelude::*;

#[derive(Clone, Copy)]
pub struct LevelDefinition {
    pub name: &'static str,
    pub firefly_goal: u32,
    pub starting_lives: u32,
    pub rain_interval: f32,
    pub rain_speed: f32,
    pub tint: Color,
}

pub const LEVELS: [LevelDefinition; 3] = [
    LevelDefinition {
        name: "Firefly Marsh",
        firefly_goal: 12,
        starting_lives: 3,
        rain_interval: 0.28,
        rain_speed: 330.0,
        tint: Color::WHITE,
    },
    LevelDefinition {
        name: "Moonlit Fen",
        firefly_goal: 16,
        starting_lives: 3,
        rain_interval: 0.21,
        rain_speed: 410.0,
        tint: Color::srgb(0.76, 0.88, 1.0),
    },
    LevelDefinition {
        name: "Tempest Grove",
        firefly_goal: 20,
        starting_lives: 3,
        rain_interval: 0.15,
        rain_speed: 500.0,
        tint: Color::srgb(0.64, 0.72, 0.90),
    },
];

#[derive(Resource, Default)]
pub struct CurrentLevel(pub usize);

impl CurrentLevel {
    pub fn definition(&self) -> LevelDefinition {
        LEVELS[self.0.min(LEVELS.len() - 1)]
    }

    pub fn is_final(&self) -> bool {
        self.0 + 1 >= LEVELS.len()
    }
}
