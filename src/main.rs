
mod frontend;
mod chip8;


use frontend::Screen;
use frontend::ScreenTrait;
use chip8::CPU;
use std::fs::File;
use std::io::prelude::*;


fn main() {

    let mut file = File::open("./tests/danm8ku.ch8").unwrap();

    let mut data = vec![];
    file.read_to_end(&mut data);

    let mut screen = Box::new(Screen::new("Hello Chip8"));
    let mut cpu = CPU::new(screen);

    cpu.load_program(data );
    cpu.run();




    println!("\nEnd of Running!");
}
