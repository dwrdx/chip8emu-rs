

use std::{fmt, usize};
use rand::Rng;

// this is the entry address of chip8, it means CPU will fetch the very first instruction that is
// stored at this address
const PC_START: u16 = 0x200;
const SP_START: u8  = 0x10;


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


#[derive(Debug)]
pub struct Operand {
    op_code: u8,
    value: u16,
}


trait InstructionSet {
    fn increment_pc(&mut self);
    fn increment_sp(&mut self);
    fn decrement_sp(&mut self);

    fn clear_display_00E0(&mut self);
    fn return_from_subroutine_00EE(&mut self);
    fn jump_to_location_1nnn(&mut self, address: u16);
    fn call_subroutine_2nnn(&mut self, address: u16);
    fn skip_if_eq_3xkk(&mut self, value: u16);
    fn skip_if_neq_4xkk(&mut self, value: u16);
    fn skip_if_reg_eq_5xy0(&mut self, value: u16);
    fn load_value_to_reg_6xkk(&mut self, value: u16);
    fn add_byte_7xkk(&mut self, value: u16);
    fn store_reg_8xy0(&mut self, value: u16);
    fn or_reg_8xy1(&mut self, value: u16);
    fn and_reg_8xy2(&mut self, value: u16);
    fn xor_reg_8xy3(&mut self, value: u16);
    fn add_reg_8xy4(&mut self, value: u16);
    fn sub_reg_8xy5(&mut self, value: u16);
    fn shift_right_8xy6(&mut self, value: u16);
    fn sub_reg_8xy7(&mut self, value: u16);
    fn shift_left_8xyE(&mut self, value: u16);
    fn skip_if_reg_neq_9xy0(&mut self, value: u16);
    fn load_value_to_index_reg_Annn(&mut self, value: u16);
    fn jump_to_location_Bnnn(&mut self, value: u16);
    fn random_number_Cxkk(&mut self, value: u16);
    fn draw_sprite_Dxyn(&mut self, value: u16);
    fn skip_if_key_pressed_Ex9E(&mut self, value: u16);
    fn skip_if_key_not_pressed_ExA1(&mut self, value: u16);
    fn load_delay_timer_Fx07(&mut self, value: u16);
    fn wait_for_key_press_Fx0A(&mut self, value: u16);
    fn load_reg_to_delay_timer_Fx15(&mut self, value: u16);
    fn load_reg_to_sound_timer_Fx18(&mut self, value: u16);
    fn add_to_index_reg_Fx1E(&mut self, value: u16);
    fn set_index_reg_to_sprite_Fx29(&mut self, value: u16);
    fn store_binary_coded_decimal_Fx33(&mut self, value: u16);
    fn store_registers_Fx55(&mut self, value: u16);
    fn load_registers_Fx65(&mut self, value: u16);
}



impl Operand {
    pub fn new(instruciton: u16) -> Operand {
        Operand {
            op_code: (((instruciton&0xF000) >> 12 ) as u8),
            value: instruciton&0x0FFF,
        }
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "OPERAND(CODE: {:01X}, VAL: {:02X})", self.op_code, self.value)
    }
}







impl CPU {
    // create an instance of chip8 CPU
    pub fn new() -> CPU {
        CPU {
            V: [0; 16],
            I: 0,
            PC: PC_START,
            SP: SP_START,
            DT: 0,
            ST: 0,
            stack: [0; 16],
            memory: [0; 4096],
        }
    }

    /// load_program loads binary to the memory of chip8 starting from PC_START
    pub fn load_program(&mut self, program: Vec<u8>) {
        // load program to memory
        for (i, byte) in program.iter().enumerate() {
            self.memory[PC_START as usize + i] = *byte;
        }
    }

    // reset CPU
    pub fn reset(&mut self) {
        self.PC = PC_START;
        self.SP = SP_START;
    }

    // run CPU
    pub fn run(&mut self) {
        loop {
            let op = self.fetch();
            self.execute(op);
        }
    }


    // fetch the operand from PC
    pub fn fetch(&mut self) -> Operand {
        let operand: Operand = Operand::new(((self.memory[self.PC as usize] as u16) << 8)  + (self.memory[self.PC as usize + 1] as u16));
        operand
    }

