use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Qubit {
    pub alpha: f64,
    pub beta: f64,
}

impl Qubit {
    pub fn new(alpha: f64, beta: f64) -> Self {
        let mut qubit = Qubit { alpha, beta };
        qubit.normalize();
        qubit
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let alpha = rng.gen::<f64>();
        let beta = (1.0 - alpha.powi(2)).sqrt();
        Qubit::new(alpha, beta)
    }

    pub fn measure(&self) -> bool {
        let mut rng = rand::thread_rng();
        let prob_zero = self.alpha.powi(2);
        rng.gen::<f64>() < prob_zero
    }

    pub fn normalize(&mut self) {
        let norm = (self.alpha.powi(2) + self.beta.powi(2)).sqrt();
        if norm != 0.0 {
            self.alpha /= norm;
            self.beta /= norm;
        }
    }
} 