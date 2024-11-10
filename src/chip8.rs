



#[derive(Debug)]
pub struct CPU {
    V: [u8; 16],   //  Vx where x = 0..F`
    I: u16,        //  I to store memory address
    PC: u16,       //  Program Counter
    SP: u8,        //  Stack Pointer
    DT: u8,        //  Delay Timer
    ST: u8,        //  Sound Timer
    stack: [u16; 16], //  Stack
}




impl CPU {
    pub fn new() -> CPU {
        CPU {
            V: [0; 16],
            I: 0,
            PC: 0,
            SP: 0,
            DT: 0,
            ST: 0,
            stack: [0; 16],
        }
    }
}













pub fn hello() {
    println!("hello");
}
