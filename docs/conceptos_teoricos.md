# Conceptos Teóricos del Emulador Cuántico

## Fundamentos de la Computación Cuántica

### Estados Cuánticos

Un qubit es un sistema cuántico de dos niveles que puede existir en una superposición de estados base |0⟩ y |1⟩. El estado general de un qubit se representa como:

|ψ⟩ = α|0⟩ + β|1⟩

donde:
- α y β son números complejos llamados amplitudes de probabilidad
- La condición de normalización requiere que |α|² + |β|² = 1
- Al medir el qubit, obtenemos:
  - |0⟩ con probabilidad |α|²
  - |1⟩ con probabilidad |β|²

### Operaciones Cuánticas

#### 1. Puerta Hadamard (H)
La puerta Hadamard crea una superposición uniforme de estados:

```
H = 1/√2 [ 1  1 ]
         [ 1 -1 ]
```

Efectos:
- H|0⟩ = (|0⟩ + |1⟩)/√2
- H|1⟩ = (|0⟩ - |1⟩)/√2

#### 2. Puerta NOT (X)
La puerta NOT invierte los estados base:

```
X = [ 0 1 ]
    [ 1 0 ]
```

Efectos:
- X|0⟩ = |1⟩
- X|1⟩ = |0⟩

#### 3. Puerta CNOT
La puerta CNOT opera en dos qubits y es fundamental para crear entrelazamiento:

```
CNOT = [ 1 0 0 0 ]
       [ 0 1 0 0 ]
       [ 0 0 0 1 ]
       [ 0 0 1 0 ]
```

### Entrelazamiento Cuántico

El entrelazamiento es un fenómeno cuántico donde los estados de dos o más qubits no pueden describirse independientemente. Los estados de Bell son ejemplos típicos de estados entrelazados:

1. |Φ⁺⟩ = (|00⟩ + |11⟩)/√2
2. |Φ⁻⟩ = (|00⟩ - |11⟩)/√2
3. |Ψ⁺⟩ = (|01⟩ + |10⟩)/√2
4. |Ψ⁻⟩ = (|01⟩ - |10⟩)/√2

## Implementación en el Emulador

### Representación de Estados

En nuestro emulador, representamos los estados cuánticos usando números de punto flotante:

```rust
pub struct Qubit {
    pub alpha: f64,  // Amplitud de |0⟩
    pub beta: f64,   // Amplitud de |1⟩
}
```

### Normalización

La normalización asegura que |α|² + |β|² = 1:

```rust
pub fn normalize(&mut self) {
    let norm = (self.alpha.powi(2) + self.beta.powi(2)).sqrt();
    if norm != 0.0 {
        self.alpha /= norm;
        self.beta /= norm;
    }
}
```

### Medición

La medición colapsa el estado cuántico según las probabilidades:

```rust
pub fn measure(&self) -> bool {
    let mut rng = rand::thread_rng();
    let prob_zero = self.alpha.powi(2);
    rng.gen::<f64>() < prob_zero
}
```

## Limitaciones de la Simulación Clásica

1. **Precisión Numérica**:
   - Usamos `f64` para las amplitudes, lo que limita la precisión
   - Los errores de redondeo pueden acumularse en circuitos complejos

2. **Escalabilidad**:
   - El espacio de estados crece exponencialmente: 2ⁿ para n qubits
   - La memoria necesaria se duplica con cada qubit adicional

3. **Simplificaciones**:
   - No simulamos decoherencia cuántica
   - No incluimos efectos de ruido cuántico
   - Las operaciones son ideales (sin errores) 