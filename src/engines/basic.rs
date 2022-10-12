use core::panic;

use num_bigint::{BigUint, ToBigUint};

use crate::core::{FracSize, FractranEngine, Program};
use num_traits::identities::Zero;

pub struct Basic {
    pub program: Program,
}

#[derive(Debug, Clone)]
pub enum Value {
    Small(u64),
    Big(BigUint),
}

impl From<u64> for Value {
    fn from(x: u64) -> Self {
        Value::Small(x)
    }
}

impl Value {
    fn multiple(&self, denom: FracSize) -> bool {
        match self {
            Value::Small(x) => x % denom as u64 == 0,
            Value::Big(x) => x % denom == BigUint::zero(),
        }
    }

    fn mult(&mut self, num: FracSize, denom: FracSize) {
        match self {
            Value::Small(x) => {
                let y = (*x / denom as u64).checked_mul(num as u64);
                if let Some(val) = y {
                    *x = val;
                } else {
                    *self = Value::Big(x.to_biguint().unwrap());
                    self.mult(num, denom);
                }
            }
            Value::Big(x) => {
                *x *= num.to_biguint().unwrap();
                *x /= denom.to_biguint().unwrap();

                // Random demotion heuristic
                // if its 32 bits there's probably a good amount of room
                if x.bits() <= 32 {
                    let y = x.to_u64_digits();
                    debug_assert!(y.len() == 1);
                    *self = Value::Small(y[0]);
                }
            }
        }
    }
}

pub struct BasicIter {
    program: Program,
    n: Value,
    cycles: i32,
}

impl BasicIter {
    fn from(engine: Basic) -> BasicIter {
        BasicIter {
            n: Value::from(engine.program.initial),
            program: engine.program,
            cycles: 0,
        }
    }
}

impl Iterator for BasicIter {
    type Item = Value;
    fn next(&mut self) -> Option<Self::Item> {
        // if self.cycles >= MAX_CYCLES {
        //     return None
        // }

        for (num, denom) in &self.program.fractions {
            if self.n.multiple(*denom) {
                self.n.mult(*num, *denom);
                self.cycles += 1;
                return Some(self.n.clone());
            }
        }
        panic!("None divide");
    }
}

impl IntoIterator for Basic {
    type Item = Value;
    type IntoIter = BasicIter;

    fn into_iter(self) -> Self::IntoIter {
        BasicIter::from(self)
    }
}

impl FractranEngine<Value> for Basic {}
