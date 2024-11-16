
mod frontend;
mod chip8;


use frontend::Screen;
use frontend::ScreenTrait;
use chip8::CPU;
use std::fs::File;
use std::io::prelude::*;
use std::sync::mpsc;


fn main() {

    let mut file = File::open("./tests/danm8ku.ch8").unwrap();

    let mut data = vec![];
    file.read_to_end(&mut data);

    let (tx, rx) = mpsc::channel();

    let mut screen = Screen::new("Hello Chip8", rx);
    let mut cpu = CPU::new(tx);

    std::thread::spawn(move || {
        cpu.load_program(data );
        cpu.run();
    });
    screen.render();


    println!("\nEnd of Running!");
}
