#![feature(int_log)]

use crate::core::FractranEngine;
use crate::core::Program;

use anyhow::Ok;
use anyhow::Result;
use engines::basic::Basic;


mod core;
mod engines;


fn main() -> Result<()> {
    let engine = Basic { program: Program::primes() };
    for val in engine {
        match val {
            core::Value::Small(x) => {
                if x.is_power_of_two() {
                    println!("{}", x.ilog2())
                }
            }
            core::Value::Big(x) => {
                let y = x.trailing_zeros().unwrap();
                if y == x.bits() - 1 {
                    println!("{:?}", y)
                }
            },
        }
    }
    return Ok(());
}
