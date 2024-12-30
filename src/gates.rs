use ndarray::Array2;
use crate::qubit::Qubit;

pub fn hadamard(qubit: &mut Qubit) {
    let h = Array2::from_shape_vec((2, 2), vec![
        1.0_f64 / 2.0_f64.sqrt(), 1.0_f64 / 2.0_f64.sqrt(),
        1.0_f64 / 2.0_f64.sqrt(), -1.0_f64 / 2.0_f64.sqrt(),
    ]).unwrap();

    let state = Array2::from_shape_vec((2, 1), vec![qubit.alpha, qubit.beta]).unwrap();
    let result = h.dot(&state);
    
    qubit.alpha = result[[0, 0]];
    qubit.beta = result[[1, 0]];
    qubit.normalize();
}

pub fn not_gate(qubit: &mut Qubit) {
    std::mem::swap(&mut qubit.alpha, &mut qubit.beta);
}

pub fn cnot(control: &Qubit, target: &mut Qubit) {
    if control.measure() {
        not_gate(target);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hadamard() {
        let mut qubit = Qubit::new(1.0, 0.0);
        hadamard(&mut qubit);
        assert!((qubit.alpha - 1.0_f64/2.0_f64.sqrt()).abs() < 1e-10);
        assert!((qubit.beta - 1.0_f64/2.0_f64.sqrt()).abs() < 1e-10);
    }

    #[test]
    fn test_not_gate() {
        let mut qubit = Qubit::new(1.0, 0.0);
        not_gate(&mut qubit);
        assert!((qubit.alpha - 0.0).abs() < 1e-10);
        assert!((qubit.beta - 1.0).abs() < 1e-10);
    }
} 