    // execute operand
    pub fn execute(&mut self, operand: Operand) {
        let mut log = String::new();
        log.push_str(&format!("PC:{:04X}, {}, ", self.PC, operand));
        println!("{}", log);

        match operand.op_code {
            0x00 => {
                match operand.value {
                    0x0E0 => {
                        self.clear_display_00E0(); 
                    },
                    0x0EE => {
                        self.return_from_subroutine_00EE();

                    },
                    _ => {
                        panic!("Invalid operand!");
                    }
                }
            },
            0x01 => {
                self.jump_to_location_1nnn(operand.value);
            },
            0x02 => {
                self.call_subroutine_2nnn(operand.value);
            },
            0x03 => {
                self.skip_if_eq_3xkk(operand.value);
            },
            0x04 => {
                self.skip_if_neq_4xkk(operand.value);
            },
            0x06 => {
                self.load_value_to_reg_6xkk(operand.value);
            },
            0x07 => {
                self.add_byte_7xkk(operand.value);
            },
            0x0A => {
                self.load_value_to_index_reg_Annn(operand.value);
            },
            0x0D => {
                self.draw_sprite_Dxyn(operand.value);
            }
            0x0F => {
                let sub_op_code = operand.value & 0x00FF;
                match sub_op_code {
                    0x07 => {
                        self.load_delay_timer_Fx07(operand.value);
                    },
                    0x0A => {
                        self.wait_for_key_press_Fx0A(operand.value);
                    },
                    0x65 => {
                        self.load_registers_Fx65(operand.value);
                    },
                    _ => {
                        panic!("Invalid operand!");
                    }
                }

            },
            _ => {
                panic!("Invalid operand!");
            }
        }

    }

}

impl InstructionSet for CPU {
    fn increment_pc(&mut self) {
        self.PC += 2;
    }

    fn increment_sp(&mut self) {
        self.SP -= 1;
    }

    fn decrement_sp(&mut self) {
        self.SP += 1;
    }



    fn clear_display_00E0(&mut self) {
        // TODO: clear the display here
        self.increment_pc();
    }

    fn return_from_subroutine_00EE(&mut self) {
        self.PC = self.stack[self.SP as usize];
        self.decrement_sp();
    }

    fn jump_to_location_1nnn(&mut self, address: u16) {
        self.PC = address;
    }

    fn call_subroutine_2nnn(&mut self, address: u16) {
        self.increment_sp();
        self.stack[self.SP as usize] = self.PC;
        self.PC = address;
    }

    fn skip_if_eq_3xkk(&mut self, value: u16) {
        self.increment_pc();
        let kk = (value & 0x00FF) as u8;
        let x = ((value & 0x0F00) >> 8) as u8;
        if self.V[x as usize] == kk {
            self.increment_pc();
        }
    }

    fn skip_if_neq_4xkk(&mut self, value: u16) {
        self.increment_pc();
        let kk = (value & 0x00FF) as u8;
        let x = ((value & 0x0F00) >> 8) as u8;
        if self.V[x as usize] != kk {
            self.increment_pc();
        }
    }

    fn skip_if_reg_eq_5xy0(&mut self, value: u16) {
        self.increment_pc();
        let x = ((value & 0x0F00) >> 8) as u8;
        let y = ((value & 0x00F0) >> 4) as u8;
        if self.V[x as usize] == self.V[y as usize] {
            self.increment_pc();
        }
    }

    fn load_value_to_reg_6xkk(&mut self, value: u16) {
        let kk = (value & 0x00FF) as u8;
        let x = ((value & 0x0F00) >> 8) as u8;
        self.V[x as usize] = kk;
        self.increment_pc();
    }

    fn add_byte_7xkk(&mut self, value: u16) {
        let kk = (value & 0x00FF) as u8;
        let x = ((value & 0x0F00) >> 8) as u8;
        self.V[x as usize] = self.V[x as usize].wrapping_add(kk);
        self.increment_pc();
    }

    fn store_reg_8xy0(&mut self, value: u16) {
        let x = ((value & 0x0F00) >> 8) as u8;
        let y = ((value & 0x00F0) >> 4) as u8;
        self.V[x as usize] = self.V[y as usize];
        self.increment_pc();
    }
    
    fn or_reg_8xy1(&mut self, value: u16) {
        let x = ((value & 0x0F00) >> 8) as u8;
        let y = ((value & 0x00F0) >> 4) as u8;
        self.V[x as usize] |= self.V[y as usize];
        self.increment_pc();
    }

    fn and_reg_8xy2(&mut self, value: u16) {
        let x = ((value & 0x0F00) >> 8) as u8;
        let y = ((value & 0x00F0) >> 4) as u8;
        self.V[x as usize] &= self.V[y as usize];
        self.increment_pc();
    }

    fn xor_reg_8xy3(&mut self, value: u16) {
        let x = ((value & 0x0F00) >> 8) as u8;
        let y = ((value & 0x00F0) >> 4) as u8;
        self.V[x as usize] ^= self.V[y as usize];
        self.increment_pc();
    }

