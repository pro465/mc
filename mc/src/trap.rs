use crate::vm::Vm;
use std::io::{self, Read, Write};

pub const TRAP: &[fn(&mut Vm)] = &[
    |v| io::stdout().write_all(&[(v.reg[0] >> 8) as u8]).unwrap(),
    |v| {
        let mut read = [0];
        io::stdin().read_exact(&mut read).unwrap();
        v.reg[0] &= 0xff;
        v.reg[0] |= u16::from(read[0]) << 8;
    },
    |_| io::stdout().flush().unwrap(),
];
