pub type FracSize = u8;

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
