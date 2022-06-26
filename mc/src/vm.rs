/*
        ("halt", 0), // 0x0 0x00
        ("trap", 1), // 0x0 addr
        ("cmp", 2),  // reg 0x00
        ("puth", 3), // reg val
        ("putl", 4), // reg val
        ("brk", 5),
        // 0b0 cnd reg reg
        // 0b1 imm11
        ("lea", 6),  // reg imm8
        ("ld", 7),   // reg 0x0 reg
        ("st", 8),   // reg 0x0 reg
        ("mv", 9),   // reg 0x0 reg
        ("not", 10), // reg 0x0 reg
        ("and", 11), // reg reg reg
        ("or", 12),  // reg reg reg
        ("ls", 13),  // reg reg reg
        ("rs", 14),  // reg reg reg
        ("add", 15), // reg reg reg
*/

use crate::trap::TRAP;

const PSR: usize = 0xf;
const PC: usize = 0xe;

pub struct Vm {
    pub(crate) reg: [u16; 0x10],
    pub(crate) mem: [u16; 0x10000],
}

impl Vm {
    #[inline]
    pub fn new(mem: [u16; 0x10000]) -> Self {
        let mut reg = [0; 0x10];

        reg[PSR] = 0b1110;

        Self { reg, mem }
    }

    pub fn step(&mut self) {
        let instr = self.load();

        let f = [
            op::<0x0>, op::<0x1>, op::<0x2>, op::<0x3>, op::<0x4>, op::<0x5>, op::<0x6>, op::<0x7>,
            op::<0x8>, op::<0x9>, op::<0xa>, op::<0xb>, op::<0xc>, op::<0xd>, op::<0xe>, op::<0xf>,
        ];

        f[(instr >> 12) as usize](self, instr & 0xfff);
    }

    pub fn halted(&self) -> bool {
        self.reg[PSR] & 1 > 0
    }

    fn load(&mut self) -> u16 {
        let pc = self.reg[PC];
        //println!("0x{:x}", pc);
        self.pc_update(pc + 1);
        self.mem[usize::from(pc)]
    }

    fn pc_update(&mut self, addr: u16) {
        self.reg[PC] = addr;
    }
}

fn op<const I: u8>(vm: &mut Vm, args: u16) {
    let (r1, r2, r3) = (args >> 8, (args >> 4) & 0xf, args & 0xf);
    let (r1, r2, r3) = (r1 as usize, r2 as usize, r3 as usize);
    let val = args & 0xff;
    let sext = |val, size| val | (val >> (size - 1)) * (0xffff_u16 << size);

    if I < 6 {
        match I {
            0 => vm.reg[PSR] = 1,
            1 => TRAP[args as usize](vm),
            2 => {
                vm.reg[PSR] = if vm.reg[r1] == 0 {
                    0b010
                } else {
                    [0b100, 0b001][(vm.reg[r1] >> 15) as usize]
                } << 1
            }
            3 => vm.reg[r1] = (val << 8) | (vm.reg[r1] & 0xff),
            4 => vm.reg[r1] = (vm.reg[r1] & 0xff00) | val,

            5 => {
                if (vm.reg[PSR] >> 1) & 7 & r1 as u16 > 0 {
                    vm.reg[PC] = if r1 > 7 {
                        vm.reg[PC].wrapping_add(sext(val, 8))
                    } else {
                        vm.reg[r2]
                    };
                } else {
                    vm.reg[PC] = vm.reg[r3];
                }
            }
            _ => unreachable!(),
        }
        return;
    }

    let (reg2, reg3) = (vm.reg[r2], vm.reg[r3]);

    vm.reg[r1] = match I {
        6 => vm.reg[PC].wrapping_add(sext(val, 8)),
        7 => vm.mem[reg3 as usize],
        8 => {
            vm.mem[vm.reg[r1] as usize] = reg3;
            return;
        }

        9 => reg3,
        10 => !reg3,
        11 => reg2 & reg3,
        12 => reg2 | reg3,
        13 => reg2 << r3,
        14 => reg2 >> r3,
        15 => reg2.wrapping_add(reg3),

        _ => unreachable!(),
    };
}
