#![warn(clippy::all)]
use super::instruction::{AddressingMode, Instruction, Opcode};
// consts for operations, addresses etc

pub struct cpu {
    acc: u8,
    x: u8,
    y: u8,
    pc: u16,
    sp: u8,
    p: u8,
    counter: usize,
    clocks: usize,
    clocks_to_pause: u16,
}

impl cpu {
    pub fn new() -> cpu {
        return cpu {
            acc: 0,
            x: 0,
            y: 0,
            pc: 0xC000,
            sp: 0xfd,
            p: 0,
            counter: 0,
            clocks: 0,
            clocks_to_pause: 6,
        };
    }
    pub fn initialise(&mut self) {}
    fn clock(&mut self) {
        self.counter += 1;
        if self.clocks_to_pause > 0 {
            self.clocks_to_pause -= 1;
            return;
        }
        let ptr = self.pc;
    }
    pub fn run_instructions(&mut self, n: usize) {
        for _i in 0..n {
            self.clocks_to_pause = 0;
            self.clock();
        }
    }
    pub fn run_instruction(&mut self, instruction: &Instruction) {
        let v = i.mode_arguments;
        match instruction.op {
            Opcode::ADC => {
                let (x1, o1) = v.overflowing_add(self.acc)
            }
            Opcode::ASL => {}
            Opcode::LSR 
        }
    }
}
