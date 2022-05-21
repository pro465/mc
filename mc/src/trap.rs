use crate::vm::Vm;
use std::io::{self, Read, Write};

pub const TRAP: &[fn(&mut Vm)] = &[
    |v| io::stdout().write_all(&[v.reg[0]]).unwrap(),
    |v| io::stdin().read_exact(&mut v.reg[0..1]).unwrap(),
    |_| io::stdout().flush().unwrap(),
];
