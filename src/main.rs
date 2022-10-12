#![feature(int_log)]

use crate::core::Program;

use anyhow::Ok;
use anyhow::Result;
use engines::basic;
use engines::basic::Basic;
use engines::register::Register;

mod core;
mod engines;

fn execute_basic_engine() {
    let engine = Basic {
        program: Program::primes(),
    };
    let mut primes = 0;
    for val in engine {
        if primes >= 50 {
            break;
        }
        match val {
            basic::Value::Small(x) => {
                if x.is_power_of_two() {
                    println!("{} {}", primes, x.ilog2());
                    primes += 1;
                }
            }
            basic::Value::Big(x) => {
                let y = x.trailing_zeros().unwrap();
                if y == x.bits() - 1 {
                    println!("{} {:?}", primes, y);
                    primes += 1;
                }
            }
        }
    }
}

fn execute_register_engine() {
    let engine = Register {
        program: Program::primes(),
        output_prime: 2,
    };
    let n = 200;
    println!("N = {}", n);
    for val in engine.into_iter().take(n) {
        print!("{:?},", val);
    }
}

fn main() -> Result<()> {
    // execute_basic_engine();
    execute_register_engine();
    return Ok(());
}
