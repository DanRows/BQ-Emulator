use criterion::{black_box, criterion_group, criterion_main, Criterion};
use quantum_emulator::QuantumEmulator;

fn benchmark_hadamard(c: &mut Criterion) {
    let mut emulator = QuantumEmulator::new(1);
    c.bench_function("hadamard_gate", |b| {
        b.iter(|| {
            emulator.apply_hadamard(black_box(0)).unwrap();
        })
    });
}

fn benchmark_cnot(c: &mut Criterion) {
    let mut emulator = QuantumEmulator::new(2);
    c.bench_function("cnot_gate", |b| {
        b.iter(|| {
            emulator.apply_cnot(black_box(0), black_box(1)).unwrap();
        })
    });
}

fn benchmark_measurement(c: &mut Criterion) {
    let emulator = QuantumEmulator::new(1);
    c.bench_function("qubit_measurement", |b| {
        b.iter(|| {
            emulator.measure(black_box(0)).unwrap();
        })
    });
}

criterion_group!(benches, benchmark_hadamard, benchmark_cnot, benchmark_measurement);
criterion_main!(benches); 