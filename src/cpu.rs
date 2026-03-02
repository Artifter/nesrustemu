use std::ops::Add;

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
    //Definicja registerow i RAMU
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
        //sprawdzam czy instrukcja manualnie nie zmienila program counter
        let program_counter_state = self.program_counter;

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
            0x0A | 0x06 | 0x16 | 0x0E | 0x1E => {
                self.asl(&opcode.mode);
            }
            0x4A | 0x46 | 0x56 | 0x4E | 0x5E => {
                self.lsr(&opcode.mode);
            }
            0x2A | 0x26 | 0x36 | 0x2E | 0x3E => {
                self.rol(&opcode.mode);
            }
            0x6A | 0x66 | 0x76 | 0x6E | 0x7E => {
                self.ror(&opcode.mode);
            }    
            
            //Bitwise instructions
            0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => {
                self.and(&opcode.mode);
            }
            0x09 | 0x05 | 0x15 | 0x0D | 0x1D | 0x19 | 0x01 | 0x11 => {
                self.ora(&opcode.mode);
            }
            0x49 | 0x45 | 0x55 | 0x4D | 0x5D | 0x59 | 0x41 | 0x51 => {
                self.eor(&opcode.mode);
            }
            0x24 | 0x2C => {
                self.bit(&opcode.mode);
            }
            
            //Compare instructions
            0xC9 | 0xC5 | 0xD5 | 0xCD | 0xDD | 0xD9 | 0xC1 | 0xD1 => {
                self.cmp(&opcode.mode);
            }
            0xE0 | 0xE4 | 0xEC => {
                self.cpx(&opcode.mode);
            }
            0xC0 | 0xC4 | 0xCC =>{
                self.cpy(&opcode.mode);
            }
            //Branch instructions
            0x90 => self.bcc(),
            0xB0 => self.bcs(),
            0xF0 => self.beq(),
            0xD0 => self.bne(),
            0x10 => self.bpl(),
            0x30 => self.bmi(),
            0x50 => self.bvc(),
            0x70 => self.bvs(),
            
            //Jump instructions
            0x4C | 0x6C => self.jmp(&opcode.mode),

            //Flagi
            0x18 => self.clc(),
            0x38 => self.sec(),
            
            
            _ => todo!()
        }
        if program_counter_state == self.program_counter {
            self.program_counter += (opcode.bytes - 1) as u16;
        }
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
            AddressingMode::Accumulator =>{
                panic!("mode Accumulator is not supported in get_operand_address");
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
        self.update_carry_flag(addition>255);
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
        self.update_carry_flag(addition>255);
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
    fn asl(&mut self, mode: &AddressingMode){
        match mode{
        AddressingMode::Accumulator =>{
            let value: u16 = self.register_a as u16;
            let shifted: u16 = value <<1;
            self.update_carry_flag(shifted>255);
            self.register_a = shifted as u8;
            self.update_zero_and_negative_flags(self.register_a as u8);
            }
        _ =>{
            let addr = self.get_operand_address(mode);
            let value: u16 = self.mem_read(addr) as u16;
            let shifted: u16 = value <<1;
            self.update_carry_flag(shifted>255);
            self.mem_write(addr, shifted as u8);
            self.update_zero_and_negative_flags(shifted as u8);
            }
        }
    }
    fn lsr(&mut self, mode: &AddressingMode){
        match mode{
        AddressingMode::Accumulator =>{
            let value: u8 = self.register_a;
            self.update_carry_flag(value&0b0000_0001 ==1);
            let shifted: u8 = value >> 1;
            self.register_a = shifted;
            self.update_zero_and_negative_flags(self.register_a);
            }
        _ =>{
            let addr = self.get_operand_address(mode);
            let value: u8 = self.mem_read(addr);
            self.update_carry_flag(value&0b0000_0001 ==1);
            let shifted: u8 = value >>1;
            self.mem_write(addr, shifted);
            self.update_zero_and_negative_flags(shifted);
            }
        }
    }
    fn rol(&mut self, mode: &AddressingMode){
        let old_carry = self.status & 0b0000_0001;
        match mode{
        AddressingMode::Accumulator =>{
            let value: u16 = self.register_a as u16;
            let shifted = value << 1 | old_carry as u16;
            self.update_carry_flag(shifted>255);
            self.register_a = shifted as u8;
            self.update_zero_and_negative_flags(self.register_a);
            }
        _ =>{
            let addr = self.get_operand_address(mode);
            let value: u16 = self.mem_read(addr) as u16;
            let shifted = value << 1 | old_carry as u16;
            self.update_carry_flag(shifted>255);
            self.mem_write(addr, shifted as u8);
            self.update_zero_and_negative_flags(shifted as u8);
            }
        }
    }
    fn ror(&mut self, mode: &AddressingMode){
        let old_carry = (self.status & 0b0000_0001) << 7;
        match mode{
        AddressingMode::Accumulator =>{
            let value = self.register_a;
            self.update_carry_flag(value & 0b0000_0001 ==1);
            let shifted: u8 = value >>1 | old_carry;
            self.register_a = shifted;
            self.update_zero_and_negative_flags(shifted);
        }
        _ =>{
            let addr = self.get_operand_address(mode);
            let value = self.mem_read(addr);
            self.update_carry_flag(value & 0b0000_0001 ==1);
            let shifted: u8 = value >>1 | old_carry;
            self.mem_write(addr, shifted);
            self.update_zero_and_negative_flags(shifted);
        } 
    }
    }

    //Funkcje bitwise
    fn and(&mut self, mode: &AddressingMode){
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.register_a = self.register_a & value;
        self.update_zero_and_negative_flags(self.register_a);
    }
    fn ora(&mut self, mode: &AddressingMode){
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.register_a = self.register_a | value;
        self.update_zero_and_negative_flags(self.register_a);
    }
    fn eor(&mut self, mode: &AddressingMode){
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.register_a = self.register_a ^ value;
        self.update_zero_and_negative_flags(self.register_a);
    }
    fn bit(&mut self, mode: &AddressingMode){
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        // zero - na podstawie A AND wartość
        if self.register_a & value == 0 {
            self.status |= 0b0000_0010;
        } else {
            self.status &= 0b1111_1101;
        }

        // negative - bit 7 samej wartości
        if value & 0b1000_0000 != 0 {
            self.status |= 0b1000_0000;
        } else {
            self.status &= 0b0111_1111;
        }

        // overflow - bit 6 samej wartości
        if value & 0b0100_0000 != 0 {
            self.status |= 0b0100_0000;
        } else {
            self.status &= 0b1011_1111;
        }
    }
    
    // Funkcja compare
    fn cmp(&mut self, mode: &AddressingMode){
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.update_carry_flag(self.register_a>= value);
        let result = self.register_a.wrapping_sub(value);
        self.update_zero_flag(result);
        self.update_negative_flag(result);
    }  
    fn cpx(&mut self, mode: &AddressingMode){
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.update_carry_flag(self.register_x>= value);
        let result = self.register_x.wrapping_sub(value);
        self.update_zero_flag(result);
        self.update_negative_flag(result);
    }  
    fn cpy(&mut self, mode: &AddressingMode){
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.update_carry_flag(self.register_y>= value);
        let result = self.register_y.wrapping_sub(value);
        self.update_zero_flag(result);
        self.update_negative_flag(result);
    }
    
    //Funkcje branch
    fn bcc(&mut self){
        let offset:i8 = self.mem_read(self.program_counter) as i8;
        
        //Carry flag check
        if (self.status & 0b0000_0001) == 0{
            self.program_counter = self.program_counter.wrapping_add_signed(offset as i16).wrapping_add(1);
        }
    }
    fn bcs(&mut self){
        let offset:i8 = self.mem_read(self.program_counter) as i8;
        
        //Carry flag check
        if (self.status & 0b0000_0001) != 0{
            self.program_counter = self.program_counter.wrapping_add_signed(offset as i16).wrapping_add(1);
        }
    }
    fn beq(&mut self){
        let offset:i8 = self.mem_read(self.program_counter) as i8;
        
        //zero flag check
        if (self.status & 0b0000_0010) != 0{
            self.program_counter = self.program_counter.wrapping_add_signed(offset as i16).wrapping_add(1);
        }
    }
    fn bne(&mut self){
       let offset:i8 = self.mem_read(self.program_counter) as i8;
        
        //zero flag check
        if (self.status & 0b0000_0010) == 0{
            self.program_counter = self.program_counter.wrapping_add_signed(offset as i16).wrapping_add(1);
        } 
    }
    fn bpl(&mut self){
        let offset:i8 = self.mem_read(self.program_counter) as i8;
        
        //negatice flag check
        if (self.status & 0b1000_0000) == 0{
            self.program_counter = self.program_counter.wrapping_add_signed(offset as i16).wrapping_add(1);
        }
    }
    fn bmi(&mut self){
        let offset:i8 = self.mem_read(self.program_counter) as i8;
        
        //negative flag check
        if (self.status & 0b1000_0000) != 0{
            self.program_counter = self.program_counter.wrapping_add_signed(offset as i16).wrapping_add(1);
        }

    }
    fn bvc(&mut self){
        let offset:i8 = self.mem_read(self.program_counter) as i8;
        
        //overflow flag check
        if (self.status & 0b0100_0000) == 0{
            self.program_counter = self.program_counter.wrapping_add_signed(offset as i16).wrapping_add(1);
        }

    }
    fn bvs(&mut self){
        let offset:i8 = self.mem_read(self.program_counter) as i8;
        
        //overflow flag check
        if (self.status & 0b0100_0000) != 0{
            self.program_counter = self.program_counter.wrapping_add_signed(offset as i16).wrapping_add(1);
        }

    }
    
    //Funkcje JMP
    fn jmp(&mut self, mode: &AddressingMode) {
        match mode {
        AddressingMode::NoneAddressing => {
            // Indirect z bugiem 6502
            let ptr = self.mem_read_u16(self.program_counter);
            let addr = if ptr & 0x00FF == 0x00FF {
                // bug - nie przechodzi na następną stronę
                let lo = self.mem_read(ptr);
                let hi = self.mem_read(ptr & 0xFF00);
                (hi as u16) << 8 | lo as u16
            } else {
                self.mem_read_u16(ptr)
            };
            self.program_counter = addr;
            }
            _ => {
                let addr = self.get_operand_address(mode);
                self.program_counter = addr;
            }
        }
    }
    
    
    //Funkcje flag
    fn clc(&mut self){
        self.status = self.status & 0b1111_1110;
    }
    fn sec(&mut self){
        self.status = self.status | 0b0000_0001;
    }
    
    
    
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
    
    fn update_carry_flag(&mut self, result: bool){
        if result{
            self.status = self.status | 0b0000_0001;
        }
        else{
            self.status = self.status & 0b1111_1110;
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
#[path = "cpu_tests.rs"]
mod test;





