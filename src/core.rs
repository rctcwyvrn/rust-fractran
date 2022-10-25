use num_traits::Pow;

pub type FracSize = u16;

#[derive(Debug)]
pub struct Program {
    pub fractions: Vec<(FracSize, FracSize)>,
    pub initial: u64,
}

impl Program {
    pub fn primes() -> Program {
        Program {
            fractions: vec![
                (17, 91),
                (78, 85),
                (19, 51),
                (23, 38),
                (29, 33),
                (77, 29),
                (95, 23),
                (77, 19),
                (1, 17),
                (11, 13),
                (13, 11),
                (15, 14),
                (15, 2),
                (55, 1),
            ],
            initial: 2,
        }
    }

    pub fn primes_optimized() -> Program {
        Program {
            fractions: vec![
                (3, 11),
                (847, 45),
                (143, 6),
                (7, 3),
                (10, 91),
                (3, 7),
                (36, 325),
                (1, 2),
                (36, 5),
            ],
            initial: 10,
        }
    }
    pub fn fibonacci(i: u32) -> Program {
        Program {
            fractions: vec![
                (17, 65),
                (133, 34),
                (17, 19),
                (23, 17),
                (2233, 69),
                (23, 29),
                (31, 23),
                (74, 341),
                (31, 37),
                (41, 31),
                (129, 287),
                (41, 43),
                (13, 41),
                (1, 13),
                (1, 3),
            ],
            initial: 78 * 5u64.pow(i),
        }
    }
}

// #[derive(Debug)]
// pub struct Log {
//     pub output: Vec<Value>,
// }

pub trait FractranEngine<Value>: IntoIterator<Item = Value>
where
    Value: From<u64>,
{
}
