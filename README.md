# Emulador Cuántico-Binario

Este es un MVP (Minimum Viable Product) de un emulador cuántico-binario implementado en Rust. El emulador permite simular qubits y aplicar operaciones cuánticas básicas, proporcionando una base para experimentar con conceptos de computación cuántica.

## Fundamentos Teóricos

### Qubits
Un qubit es la unidad básica de información cuántica, análogo al bit clásico. A diferencia de un bit clásico que solo puede estar en estado 0 o 1, un qubit puede estar en una superposición de ambos estados:

|ψ⟩ = α|0⟩ + β|1⟩

donde:
- α y β son números complejos llamados amplitudes
- |α|² + |β|² = 1 (normalización)
- |α|² es la probabilidad de medir el estado |0⟩
- |β|² es la probabilidad de medir el estado |1⟩

### Puertas Cuánticas Implementadas

1. **Puerta Hadamard (H)**:
   - Crea una superposición cuántica
   - Matriz de transformación:
     ```
     H = 1/√2 [ 1  1 ]
              [ 1 -1 ]
     ```
   - Transforma |0⟩ → (|0⟩ + |1⟩)/√2
   - Transforma |1⟩ → (|0⟩ - |1⟩)/√2

2. **Puerta NOT (X)**:
   - Invierte el estado del qubit
   - Matriz de transformación:
     ```
     X = [ 0 1 ]
         [ 1 0 ]
     ```
   - Transforma |0⟩ → |1⟩ y |1⟩ → |0⟩

3. **Puerta CNOT (Control-NOT)**:
   - Opera sobre dos qubits: control y objetivo
   - Aplica NOT al qubit objetivo solo si el qubit de control está en |1⟩
   - Fundamental para crear entrelazamiento cuántico

## Implementación

### Estructura del Qubit
```rust
pub struct Qubit {
    pub alpha: f64,  // Amplitud del estado |0⟩
    pub beta: f64,   // Amplitud del estado |1⟩
}
```

### Operaciones Principales

1. **Creación de Qubits**:
   ```rust
   // Crear un qubit en estado |0⟩
   let qubit = Qubit::new(1.0, 0.0);
   
   // Crear un qubit aleatorio normalizado
   let random_qubit = Qubit::random();
   ```

2. **Aplicación de Puertas**:
   ```rust
   // Aplicar Hadamard
   emulator.apply_hadamard(qubit_index);
   
   // Aplicar NOT
   emulator.apply_not(qubit_index);
   
   // Aplicar CNOT
   emulator.apply_cnot(control_index, target_index);
   ```

3. **Medición**:
   ```rust
   // Medir un qubit específico
   let result = emulator.measure(qubit_index);
   
   // Medir todos los qubits
   let results = emulator.measure_all();
   ```

## Ejemplo de Entrelazamiento Cuántico

El siguiente código crea un par de qubits entrelazados (estado de Bell):

```rust
use quantum_emulator::QuantumEmulator;

fn main() {
    let mut emulator = QuantumEmulator::new(2);
    
    // Crear superposición en el primer qubit
    emulator.apply_hadamard(0);
    
    // Entrelazar los qubits
    emulator.apply_cnot(0, 1);
    
    // Al medir, los qubits siempre mostrarán resultados correlacionados:
    // |00⟩ o |11⟩ con igual probabilidad
    let measurements = emulator.measure_all();
}
```

## Limitaciones Actuales

1. **Simulación Clásica**: 
   - El emulador utiliza números de punto flotante para simular amplitudes cuánticas
   - La precisión está limitada por el tipo `f64`

2. **Escalabilidad**:
   - La simulación se vuelve exponencialmente más compleja con cada qubit adicional
   - Recomendado para experimentos con pocos qubits (2-5)

3. **Puertas Implementadas**:
   - Solo incluye las puertas básicas (H, X, CNOT)
   - No incluye puertas de fase o rotación

## Uso Avanzado

### Visualización de Estados
```rust
// Obtener el estado completo de un qubit
let (alpha, beta) = emulator.get_state(qubit_index).unwrap();
println!("Estado: {}|0⟩ + {}|1⟩", alpha, beta);
```

### Experimentos Múltiples
```rust
// Realizar múltiples mediciones para observar distribuciones
for _ in 0..100 {
    let mut emulator = QuantumEmulator::new(1);
    emulator.apply_hadamard(0);
    let result = emulator.measure(0);
    // Analizar resultados...
}
```

## Contribuir

Las áreas principales para contribuciones incluyen:
1. Implementación de más puertas cuánticas
2. Mejora de la eficiencia de simulación
3. Adición de visualizaciones de estados cuánticos
4. Implementación de algoritmos cuánticos conocidos

## Licencia

Este proyecto está licenciado bajo la Licencia MIT - ver el archivo LICENSE para más detalles. 