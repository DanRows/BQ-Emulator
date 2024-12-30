use crate::qubit::Qubit;
use crate::gates::{hadamard, not_gate, cnot};

pub struct QuantumEmulator {
    qubits: Vec<Qubit>,
}

impl QuantumEmulator {
    pub fn new(num_qubits: usize) -> Self {
        let qubits = (0..num_qubits)
            .map(|_| Qubit::new(1.0, 0.0))
            .collect();
        QuantumEmulator { qubits }
    }

    pub fn get_num_qubits(&self) -> usize {
        self.qubits.len()
    }

    pub fn apply_hadamard(&mut self, qubit_index: usize) -> Result<(), String> {
        if qubit_index >= self.qubits.len() {
            return Err(format!("Índice de qubit {} fuera de rango", qubit_index));
        }
        hadamard(&mut self.qubits[qubit_index]);
        Ok(())
    }

    pub fn apply_not(&mut self, qubit_index: usize) -> Result<(), String> {
        if qubit_index >= self.qubits.len() {
            return Err(format!("Índice de qubit {} fuera de rango", qubit_index));
        }
        not_gate(&mut self.qubits[qubit_index]);
        Ok(())
    }

    pub fn apply_cnot(&mut self, control: usize, target: usize) -> Result<(), String> {
        if control >= self.qubits.len() || target >= self.qubits.len() {
            return Err(format!("Índices de qubit fuera de rango"));
        }
        if control == target {
            return Err(String::from("Control y target no pueden ser el mismo qubit"));
        }
        
        // Clonamos el qubit de control para evitar el préstamo mutable
        let control_qubit = self.qubits[control].clone();
        cnot(&control_qubit, &mut self.qubits[target]);
        Ok(())
    }

    pub fn measure(&self, qubit_index: usize) -> Result<bool, String> {
        if qubit_index >= self.qubits.len() {
            return Err(format!("Índice de qubit {} fuera de rango", qubit_index));
        }
        Ok(self.qubits[qubit_index].measure())
    }

    pub fn measure_all(&self) -> Vec<bool> {
        self.qubits.iter().map(|q| q.measure()).collect()
    }

    pub fn get_state(&self, qubit_index: usize) -> Result<(f64, f64), String> {
        if qubit_index >= self.qubits.len() {
            return Err(format!("Índice de qubit {} fuera de rango", qubit_index));
        }
        Ok((self.qubits[qubit_index].alpha, self.qubits[qubit_index].beta))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_emulator() {
        let emulator = QuantumEmulator::new(2);
        assert_eq!(emulator.get_num_qubits(), 2);
    }

    #[test]
    fn test_hadamard_gate() {
        let mut emulator = QuantumEmulator::new(1);
        assert!(emulator.apply_hadamard(0).is_ok());
        let (alpha, beta) = emulator.get_state(0).unwrap();
        assert!((alpha - 1.0_f64/2.0_f64.sqrt()).abs() < 1e-10);
        assert!((beta - 1.0_f64/2.0_f64.sqrt()).abs() < 1e-10);
    }
} 