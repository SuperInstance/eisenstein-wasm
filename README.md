# eisenstein-wasm

[![npm](https://img.shields.io/npm/v/eisenstein-wasm.svg)](https://www.npmjs.com/package/eisenstein-wasm)

Same exact hex integer arithmetic as [eisenstein](https://github.com/SuperInstance/eisenstein), compiled to WebAssembly for browsers and Node.js.

## Install

```bash
npm install eisenstein-wasm
```

## Usage

```js
import { E12 } from "eisenstein-wasm";

const a = new E12(1, 0);
const b = new E12(0, 1);
console.log(a.add(b).toString()); // "(1, 1)"
```

See **[npm-package/README.md](npm-package/README.md)** for full API docs.

## Eisenstein Ecosystem

Part of the **[Eisenstein hex integer ecosystem](https://github.com/SuperInstance/eisenstein)** — exact hex arithmetic from microcontrollers to browsers to formal verification.

| Project | Description |
|---------|-------------|
| **[eisenstein](https://github.com/SuperInstance/eisenstein)** | Core Rust crate — exact hex arithmetic, zero deps |
| **[eisenstein-c](https://github.com/SuperInstance/eisenstein-c)** | Same math, for microcontrollers. 1KB `.text`. |
| **[eisenstein-wasm](https://github.com/SuperInstance/eisenstein-wasm)** | Same math, for browsers and Node.js |
| **[eisenstein-bench](https://github.com/SuperInstance/eisenstein-bench)** | Benchmark all implementations side-by-side |
| **[eisenstein-fuzz](https://github.com/SuperInstance/eisenstein-fuzz)** | Property-based fuzzing across the ecosystem |
| **[eisenstein-do178c](https://github.com/SuperInstance/eisenstein-do178c)** | DO-178C formally verified for safety-critical systems |
| **[arm-neon-eisenstein-bench](https://github.com/SuperInstance/arm-neon-eisenstein-bench)** | 4× parallel hex math on ARM NEON |
| **[hexgrid-gen](https://github.com/SuperInstance/hexgrid-gen)** | Code generation for any language in the ecosystem |
| **[constraint-theory-core](https://github.com/SuperInstance/constraint-theory-core)** | Production constraint framework built on Eisenstein math |
| **[flux-lucid](https://github.com/SuperInstance/flux-lucid)** | Unified intent-directed ecosystem orchestrator |

**Next →** Benchmark it: **[eisenstein-bench](https://github.com/SuperInstance/eisenstein-bench)**

## License

MIT OR Apache-2.0
