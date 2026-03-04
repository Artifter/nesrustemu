const VRAM_START: u16 = 0x0000;
const VRAM_END: u16 =  0x1FFF;
const PPU_REGISTERS_START:u16 = 0x2000;
const PPU_REGISTERS_END:u16 = 0x3FFF;




pub struct Bus{
    cpu_vram: [u8;2048]
    ppu_registers[u8;8]
}

pub trait Mem{
    fn mem_read(&self, addr:u16) -> u8;
    fn mem_write(&mut self, addr: u16, data: u8);
}


impl Bus{
    pub fn new() -> Self{
        Bus{
        cpu_vram: [0;2048],
        ppu_registers: [0x2000;8],
        }
    }
}

impl Mem for Bus{
    fn mem_read(&self, addr:u16)-> u8{
        match addr{
            VRAM_START..=VRAM_END =>{
                let mirrored_addr = addr  & 0b0000_0111_1111_1111;
                self.cpu_vram[mirrored_addr as usize]
            }
            
            _ =>{addr as u8}
        }
        
    }
    fn mem_write(&mut self, addr: u16, data: u8){
        todo!()
    }
}