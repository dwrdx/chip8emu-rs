
mod frontend;
mod chip8;


use frontend::Screen;
use frontend::ScreenTrait;
use chip8::{CPU, Sprite};
use std::fs::File;
use std::io::prelude::*;
use std::sync::mpsc;


fn load_rom_and_spawn(filename: &str, tx:mpsc::Sender<Sprite>) {
    let mut cpu = CPU::new(tx);
    let mut file = File::open(filename).unwrap();
    let mut data = vec![];
    file.read_to_end(&mut data);

    std::thread::spawn(move || {
        cpu.load_program(data );
        cpu.run();
    });
}


fn main() {
    let (tx, rx) = mpsc::channel();

    load_rom_and_spawn("./tests/danm8ku.ch8", tx);

    let mut screen = Screen::new("Rust Chip8", rx);
    screen.render();
}
