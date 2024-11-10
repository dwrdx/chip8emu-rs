

mod chip8;
use chip8::CPU;
use std::fs::File;
use std::io::prelude::*;


fn main() {
    let mut cpu = CPU::new();

    let mut file = File::open("./tests/danm8ku.ch8").unwrap();

    let mut data = vec![];
    file.read_to_end(&mut data);

    for byte in &data {
        print!("{:02x}", byte); 
    }






    println!("\nEnd of Running!");
}
