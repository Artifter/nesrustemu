const RAM_START: u16 = 0x0000;
const RAM_END: u16 =  0x1FFF;
const VRAM_START: u16 = 0x2008;
const VRAM_END: u16 = 0x3FFF;



pub struct Bus{
    cpu_vram: [u8;2048]
}

trait Mem{
    fn mem_read(&self, addr:u16) -> u8;
}


impl Bus{
    pub fn new() -> Self{
        Bus{
        cpu_vram: [0;2048]
        }
    }
}

impl Mem for Bus{
    fn mem_read(&self, addr:u16)-> u8{
        match{
            RAM_START

        }
        
    }
}