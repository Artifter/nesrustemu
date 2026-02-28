mod cpu;
mod opcodes;

use crate::cpu::CPU;


fn main() {
   let mut cpu = CPU::new();
    
        cpu.load(vec![0xe8, 0xe8, 0x00]);
        cpu.reset();
        cpu.run();
}
