use mc::vm::Vm;

fn main() {
    let mut mem = [0; 0x10000];
    mem[0] = 0b0011_0000;
    mem[1] = b'h';
    mem[2] = 0b0010_0000;

    let mut vm = Vm::new(mem);

    while !vm.halted() {
       vm.step();
    }
}
