pub fn random_01(seed: f32) -> f32 {
    (seed.sin() * 43758.547).abs().fract()
}

pub fn random_range(seed: f32, min: f32, max: f32) -> f32 {
    min + random_01(seed) * (max - min)
}
