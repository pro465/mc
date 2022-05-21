/*
        ("halt", 0),
        ("cmp", 1),  // reg
        ("trap", 2), // 4addr
        ("if", 3),  // 0 cnd
        ("push", 4), // reg
        ("put", 5),  // reg val
        ("ld", 6),   // reg reg reg
        ("st", 7),   // reg reg reg
        ("or", 8),   // reg reg reg
        ("and", 9),  // reg reg reg
        ("xor", 10,  // reg reg reg
        ("ls", 11),  // reg reg reg
        ("rs", 12),  // reg reg reg
        ("add", 13), // reg reg reg
        ("mv", 14),  // reg 0x0 reg
        ("jmp", 15),
        // 0b00 addrh addrl
        // 0b01 imm8
        // 0b10 0x00 reg
        // 0b11 reg reg
*/

use crate::trap::TRAP;

const PSR: usize = 0xf;
const PCL: usize = 0xe;
const PCH: usize = 0xd;
const SPL: usize = 0xc;
const SPH: usize = 0xb;

pub struct Vm {
    pub(crate) reg: [u8; 0x10],
    pub(crate) pc: u16,
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
        self.pc_update(pc + 1);
        self.mem[usize::from(pc)]
    }

    fn get(&self, reg: u8) -> u8 {
        self.reg[reg as usize]
    }

    fn pc_update(&mut self, addr: u16) {
        self.pc = addr;
        self.reg[PCH] = (addr >> 8) as u8;
        self.reg[PCL] = addr as u8;
    }
}

fn op<const I: u8>(vm: &mut Vm, args: usize) {
    if I < 5 {
        match I {
            0 => {
                vm.reg[PSR] = 1;
            }

            1 => {
                vm.reg[PSR] = if vm.reg[args] == 0 {
                    0b010 << 1
                } else {
                    [0b100, 0b001][(vm.reg[args] >> 7) as usize] << 1
                };
            }

            2 => {
                TRAP[args as usize](vm);
            }

            3 => {
                if (vm.reg[PSR] >> 1) & args as u8 == 0 {
                    let next = vm.load();
                    let offset = if next == 0xf0 {
                        2
                    } else if next >> 4 > 4 {
                        1
                    } else {
                        return;
                    };

                    let addr = vm.pc + offset;
                    vm.pc_update(addr);
                }
            }

            4 => {
                let p = vm.reg[args];
                let mut sp = u16::from_be_bytes([vm.reg[SPH], vm.reg[SPL]]);

                vm.mem[sp as usize] = p;

                sp -= 1;

                vm.reg[SPH] = (sp >> 8) as u8;
                vm.reg[SPL] = sp as u8;
            }

            _ => unreachable!(),
        }
        return;
    }

    let next = vm.load();

    if I == 5 {
        vm.reg[args] = next;
        return;
    }

    let (reg1, reg2, reg3) = (vm.reg[args], next >> 4, next & 0xf);

    let sext = |x| {
        if x & 0x80 > 0 {
            x as u16 | 0xff00
        } else {
            x as u16
        }
    };

    let i = 1_u16 << I;

    if i & 0x80C0 > 0 {
        if I == 6 {
            vm.reg[reg3 as usize] = vm.mem[u16::from_be_bytes([reg1, vm.get(reg2)]) as usize];
        } else {
            let addr = u16::from_be_bytes([vm.get(reg2), vm.get(reg3)]);
            if I == 7 {
                vm.mem[addr as usize] = reg1;
            } else {
                let addr = match args & 0b11 {
                    0b00 => u16::from_be_bytes([next, vm.load()]),
                    0b01 => vm.pc.wrapping_add(sext(next)),
                    0b10 => vm.pc.wrapping_add(sext(vm.get(reg3))),
                    0b11 => addr,

                    _ => unreachable!(),
                };

                vm.pc_update(addr);
            }
        }
    } else {
        let reg2 = vm.get(reg2);
        vm.reg[reg3 as usize] = match I {
            0x8 => reg1 | reg2,
            0x9 => reg1 & reg2,
            0xa => reg1 ^ reg2,
            0xb => reg1 << reg2,
            0xc => reg1 >> reg2,
            0xd => reg1.wrapping_add(reg2),
            0xe => reg1,
            _ => unreachable!(),
        }
    }
}
