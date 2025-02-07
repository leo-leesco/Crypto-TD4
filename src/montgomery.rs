use std::ops::Mul;

use primitive_types::{U256, U512};

use crate::{A, P};

#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: U256,
    z: U256,
}

/// (a+b)%p
fn add_modulo(a: U256, b: U256, p: U256) -> U256 {
    let a: U512 = U512::from(a);
    let b: U512 = U512::from(b);
    U256::try_from((a + b) % p).unwrap()
}

/// (a*b)%p
fn mul_modulo(a: U256, b: U256, p: U256) -> U256 {
    let a: U512 = U512::from(a);
    let b: U512 = U512::from(b);
    U256::try_from((a * b) % p).unwrap()
}

/// (a-b)%p
fn sub_modulo(a: U256, b: U256, p: U256) -> U256 {
    let a: U512 = U512::from(a);
    let b: U512 = U512::from(b);
    if a > b {
        U256::try_from((a - b) % p).unwrap()
    } else {
        U256::try_from(((a + p) - b) % p).unwrap()
    }
}

/// (x^n)%p
fn pow_modulo(mut x: U256, mut n: U256, p: U256) -> U256 {
    let mut result = U256::one();
    while !n.is_zero() {
        if !(n & U256::one()).is_zero() {
            result = mul_modulo(result, x, p);
        }
        x = mul_modulo(x, x, p);
        n >>= 1;
    }
    result
}

impl Mul<U256> for Point {
    type Output = Self;
    fn mul(self, rhs: U256) -> Self::Output {
        let mut r0 = self.clone();
        let mut r1 = self.double();
        for i in (0..rhs.bits()).rev() {
            if rhs.bit(i) {
                r0 = r0.add(r1, self);
                r1 = r1.double();
            } else {
                r1 = r0.add(r1, self);
                r0 = r0.double();
            }
        }
        r0
    }
}

impl Point {
    fn add(self, rhs: Point, diff: Point) -> Point {
        let xpzp = add_modulo(self.x, self.z, P);
        let xpzq = add_modulo(rhs.x, rhs.z, P);
        let xmzp = sub_modulo(self.x, self.z, P);
        let xmzq = sub_modulo(rhs.x, rhs.z, P);
        let mp = mul_modulo(xmzp, xpzq, P);
        let pm = mul_modulo(xpzp, xmzq, P);
        let p = add_modulo(mp, pm, P);
        let m = sub_modulo(mp, pm, P);
        Point {
            x: mul_modulo(diff.z, mul_modulo(p, p, P), P),
            z: mul_modulo(diff.x, mul_modulo(m, m, P), P),
        }
    }

    fn double(self) -> Point {
        let p = add_modulo(self.x, self.z, P);
        let m = sub_modulo(self.x, self.z, P);
        let p2 = mul_modulo(p, p, P);
        let m2 = mul_modulo(m, m, P);
        let xz = sub_modulo(p2, m2, P);
        Point {
            x: mul_modulo(p2, m2, P),
            z: mul_modulo(xz, add_modulo(m2, mul_modulo(A, xz, P), P), P),
        }
    }
    pub fn new(u: U256) -> Self {
        Point {
            x: u,
            z: U256([1, 0, 0, 0]),
        }
    }

    pub fn get(self) -> U256 {
        mul_modulo(self.x, pow_modulo(self.z, P - 2, P), P)
    }
}
