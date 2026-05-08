# eisenstein-wasm

**Same exact Eisenstein integer arithmetic, compiled for the browser and Node.js.**

WASM package that wraps the Rust [eisenstein](https://github.com/SuperInstance/eisenstein) crate. Every operation — norm, rotation, hex distance, disk iteration — runs in WebAssembly with integer arithmetic. No floats, no drift, no rounding. The WASM binary is ~15KB gzipped.

## Quick Start

```javascript
import init, { E12Wasm, hex_disk, drift_test } from 'eisenstein-wasm';

await init();

const a = E12Wasm.new(3, 1);
const b = E12Wasm.new(1, 2);

a.add(b);           // exact
a.mul(b);           // exact
a.rotate_60();      // stays on the lattice

console.log(a.norm());  // 7 — exact integer

// 10,000 rotations, zero drift
const result = drift_test(10000);
console.log(result.float_drift);  // ~2e-12 (float version drifts)
```

Install:

```bash
npm install @superinstance/eisenstein-wasm
```

## Why WASM?

Floating-point hex coordinates in JavaScript drift after repeated operations — JS has no control over FPU rounding modes across browsers. Eisenstein integers don't have that problem because the arithmetic is integer-only. The WASM binary gives you exact, deterministic hex math that behaves identically on every browser, every device, every time.

## License

MIT OR Apache-2.0
