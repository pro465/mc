use mc::vm::Vm;
use std::env;
use std::fs::File;
use std::io::{ErrorKind, Read};

fn main() {
    let mut prog = [0; 0x20000];

    for p in env::args().skip(1) {
        let mut f = File::open(p).unwrap();
        let mut origin = [0; 2];

        f.read_exact(&mut origin).unwrap();

        let origin = u16::from_be_bytes(origin) as usize;
        if let Err(e) = f.read_exact(&mut prog[origin * 2..]) {
            if e.kind() != ErrorKind::UnexpectedEof {
                panic!("error reading file to load bytecode: {:?}", e);
            }
        }
    }
    let mut mem = [0; 0x10000];

    for (i, x) in prog.chunks_exact(2).enumerate() {
        mem[i] = u16::from_be_bytes(x.try_into().unwrap());
    }

    let mut vm = Vm::new(mem);

    while !vm.halted() {
        vm.step();
    }
}
