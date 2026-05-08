# @superinstance/eisenstein-wasm

Eisenstein integer arithmetic for WebAssembly. Exact hex grid operations in the browser — zero drift, integer arithmetic, 60° rotation that always returns to the lattice.

## Installation

```bash
npm install @superinstance/eisenstein-wasm
```

## Usage

```javascript
import init, { E12Wasm, hex_disk, drift_test } from '@superinstance/eisenstein-wasm';

await init();

// Create Eisenstein integers
const a = E12Wasm.new(3, 1);
const b = E12Wasm.new(1, 2);

// Exact arithmetic
const sum = a.add(b);
const prod = a.mul(b);

// 60° rotation — always stays on the lattice
const rotated = a.rotate_60();

// Norm is an integer — no sqrt
console.log(a.norm()); // 7

// Drift test: 10K rotations
const result = drift_test(10000);
console.log(result.exact_drift); // 0.0 — exact!
console.log(result.float_drift); // ~2e-12 — float drifts

// Get all hex grid points within radius
const disk = hex_disk(10); // 331 points, all exact
```

## Why?

Floating-point hex coordinates drift. Eisenstein integers don't. Every rotation, addition, and norm is computed with integer arithmetic — exact, deterministic, and 3× faster than the float equivalent.

## Performance

The WASM binary is ~15KB gzipped. All arithmetic operations complete in <1μs. Running in a web worker, you can simulate thousands of hex grid agents per frame without jank.
