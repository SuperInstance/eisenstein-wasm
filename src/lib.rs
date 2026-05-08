use wasm_bindgen::prelude::*;
use eisenstein::{E12, HexDisk};

#[wasm_bindgen]
pub struct E12Wasm {
    a: i32,
    b: i32,
}

#[wasm_bindgen]
impl E12Wasm {
    pub fn new(a: i32, b: i32) -> E12Wasm {
        E12Wasm { a, b }
    }

    pub fn zero() -> E12Wasm {
        E12Wasm { a: 0, b: 0 }
    }

    pub fn one() -> E12Wasm {
        E12Wasm { a: 1, b: 0 }
    }

    pub fn omega() -> E12Wasm {
        E12Wasm { a: 0, b: 1 }
    }

    pub fn a(&self) -> i32 { self.a }
    pub fn b(&self) -> i32 { self.b }

    pub fn add(&self, other: &E12Wasm) -> E12Wasm {
        let x = E12::new(self.a, self.b);
        let y = E12::new(other.a, other.b);
        let r = x + y;
        E12Wasm { a: r.a(), b: r.b() }
    }

    pub fn sub(&self, other: &E12Wasm) -> E12Wasm {
        let x = E12::new(self.a, self.b);
        let y = E12::new(other.a, other.b);
        let r = x - y;
        E12Wasm { a: r.a(), b: r.b() }
    }

    pub fn mul(&self, other: &E12Wasm) -> E12Wasm {
        let x = E12::new(self.a, self.b);
        let y = E12::new(other.a, other.b);
        let r = x * y;
        E12Wasm { a: r.a(), b: r.b() }
    }

    pub fn neg(&self) -> E12Wasm {
        let r = E12::new(-self.a, -self.b);
        E12Wasm { a: r.a(), b: r.b() }
    }

    pub fn rotate_60(&self) -> E12Wasm {
        // 60° rotation: multiply by ω = (0,1)
        // (a,b) → (-b, a-b)
        E12Wasm { a: -self.b, b: self.a - self.b }
    }

    pub fn norm(&self) -> i32 {
        let a = self.a as i64;
        let b = self.b as i64;
        (a*a - a*b + b*b) as i32
    }

    pub fn equals(&self, other: &E12Wasm) -> bool {
        self.a == other.a && self.b == other.b
    }

    pub fn to_string(&self) -> String {
        format!("({}, {})", self.a, self.b)
    }
}

#[wasm_bindgen]
pub fn drift_test(iterations: u32) -> String {
    let p = E12::new(5, 3);
    let mut r = p;
    let mut fa = 5.0f64;
    let mut fb = 3.0f64;
    for _ in 0..iterations {
        // E12 rotation
        r = E12::new(-r.b(), r.a() - r.b());
        // Float rotation
        let na = -fb;
        let nb = fa + fb;
        fa = na;
        fb = nb;
    }
    let exact_drift = if r == p { 0.0 } else { (r - p).norm() as f64 };
    let float_drift = ((fa - 5.0).powi(2) + (fb - 3.0).powi(2)).sqrt();
    format!(
        "After {} rotations:\nE12 drift: {} (exact: {})\nFloat drift: {:.6e}",
        iterations, exact_drift, r == p, float_drift
    )
}

#[wasm_bindgen]
pub fn disk_count(radius: u32) -> String {
    let disk = HexDisk::radius(radius);
    let count = disk.count();
    let formula = 3 * radius * radius + 3 * radius + 1;
    format!("Disk(R={}): {} points (formula: {})", radius, count, formula)
}
