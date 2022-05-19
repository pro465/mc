/*
            halt,
            cmp, // reg
            trap, // 4addr
            put, // reg val
            ld, // reg reg reg
            st, // reg reg reg
            brk, // 0 cnd reg reg
            or, // reg reg reg
            and, // reg reg reg
            xor, // reg reg reg
            ls, // reg reg reg
            rs, // reg reg reg
            add, // reg reg reg
            mul, // reg reg reg
            div, // reg reg reg
            not, // reg reg

*/

use crate::trap::TRAP;

pub struct Vm {
    pub(crate) reg: [u8; 0x10],
    pub(crate) pc: usize,
    pub(crate) mem: [u8; 0x10000],
}

impl Vm {
    #[inline]
    pub fn new(mem: [u8; 0x10000]) -> Self {
        Self {
            reg: [0; 0x10],
            pc: 0,
            mem,
        }
    }

    pub fn step(&mut self) {
        let instr = self.load();

        let f = [
            op::<0x0>, op::<0x1>, op::<0x2>, op::<0x3>, op::<0x4>, op::<0x5>, op::<0x6>, op::<0x7>,
            op::<0x8>, op::<0x9>, op::<0xa>, op::<0xb>, op::<0xc>, op::<0xd>, op::<0xe>, op::<0xf>,
        ];

        f[(instr >> 4) as usize](self, (instr & 0xf) as usize);
    }

    pub fn halted(&self) -> bool {
        self.reg[0xf] & 1 != 0
    }

    fn load(&mut self) -> u8 {
        let pc = self.pc;
        self.pc_update(pc as u16 + 1);
        self.mem[pc]
    }

    fn get(&self, reg: u8) -> u8 {
        self.reg[reg as usize]
    }

    fn pc_update(&mut self, addr: u16) {
        self.pc = addr as _;
        self.reg[0xd] = (addr >> 8) as u8;
        self.reg[0xe] = addr as u8;
    }
}

fn op<const I: u8>(vm: &mut Vm, args: usize) {
    if I == 0 {
        vm.reg[0xf] = 1;
        return;
    }

    if I == 1 {
        vm.reg[0xf] = if vm.reg[args] == 0 {
            0b010 << 1
        } else {
            [0b100, 0b001][(vm.reg[args] >> 7) as usize] << 1
        };
        return;
    }

    if I == 2 {
        TRAP[args as usize](vm);
        return;
    }

    let i = 1_u16 << I;
    let next = vm.load();

    if I == 3 {
        vm.reg[args] = next;
        return;
    }

    let (reg1, reg2, reg3) = (vm.reg[args], next >> 4, next & 0xf);

    if i & 0x70 > 0 {
        let addr = ((vm.get(reg2) as u16) << 8 | vm.get(reg3) as u16) as usize;
        if I == 4 {
            vm.reg[args] = vm.mem[addr];
        } else if I == 5 {
            vm.mem[addr] = reg1;
        } else {
            if vm.reg[0xf] & args as u8 > 0 {
                vm.pc_update(addr as u16);
            }
        }
    } else {
        let reg2 = vm.get(reg2);
        vm.reg[reg3 as usize] = match I {
            7 => reg1 | reg2,
            8 => reg1 & reg2,
            9 => reg1 ^ reg2,
            0xa => reg1 << reg2,
            0xb => reg1 >> reg2,
            0xc => reg1.wrapping_add(reg2),
            0xd => reg1.wrapping_mul(reg2),
            0xe => reg1.wrapping_div(reg2),
            0xf => !reg1,
            _ => unreachable!(),
        }
    }
}
