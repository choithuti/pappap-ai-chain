// src/ai/snn.rs
use serde::{Serialize, Deserialize};
use fixed::types::I48F16; // Số học dấu phẩy tĩnh 48.16 bit
use rayon::prelude::*;    // Xử lý song song
use parking_lot::RwLock;  // Mutex nhanh
use std::sync::Arc;

// Định nghĩa kiểu số Deterministic (Bất biến trên mọi máy)
pub type DNum = I48F16; 

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Neuron {
    pub potential: DNum,
    pub threshold: DNum,
    pub decay: DNum,
    pub weights: Vec<DNum>,
    pub last_spike_height: u64,
}

impl Neuron {
    pub fn new(input_size: usize) -> Self {
        let default_weight = DNum::from_num(0.01); 
        Self {
            potential: DNum::from_num(0),
            threshold: DNum::from_num(1.0),
            decay: DNum::from_num(0.9), 
            weights: vec![default_weight; input_size],
            last_spike_height: 0,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Layer {
    pub neurons: Vec<Neuron>,
}

impl Layer {
    pub fn new(size: usize, input_size: usize) -> Self {
        let neurons = (0..size).map(|_| Neuron::new(input_size)).collect();
        Self { neurons }
    }

    pub fn forward(&mut self, inputs: &[DNum], current_height: u64) -> Vec<DNum> {
        // Rayon par_iter_mut() chia việc cho CPU
        self.neurons.par_iter_mut().map(|n| {
            let mut activation = DNum::from_num(0);
            
            for (i, &input) in inputs.iter().enumerate() {
                if i < n.weights.len() {
                    activation += input * n.weights[i];
                }
            }

            n.potential += activation;

            if n.potential >= n.threshold {
                n.potential = DNum::from_num(0);
                n.last_spike_height = current_height;
                DNum::from_num(1) // SPIKE!
            } else {
                n.potential *= n.decay;
                DNum::from_num(0)
            }
        }).collect()
    }
}

pub struct SNN {
    pub layers: Arc<RwLock<Vec<Layer>>>,
}

impl SNN {
    pub fn new() -> Self {
        // Cấu trúc mạng: Input(64) -> Hidden(128) -> Output(10)
        let l1 = Layer::new(128, 64);
        let l2 = Layer::new(10, 128);

        Self {
            layers: Arc::new(RwLock::new(vec![l1, l2])),
        }
    }

    pub fn process(&self, inputs: Vec<DNum>, height: u64) -> Vec<DNum> {
        let mut current_input = inputs;
        let mut layers = self.layers.write();

        for layer in layers.iter_mut() {
            current_input = layer.forward(&current_input, height);
        }
        
        current_input
    }
}
