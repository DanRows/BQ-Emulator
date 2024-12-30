use quantum_emulator::QuantumEmulator;

fn main() {
    println!("=== Experimento de Entrelazamiento Cuántico ===\n");
    
    // Crear un emulador con 2 qubits
    let mut emulator = QuantumEmulator::new(2);
    println!("Estado inicial:");
    print_qubit_states(&emulator);

    // Paso 1: Aplicar Hadamard al primer qubit
    println!("\nPaso 1: Aplicando puerta Hadamard al qubit 0");
    emulator.apply_hadamard(0).unwrap();
    print_qubit_states(&emulator);

    // Paso 2: Aplicar CNOT para entrelazar los qubits
    println!("\nPaso 2: Aplicando CNOT entre qubit 0 (control) y qubit 1 (target)");
    emulator.apply_cnot(0, 1).unwrap();
    print_qubit_states(&emulator);

    // Paso 3: Realizar múltiples mediciones para ver correlación
    println!("\nPaso 3: Realizando 10 mediciones del estado entrelazado:");
    for i in 0..10 {
        let measurements = emulator.measure_all();
        println!("Medición {}: Qubit 0 = {}, Qubit 1 = {}", 
            i + 1,
            if measurements[0] { "|1⟩" } else { "|0⟩" },
            if measurements[1] { "|1⟩" } else { "|0⟩" }
        );
        
        // Reiniciar el estado para la siguiente medición
        emulator = QuantumEmulator::new(2);
        emulator.apply_hadamard(0).unwrap();
        emulator.apply_cnot(0, 1).unwrap();
    }
}

fn print_qubit_states(emulator: &QuantumEmulator) {
    for i in 0..2 {
        let (alpha, beta) = emulator.get_state(i).unwrap();
        println!("Qubit {}: α = {:.3}, β = {:.3}", i, alpha, beta);
    }
}
