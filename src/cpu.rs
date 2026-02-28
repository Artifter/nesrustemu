use crate::opcodes::CPU_OPS_CODES;
use crate::opcodes::AddressingMode;
/*
                    7  bit  0
                    ---- ----
                    NV1B DIZC
                    |||| ||||
                    |||| |||+- Carry
                    |||| ||+-- Zero
                    |||| |+--- Interrupt Disable
                    |||| +---- Decimal
                    |||+------ (No CPU effect; see: the B flag)
                    ||+------- (No CPU effect; always pushed as 1)
                    |+-------- Overflow
                    +--------- Negative
                    */



pub struct CPU{
    pub register_a: u8,
    pub register_x: u8,
    pub register_y:u8,
    pub stack_pointer: u8,
    pub status: u8,
    pub program_counter: u16,
    memory: [u8; 0x10000]
}
impl CPU {
    //Definicja pointerow i RAMU
    pub fn new() -> Self{
        CPU { 
            register_a:0, 
            register_x:0,
            register_y:0,
            stack_pointer: 0xFF,
            status: 0, 
            program_counter: 0,
            memory: [0;0x10000]
        }
    }
    //PAMIEC
    fn mem_read(&mut self, addr:u16) -> u8{
        self.memory[addr as usize]
    } 
    fn mem_read_u16(&mut self, pos:u16) -> u16{
        let lo = self.mem_read(pos) as u16;
        let hi = self.mem_read(pos.wrapping_add(1)) as u16;
        (hi<<8) | (lo as u16)
    }
    fn mem_write_u16(&mut self, pos:u16, data: u16){
        let hi = (data >> 8) as u8;
        let lo = (data & 0xff) as u8;
        self.mem_write(pos, lo);
        self.mem_write(pos.wrapping_add(1), hi);
    }
    fn mem_write(&mut self, addr: u16, data: u8){
        self.memory[addr as usize] = data;
    }
    // OPERACJE NA STACKU
    fn get_stack_addr(&self)->u16{
        0x0100 | self.stack_pointer as u16
    }
    fn stack_push(&mut self, data: u8){
        self.mem_write(self.get_stack_addr(), data);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1)
    }
    fn stack_pop(&mut self) -> u8{
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        self.mem_read(self.get_stack_addr())
    }
    
    pub fn reset(&mut self){
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.stack_pointer = 0xFF;
        self.status = 0;
        self.program_counter = self.mem_read_u16(0xFFFC);
    }
    
    pub fn load(&mut self, program: Vec<u8>){
        self.memory[0x8000 .. (0x8000 + program.len())].copy_from_slice(&program[..]);
        self.mem_write_u16(0xFFFC, 0x8000);
    }
    pub fn load_and_run(&mut self, program: Vec<u8>){
        self.load(program);
        self.reset();
        self.run();
    }

   pub fn run(&mut self) {
    loop {
        let loaded_code = self.mem_read(self.program_counter);
        self.program_counter += 1;
        
        let opcode = CPU_OPS_CODES.iter().find(|op| op.code == loaded_code)
            .expect("nieznany opcode!");
        
        match loaded_code {
            0x00 => return,
            //transfer instructions 
            0xAA => self.tax(),
            0x8A => self.txa(),
            0xA8 => self.tay(),
            0x98 => self.tya(),
            
            //access instructions
            0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => {
                self.lda(&opcode.mode);
            }
            0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 =>{
                self.sta(&opcode.mode);
            }
            0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE =>{
                self.ldx(&opcode.mode);
            }
            0x86 | 0x96 | 0x8E =>{
                self.stx(&opcode.mode);
            }
            0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC =>{
                self.ldy(&opcode.mode);
            }
            0x84 | 0x94 | 0x8C =>{
                self.sty(&opcode.mode);
            }
            
            //arithmetic instruction
            0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => {
                self.adc(&opcode.mode);
            }
            0xE9 | 0xE5 | 0xF5 | 0xED | 0xFD | 0xF9 | 0xE1 | 0xF1 => {
                self.sbc(&opcode.mode);
            }
            0xE6 | 0xF6 | 0xEE | 0xFE => {
                self.inc(&opcode.mode);
            }
            0xC6 | 0xD6 | 0xCE | 0xDE => {
                self.dec(&opcode.mode);
            }
            0xE8 => self.inx(),
            0xCA => self.dex(),
            0xC8 =>self.iny(),
            0x88 =>self.dey(),
            
            //shift instructions
            _ => todo!()
        }
        self.program_counter += (opcode.bytes - 1) as u16;
    }
}
    
    
    fn get_operand_address(&mut self, mode:&AddressingMode)->u16{
        match mode{
            AddressingMode::Immediate => self.program_counter,
            AddressingMode::ZeroPage => self.mem_read(self.program_counter) as u16,
            AddressingMode::Absolute => self.mem_read_u16(self.program_counter),
            AddressingMode::ZeroPage_X =>
            {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_x) as u16;
                addr
            }
            AddressingMode::ZeroPage_Y =>{
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_y) as u16;
                addr
            }
            AddressingMode::Absolute_X=>{
                let base = self.mem_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_x as u16);
                addr
            }
            AddressingMode::Absolute_Y=>{
                let base = self.mem_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_y as u16);
                addr
            }

            AddressingMode::Indirect_X=>{
                let base: u8 = self.mem_read(self.program_counter);

                let ptr: u8 = (base as u8).wrapping_add(self.register_x);
                let lo = self.mem_read(ptr as u16);
                let hi: u8 = self.mem_read(ptr.wrapping_add(1) as u16);
                (hi as u16) <<8 | (lo as u16)
            }
            AddressingMode::Indirect_Y => {
               let base = self.mem_read(self.program_counter);

               let lo = self.mem_read(base as u16);
               let hi = self.mem_read((base as u8).wrapping_add(1) as u16);
               let deref_base = (hi as u16) << 8 | (lo as u16);
               let deref = deref_base.wrapping_add(self.register_y as u16);
               deref
           }
            AddressingMode::NoneAddressing => {
               panic!("mode {:?} is not supported", mode);
           }

        }
    }

    // Funkcje procesora access
    fn lda(&mut self, mode:&AddressingMode){
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        
        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }
    fn sta(&mut self, mode:&AddressingMode){
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_a);
    }
    fn ldx(&mut self, mode:&AddressingMode){
        let addr = self.get_operand_address(mode);
        self.register_x = self.mem_read(addr);
        self.update_zero_and_negative_flags(self.register_x);
    }
    fn stx(&mut self, mode:&AddressingMode){
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_x);
    }
    fn ldy(&mut self, mode:&AddressingMode){
        let addr = self.get_operand_address(mode);
        self.register_y = self.mem_read(addr);
        self.update_zero_and_negative_flags(self.register_y);
    }
    fn sty(&mut self, mode:&AddressingMode){
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_y);
    }
    // Funkcje Transfer
    fn tax(&mut self){
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }
    fn txa(&mut self){
        self.register_a = self.register_x;
        self.update_zero_and_negative_flags(self.register_a);
    }
    fn tay(&mut self){
        self.register_y = self.register_a;
        self.update_zero_and_negative_flags(self.register_y);
    }
    fn tya(&mut self){
        self.register_a = self.register_y;
        self.update_zero_and_negative_flags(self.register_a);
    }
    //Funkcje arithmetic
    fn inx(&mut self){
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x);
    }
    fn dex(&mut self){
        self.register_x = self.register_x.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.register_x);
    }
    fn iny(&mut self){
        self.register_y = self.register_y.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_y);
    }
    fn dey(&mut self){
        self.register_y = self.register_y.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.register_y);
    }
    fn adc(&mut self, mode: &AddressingMode){
        let addr = self.get_operand_address(mode);
        let value:u16 = self.mem_read(addr) as u16;
        let addition:u16 = (self.register_a as u16).wrapping_add(value).wrapping_add((self.status & 0b0000_0001) as u16);
        //flaga carry
        self.update_carry_flag(addition);
        //flaga overflow
        self.update_overflow_flag(addition, value);
        self.register_a = addition as u8;
        self.update_zero_and_negative_flags(self.register_a);
    }   
    fn sbc(& mut self, mode: &AddressingMode){
        let addr = self.get_operand_address(mode);
        let value:u16 = (!self.mem_read(addr)) as u16;
        let addition:u16 = (self.register_a as u16).wrapping_add(value).wrapping_add((self.status & 0b0000_0001) as u16);
        //flaga carry
        self.update_carry_flag(addition);
        //flaga overflow
        self.update_overflow_flag(addition, value);
        self.register_a = addition as u8;
        self.update_zero_and_negative_flags(self.register_a);
    }
    fn inc(&mut self, mode: &AddressingMode){
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        let result = value.wrapping_add(1);
        self.mem_write(addr, result);
        self.update_zero_and_negative_flags(result);
    }
    fn dec(&mut self, mode: &AddressingMode){
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        let result = value.wrapping_sub(1);
        self.mem_write(addr, result);
        self.update_zero_and_negative_flags(result);
    }
    //Funkcje Shift
    
    // Ustawianie flag
    fn update_zero_and_negative_flags(&mut self, result: u8){
        self.update_zero_flag(result);
        //negative
        self.update_negative_flag(result);
    }
    fn update_zero_flag(&mut self, result:u8){
        if result == 0{
            self.status = self.status | 0b0000_0010;
        }
        else{
            self.status = self.status & 0b1111_1101;
        }
    }
    fn update_negative_flag(&mut self, result:u8){
        if result & 0b1000_0000 != 0{
            self.status = self.status | 0b1000_0000;
            }
        else{
            self.status = self.status & 0b0111_1111;
        }
    }
    fn update_carry_flag(&mut self, result: u16){
        if result > 255
        {
            self.status = self.status | 0b0000_0001;
        }
        else{
            self.status = self.status & 0b1111_1110
        }

    }
    fn update_overflow_flag(&mut self, result: u16, value:u16){
        let bit7_a: u16 = (self.register_a as u16) & 0b1000_0000;
        let bit7_value: u16 = value & 0b1000_0000;
        let bit7_result: u16 = result & 0b1000_0000;
        if (bit7_a != bit7_result) && (bit7_value != bit7_result){
            self.status = self.status | 0b0100_0000
        }
        else{
            self.status = self.status & 0b1011_1111
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    mod lda {
        use super::*;

        #[test]
        fn immediate_load_data() {
            let mut cpu = CPU::new();
            cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
            assert_eq!(cpu.register_a, 0x05);
            assert!(cpu.status & 0b0000_0010 == 0b00);
            assert!(cpu.status & 0b1000_0000 == 0);
        }

        #[test]
        fn zero_flag() {
            let mut cpu = CPU::new();
            cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
            assert!(cpu.status & 0b0000_0010 == 0b10);
        }
    }

    mod tax {
        use super::*;

        #[test]
        fn move_a_to_x() {
            let mut cpu = CPU::new();
            cpu.load(vec![0xaa, 0x00]);
            cpu.reset();
            cpu.register_a = 10;
            cpu.run();
            assert_eq!(cpu.register_x, 10);
        }
    }

    mod inx {
        use super::*;

        #[test]
        fn overflow() {
            let mut cpu = CPU::new();
            cpu.load(vec![0xe8, 0xe8, 0x00]);
            cpu.reset();
            cpu.register_x = 0xff;
            cpu.run();
            assert_eq!(cpu.register_x, 1);
        }
    }

    mod sta {
        use super::*;

        #[test]
        fn zero_page() {
            let mut cpu = CPU::new();
            cpu.load_and_run(vec![0xa9, 0x42, 0x85, 0x10, 0x00]);
            assert_eq!(cpu.mem_read(0x10), 0x42);
        }
    }

    mod integration {
        use super::*;

        #[test]
        fn five_ops_working_together() {
            let mut cpu = CPU::new();
            cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
            assert_eq!(cpu.register_x, 0xc1);
        }
    }

    mod adc {
        use super::*;

        #[test]
        fn basic() {
            let mut cpu = CPU::new();
            cpu.load_and_run(vec![0xa9, 10, 0x69, 5, 0x00]);
            assert_eq!(cpu.register_a, 15);
        }

        #[test]
        fn carry_set() {
            let mut cpu = CPU::new();
            cpu.load_and_run(vec![0xa9, 200, 0x69, 100, 0x00]);
            assert_eq!(cpu.register_a, 44);
            assert!(cpu.status & 0b0000_0001 != 0);
        }

        #[test]
        fn zero_flag() {
            let mut cpu = CPU::new();
            cpu.load_and_run(vec![0xa9, 0, 0x69, 0, 0x00]);
            assert!(cpu.status & 0b0000_0010 != 0);
        }

        #[test]
        fn negative_flag() {
            let mut cpu = CPU::new();
            cpu.load_and_run(vec![0xa9, 200, 0x69, 10, 0x00]);
            assert!(cpu.status & 0b1000_0000 != 0);
        }

        #[test]
        fn overflow_positive() {
            let mut cpu = CPU::new();
            cpu.load_and_run(vec![0xa9, 127, 0x69, 1, 0x00]);
            assert!(cpu.status & 0b0100_0000 != 0);
        }

        #[test]
        fn no_overflow() {
            let mut cpu = CPU::new();
            cpu.load_and_run(vec![0xa9, 50, 0x69, 30, 0x00]);
            assert!(cpu.status & 0b0100_0000 == 0);
        }

        #[test]
        fn with_carry() {
            let mut cpu = CPU::new();
            cpu.load_and_run(vec![0xa9, 200, 0x69, 100, 0xa9, 10, 0x69, 5, 0x00]);
            assert_eq!(cpu.register_a, 16);
        }

        #[test]
        fn overflow_negative() {
            let mut cpu = CPU::new();
            cpu.load_and_run(vec![0xa9, 0xB0, 0x69, 0xA6, 0x00]);
            assert!(cpu.status & 0b0100_0000 != 0);
        }

        #[test]
        fn carry_clears() {
            let mut cpu = CPU::new();
            cpu.load_and_run(vec![0xa9, 200, 0x69, 100, 0xa9, 10, 0x69, 5, 0x00]);
            assert!(cpu.status & 0b0000_0001 == 0);
        }

        #[test]
        fn zero_page() {
            let mut cpu = CPU::new();
            cpu.load_and_run(vec![0xa9, 42, 0x85, 0x10, 0xa9, 10, 0x65, 0x10, 0x00]);
            assert_eq!(cpu.register_a, 52);
        }
    }
        mod sbc {
        use super::*;

        // 10 - 5 = 5 (carry ustawione przez poprzednie 0xFF + 0x01)
        #[test]
        fn basic() {
            let mut cpu = CPU::new();
            // 0xFF + 0x01 → carry = 1, potem LDA 10, SBC 5
            cpu.load_and_run(vec![0xa9, 0xFF, 0x69, 0x01, 0xa9, 10, 0xE9, 5, 0x00]);
            assert_eq!(cpu.register_a, 5);
        }

        // 3 - 10 = -7 (carry = 1 przed SBC)
        #[test]
        fn negative_result() {
            let mut cpu = CPU::new();
            cpu.load_and_run(vec![0xa9, 0xFF, 0x69, 0x01, 0xa9, 3, 0xE9, 10, 0x00]);
            assert!(cpu.status & 0b1000_0000 != 0); // negative = 1
        }

        // 5 - 5 = 0
        #[test]
        fn zero_flag() {
            let mut cpu = CPU::new();
            cpu.load_and_run(vec![0xa9, 0xFF, 0x69, 0x01, 0xa9, 5, 0xE9, 5, 0x00]);
            assert!(cpu.status & 0b0000_0010 != 0); // zero = 1
        }

        // 3 - 10 → underflow → carry = 0
        #[test]
        fn carry_clear_on_underflow() {
            let mut cpu = CPU::new();
            cpu.load_and_run(vec![0xa9, 0xFF, 0x69, 0x01, 0xa9, 3, 0xE9, 10, 0x00]);
            assert!(cpu.status & 0b0000_0001 == 0); // carry = 0
        }

        // 10 - 5 → no underflow → carry = 1
        #[test]
        fn carry_set_no_underflow() {
            let mut cpu = CPU::new();
            cpu.load_and_run(vec![0xa9, 0xFF, 0x69, 0x01, 0xa9, 10, 0xE9, 5, 0x00]);
            assert!(cpu.status & 0b0000_0001 != 0); // carry = 1
        }

        // overflow: 127 - (-1) = 128 → plus - minus = minus → overflow
        #[test]
        fn overflow() {
            let mut cpu = CPU::new();
            cpu.load_and_run(vec![0xa9, 0xFF, 0x69, 0x01, 0xa9, 127, 0xE9, 0xFF, 0x00]);
            assert!(cpu.status & 0b0100_0000 != 0); // overflow = 1
        }

        // brak overflow: 50 - 30 = 20
        #[test]
        fn no_overflow() {
            let mut cpu = CPU::new();
            cpu.load_and_run(vec![0xa9, 0xFF, 0x69, 0x01, 0xa9, 50, 0xE9, 30, 0x00]);
            assert!(cpu.status & 0b0100_0000 == 0); // overflow = 0
        }

        // SBC bez carry (carry = 0): 10 - 5 - 1 = 4
        #[test]
        fn without_carry() {
            let mut cpu = CPU::new();
            // nie ustawiamy carry, więc SBC odejmie dodatkowe 1
            cpu.load_and_run(vec![0xa9, 10, 0xE9, 5, 0x00]);
            assert_eq!(cpu.register_a, 4);
        }

        // SBC ZeroPage
        #[test]
        fn zero_page() {
            let mut cpu = CPU::new();
            // ustaw carry, zapisz 42 pod 0x10, LDA 50, SBC z 0x10
            cpu.load_and_run(vec![
                0xa9, 0xFF, 0x69, 0x01,  // carry = 1
                0xa9, 42, 0x85, 0x10,     // STA 0x10
                0xa9, 50, 0xE5, 0x10,     // LDA 50, SBC ZeroPage 0x10
                0x00
            ]);
            assert_eq!(cpu.register_a, 8); // 50 - 42 = 8
        }
    }
}