    fn add_reg_8xy4(&mut self, value: u16) {
        let x = ((value & 0x0F00) >> 8) as u8;
        let y = ((value & 0x00F0) >> 4) as u8;
        self.V[0xF] = if self.V[x as usize] + self.V[y as usize] > 255 { 1 } else { 0 };
        self.V[x as usize] += self.V[y as usize];
        self.increment_pc();
    }

    fn sub_reg_8xy5(&mut self, value: u16) {
        let x = ((value & 0x0F00) >> 8) as u8;
        let y = ((value & 0x00F0) >> 4) as u8;
        self.V[0xF] = if self.V[x as usize] > self.V[y as usize] { 1 } else { 0 };
        self.V[x as usize] -= self.V[y as usize];
        self.increment_pc();
    }

    fn shift_right_8xy6(&mut self, value: u16) {
        let x = ((value & 0x0F00) >> 8) as u8;
        self.V[0xF] = self.V[x as usize] & 0x1;
        self.V[x as usize] >>= 1;
        self.increment_pc();
    }

    fn sub_reg_8xy7(&mut self, value: u16) {
        let x = ((value & 0x0F00) >> 8) as u8;
        let y = ((value & 0x00F0) >> 4) as u8;
        self.V[0xF] = if self.V[y as usize] > self.V[x as usize] { 1 } else { 0 };
        self.V[x as usize] = self.V[y as usize] - self.V[x as usize];
        self.increment_pc();
    }

    fn shift_left_8xyE(&mut self, value: u16) {
        let x = ((value & 0x0F00) >> 8) as u8;
        self.V[0xF] = self.V[x as usize] >> 7;
        self.V[x as usize] <<= 1;
        self.increment_pc();
    }

    fn skip_if_reg_neq_9xy0(&mut self, value: u16) {
        self.increment_pc();
        let x = ((value & 0x0F00) >> 8) as u8;
        let y = ((value & 0x00F0) >> 4) as u8;
        if self.V[x as usize] != self.V[y as usize] {
            self.increment_pc();
        }
    }

    fn load_value_to_index_reg_Annn(&mut self, value: u16) {
        self.I = value;
        self.increment_pc();
    }

    fn jump_to_location_Bnnn(&mut self, value: u16) {
        self.PC = value + self.V[0] as u16;
    }

    fn random_number_Cxkk(&mut self, value: u16) {
        let x = ((value & 0x0F00) >> 8) as u8;
        let kk = (value & 0x00FF) as u8;
        let mut rng = rand::thread_rng();
        let n: u8 = rng.gen_range(0..255);
        self.V[x as usize] = n & kk;
        self.increment_pc();
    }

    fn draw_sprite_Dxyn(&mut self, value: u16) {
        // TODO: display

        self.increment_pc();
    }

    fn skip_if_key_pressed_Ex9E(&mut self, value: u16) {
        // TODO: check keyboard

        self.increment_pc();
    }

    fn skip_if_key_not_pressed_ExA1(&mut self, value: u16) {
        // TODO: check keyboard

        self.increment_pc();
    }

    fn load_delay_timer_Fx07(&mut self, value: u16) {
        let x = ((value & 0x0F00) >> 8) as u8;
        self.V[x as usize] = self.DT;
        self.increment_pc();
    }

    fn wait_for_key_press_Fx0A(&mut self, value: u16) {
        // TODO: check keyboard
        self.increment_pc();
    }

    fn load_reg_to_delay_timer_Fx15(&mut self, value: u16) {
        let x = ((value & 0x0F00) >> 8) as u8;
        self.DT = self.V[x as usize];
        self.increment_pc();
    }

    fn load_reg_to_sound_timer_Fx18(&mut self, value: u16) {
        let x = ((value & 0x0F00) >> 8) as u8;
        self.ST = self.V[x as usize];
        self.increment_pc();
    }

    fn add_to_index_reg_Fx1E(&mut self, value: u16) {
        let x = ((value & 0x0F00) >> 8) as u8;
        self.I += self.V[x as usize] as u16;
        self.increment_pc();
    }

    fn set_index_reg_to_sprite_Fx29(&mut self, value: u16) {
        // TODO: display

        self.increment_pc();
    }

    fn store_binary_coded_decimal_Fx33(&mut self, value: u16) {
        // TODO: fix me
        self.increment_pc();
    }

    fn store_registers_Fx55(&mut self, value: u16) {
        // TODO: fix me
        self.increment_pc();
    }

    fn load_registers_Fx65(&mut self, value: u16) {
        // TODO: fix me
        self.increment_pc()
    }

}

