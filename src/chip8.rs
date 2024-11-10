


// this is the entry address of chip8, it means CPU will fetch the very first instruction that is
// stored at this address
const PC_START: u16 = 0x200;


#[derive(Debug)]
pub struct CPU {
    V: [u8; 16],   //  Vx where x = 0..F`
    I: u16,        //  I to store memory address
    PC: u16,       //  Program Counter
    SP: u8,        //  Stack Pointer
    DT: u8,        //  Delay Timer
    ST: u8,        //  Sound Timer
    stack: [u16; 16], //  Stack
    memory: [u8; 4096], //  4k memory
}




impl CPU {
    // create an instance of chip8 CPU
    pub fn new() -> CPU {
        CPU {
            V: [0; 16],
            I: 0,
            PC: 0,
            SP: 0,
            DT: 0,
            ST: 0,
            stack: [0; 16],
            memory: [0; 4096],
        }
    }

    /// load_program loads binary to the memory of chip8
    pub fn load_program(&mut self, program: Vec<u8>) {
        


    }


    pub fn execute(&mut self) {
        println!("I = {}", self.I);
    }
}

