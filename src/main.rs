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
        program: Program::primes_optimized(),
        output_base: 10,
        // program: Program::primes(),
        // output_base: 2
    };
    let n = 250;
    println!("N = {}", n);
    for val in engine.into_iter().take(n) {
        print!("{:?},", val);
    }
}

fn execute_fib() {
    for i in 1..15 {
        let engine = Register {
            program: Program::fibonacci(i),
            output_base: 2,
        };
        let n = 1;
        for val in engine.into_iter().take(n) {
            print!("{:?},", val);
        }
    }
}

fn main() -> Result<()> {
    // execute_basic_engine();
    // execute_register_engine();
    execute_fib();
    return Ok(());
}
