use crate::vm::Vm;
use std::io::{self, Write};
pub const TRAP: &[fn(&mut Vm)] = &[
    |v| io::stdout().write_all(&[v.reg[0]]).unwrap(),
];
