// src/ai/snn.rs
use serde::{Serialize, Deserialize};
use fixed::types::I48F16; // 48 bit nguyên, 16 bit thập phân -> Độ chính xác tuyệt đối
use rayon::prelude::*;    // Xử lý song song
use parking_lot::RwLock;  // Khóa hiệu năng cao
use std::sync::Arc;

// Định nghĩa kiểu số Deterministic (Bất biến trên mọi máy)
// Thay vì f32, ta dùng FixedNumber. 
// Ví dụ: 1.5 luôn là 0x0000000000018000
pub type DNum = I48F16; 

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Neuron {
    pub potential: DNum,
    pub threshold: DNum,
    pub decay: DNum,
    pub weights: Vec<DNum>,
    pub last_spike_height: u64, // Block height lần cuối bắn tín hiệu
}

impl Neuron {
    pub fn new(input_size: usize) -> Self {
        // Khởi tạo trọng số mặc định (Deterministic)
        // Trong thực tế sẽ load từ file gene hoặc training
        let default_weight = DNum::from_num(0.01); 
        Self {
            potential: DNum::from_num(0),
            threshold: DNum::from_num(1.0),
            decay: DNum::from_num(0.9), // Giảm 10% mỗi tick
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

    /// Xử lý song song toàn bộ layer (Rayon)
    pub fn forward(&mut self, inputs: &[DNum], current_height: u64) -> Vec<DNum> {
        // Rayon par_iter_mut() giúp chia việc cho tất cả nhân CPU
        self.neurons.par_iter_mut().map(|n| {
            let mut activation = DNum::from_num(0);
            
            // Tính tổng inputs * weights
            for (i, &input) in inputs.iter().enumerate() {
                if i < n.weights.len() {
                    activation += input * n.weights[i];
                }
            }

            // Cộng dồn vào điện thế màng (Membrane Potential)
            n.potential += activation;

            // Kiểm tra ngưỡng kích hoạt (Spike)
            if n.potential >= n.threshold {
                n.potential = DNum::from_num(0); // Reset (Hyperpolarization)
                n.last_spike_height = current_height;
                DNum::from_num(1) // Output: SPIKE!
            } else {
                // Suy hao tự nhiên (Leak)
                n.potential *= n.decay;
                DNum::from_num(0) // No Spike
            }
        }).collect() // Thu thập kết quả về Vector
    }
}

pub struct SNN {
    pub layers: Arc<RwLock<Vec<Layer>>>, // Dùng parking_lot RwLock
}

impl SNN {
    pub fn new() -> Self {
        // Khởi tạo cấu trúc mạng đơn giản: Input -> Hidden -> Output
        // Input: 64 (từ RenderParams), Hidden: 128, Output: 10
        let l1 = Layer::new(128, 64);
        let l2 = Layer::new(10, 128);

        Self {
            layers: Arc::new(RwLock::new(vec![l1, l2])),
        }
    }

    /// Hàm lan truyền xuôi (Deterministic Forward Pass)
    /// Input: Vector các số Fixed (từ signal_sanitizer)
    /// Output: Vector kết quả
    pub fn process(&self, inputs: Vec<DNum>, height: u64) -> Vec<DNum> {
        let mut current_input = inputs;
        let mut layers = self.layers.write(); // Lấy quyền ghi (nhanh nhờ parking_lot)

        for layer in layers.iter_mut() {
            current_input = layer.forward(&current_input, height);
        }
        
        current_input
    }
}
