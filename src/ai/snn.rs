use std::sync::Arc;

pub struct SNNCore;

impl SNNCore {
    pub fn new() -> Self { Self {} }

    pub async fn deterministic_forward(&self, _input: f32, height: u64) -> f32 {
        let seed = height as f64;
        let spike = ((seed * 7.77).sin() * 1000.0) as f32;
        (spike.abs() % 100.0) + 1.0
    }
}
