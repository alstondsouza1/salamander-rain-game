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

#[derive(Resource, Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Difficulty {
    Easy,
    #[default]
    Normal,
    Hard,
}

impl Difficulty {
    /// Scales the level's base rain speed. < 1.0 slows rain, > 1.0 speeds it up.
    pub fn rain_speed_multiplier(&self) -> f32 {
        match self {
            Difficulty::Easy => 0.75,
            Difficulty::Normal => 1.0,
            Difficulty::Hard => 1.3,
        }
    }

    /// Scales the level's base rain interval. > 1.0 widens the gap between drops
    /// (less frequent rain), < 1.0 narrows it (more frequent rain).
    pub fn rain_interval_multiplier(&self) -> f32 {
        match self {
            Difficulty::Easy => 1.35,
            Difficulty::Normal => 1.0,
            Difficulty::Hard => 0.7,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Difficulty::Easy => "EASY",
            Difficulty::Normal => "NORMAL",
            Difficulty::Hard => "HARD",
        }
    }

    /// Cycles Easy -> Normal -> Hard -> Easy.
    pub fn next(&self) -> Difficulty {
        match self {
            Difficulty::Easy => Difficulty::Normal,
            Difficulty::Normal => Difficulty::Hard,
            Difficulty::Hard => Difficulty::Easy,
        }
    }
}

#[derive(Resource, Default)]
pub struct CurrentLevel(pub usize);

impl CurrentLevel {
    pub fn definition(&self) -> LevelDefinition {
        LEVELS[self.0.min(LEVELS.len() - 1)]
    }

    /// The current level's base definition with only rain speed and rain interval
    /// scaled by the chosen difficulty. All other fields are left untouched.
    pub fn definition_with(&self, difficulty: Difficulty) -> LevelDefinition {
        let base = self.definition();
        LevelDefinition {
            rain_interval: base.rain_interval * difficulty.rain_interval_multiplier(),
            rain_speed: base.rain_speed * difficulty.rain_speed_multiplier(),
            ..base
        }
    }

    pub fn is_final(&self) -> bool {
        self.0 + 1 >= LEVELS.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_difficulty_is_normal() {
        assert_eq!(Difficulty::default(), Difficulty::Normal);
    }

    #[test]
    fn speed_multipliers_are_ordered() {
        assert!(Difficulty::Easy.rain_speed_multiplier() < 1.0);
        assert_eq!(Difficulty::Normal.rain_speed_multiplier(), 1.0);
        assert!(Difficulty::Hard.rain_speed_multiplier() > 1.0);
        assert!(
            Difficulty::Easy.rain_speed_multiplier() < Difficulty::Hard.rain_speed_multiplier()
        );
    }

    #[test]
    fn interval_multipliers_are_inversely_ordered() {
        // Higher interval = less frequent rain. Easy is the least frequent.
        assert!(Difficulty::Easy.rain_interval_multiplier() > 1.0);
        assert_eq!(Difficulty::Normal.rain_interval_multiplier(), 1.0);
        assert!(Difficulty::Hard.rain_interval_multiplier() < 1.0);
        assert!(
            Difficulty::Hard.rain_interval_multiplier()
                < Difficulty::Easy.rain_interval_multiplier()
        );
    }

    #[test]
    fn labels_match_difficulties() {
        assert_eq!(Difficulty::Easy.label(), "EASY");
        assert_eq!(Difficulty::Normal.label(), "NORMAL");
        assert_eq!(Difficulty::Hard.label(), "HARD");
    }

    #[test]
    fn next_cycles_through_all_difficulties() {
        assert_eq!(Difficulty::Easy.next(), Difficulty::Normal);
        assert_eq!(Difficulty::Normal.next(), Difficulty::Hard);
        assert_eq!(Difficulty::Hard.next(), Difficulty::Easy);
    }

    #[test]
    fn easy_slows_and_thins_rain_relative_to_base() {
        let level = CurrentLevel(0);
        let base = level.definition();
        let easy = level.definition_with(Difficulty::Easy);
        assert!(easy.rain_speed < base.rain_speed);
        assert!(easy.rain_interval > base.rain_interval);
    }

    #[test]
    fn hard_speeds_and_thickens_rain_relative_to_base() {
        let level = CurrentLevel(0);
        let base = level.definition();
        let hard = level.definition_with(Difficulty::Hard);
        assert!(hard.rain_speed > base.rain_speed);
        assert!(hard.rain_interval < base.rain_interval);
    }

    #[test]
    fn normal_leaves_rain_values_unchanged() {
        let level = CurrentLevel(1);
        let base = level.definition();
        let normal = level.definition_with(Difficulty::Normal);
        assert_eq!(normal.rain_speed, base.rain_speed);
        assert_eq!(normal.rain_interval, base.rain_interval);
    }

    #[test]
    fn difficulty_only_affects_rain_fields() {
        let level = CurrentLevel(2);
        let base = level.definition();
        for difficulty in [Difficulty::Easy, Difficulty::Normal, Difficulty::Hard] {
            let scaled = level.definition_with(difficulty);
            assert_eq!(scaled.firefly_goal, base.firefly_goal);
            assert_eq!(scaled.starting_lives, base.starting_lives);
            assert_eq!(scaled.name, base.name);
            assert_eq!(scaled.tint, base.tint);
        }
    }
}
