# Ejemplos Prácticos del Emulador Cuántico

Este documento presenta varios ejemplos prácticos para entender y utilizar el emulador cuántico.

## 1. Superposición Cuántica

### Creación de Superposición con Puerta Hadamard

```rust
use quantum_emulator::QuantumEmulator;

fn main() {
    let mut emulator = QuantumEmulator::new(1);
    
    // Estado inicial |0⟩
    let (alpha, beta) = emulator.get_state(0).unwrap();
    println!("Estado inicial: {}|0⟩ + {}|1⟩", alpha, beta);
    
    // Aplicar Hadamard
    emulator.apply_hadamard(0).unwrap();
    let (alpha, beta) = emulator.get_state(0).unwrap();
    println!("Después de Hadamard: {}|0⟩ + {}|1⟩", alpha, beta);
    
    // Realizar 100 mediciones para ver la distribución
    let mut zeros = 0;
    let mut ones = 0;
    for _ in 0..100 {
        if emulator.measure(0).unwrap() {
            ones += 1;
        } else {
            zeros += 1;
        }
    }
    println!("Resultados: |0⟩: {}%, |1⟩: {}%", zeros, ones);
}
```

## 2. Estado de Bell (Entrelazamiento)

### Creación del Estado de Bell |Φ⁺⟩

```rust
use quantum_emulator::QuantumEmulator;

fn main() {
    let mut emulator = QuantumEmulator::new(2);
    
    // Paso 1: Hadamard en qubit 0
    emulator.apply_hadamard(0).unwrap();
    
    // Paso 2: CNOT entre qubit 0 y 1
    emulator.apply_cnot(0, 1).unwrap();
    
    // Realizar múltiples mediciones para ver correlación
    println!("Mediciones del estado de Bell:");
    for i in 0..10 {
        let results = emulator.measure_all();
        println!("Medición {}: |{}{}⟩", i+1, 
            if results[0] { "1" } else { "0" },
            if results[1] { "1" } else { "0" });
    }
}
```

## 3. Operaciones NOT y Mediciones

### Demostración de la Puerta NOT

```rust
use quantum_emulator::QuantumEmulator;

fn main() {
    let mut emulator = QuantumEmulator::new(1);
    
    // Estado inicial |0⟩
    let (alpha, beta) = emulator.get_state(0).unwrap();
    println!("Estado inicial: {}|0⟩ + {}|1⟩", alpha, beta);
    
    // Aplicar NOT
    emulator.apply_not(0).unwrap();
    let (alpha, beta) = emulator.get_state(0).unwrap();
    println!("Después de NOT: {}|0⟩ + {}|1⟩", alpha, beta);
    
    // Medir
    let result = emulator.measure(0).unwrap();
    println!("Medición: |{}⟩", if result { "1" } else { "0" });
}
```

## 4. Circuito Cuántico Complejo

### Combinación de Varias Operaciones

```rust
use quantum_emulator::QuantumEmulator;

fn main() {
    let mut emulator = QuantumEmulator::new(2);
    
    // Secuencia de operaciones
    emulator.apply_hadamard(0).unwrap();  // H en q0
    emulator.apply_not(1).unwrap();       // X en q1
    emulator.apply_cnot(0, 1).unwrap();   // CNOT q0->q1
    emulator.apply_hadamard(0).unwrap();  // H en q0
    
    // Medir estados finales
    println!("Estados finales:");
    for i in 0..2 {
        let (alpha, beta) = emulator.get_state(i).unwrap();
        println!("Qubit {}: {}|0⟩ + {}|1⟩", i, alpha, beta);
    }
}
```

## 5. Análisis Estadístico

### Distribución de Mediciones

```rust
use quantum_emulator::QuantumEmulator;
use std::collections::HashMap;

fn main() {
    let mut emulator = QuantumEmulator::new(2);
    let num_measurements = 1000;
    
    // Preparar estado entrelazado
    emulator.apply_hadamard(0).unwrap();
    emulator.apply_cnot(0, 1).unwrap();
    
    // Realizar mediciones y contar resultados
    let mut results = HashMap::new();
    for _ in 0..num_measurements {
        let measurement = emulator.measure_all();
        let key = format!("{}{}", 
            if measurement[0] { "1" } else { "0" },
            if measurement[1] { "1" } else { "0" });
        *results.entry(key).or_insert(0) += 1;
    }
    
    // Mostrar distribución
    println!("Distribución de {} mediciones:", num_measurements);
    for (state, count) in results.iter() {
        let percentage = (*count as f64 / num_measurements as f64) * 100.0;
        println!("|{}⟩: {:.1}%", state, percentage);
    }
}
```

## Notas Importantes

1. **Precisión de Mediciones**:
   - Las mediciones son probabilísticas
   - Se recomienda realizar múltiples mediciones para obtener distribuciones estadísticas

2. **Estado de los Qubits**:
   - Después de una medición, el estado colapsa
   - Para repetir un experimento, es necesario reinicializar el sistema

3. **Entrelazamiento**:
   - Las mediciones de qubits entrelazados están correlacionadas
   - El orden de las mediciones puede afectar los resultados

4. **Depuración**:
   - Usar `get_state()` para verificar estados intermedios
   - Imprimir amplitudes para entender la evolución del sistema 