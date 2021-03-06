use super::cpu::JumpType;
use std::fmt;

const ROM_ADDR: usize = 0x200;

#[derive(Default)]
pub struct Registers {
    reg_gp: [u8; 16],
    reg_i: u16,

    reg_delay: u8,
    reg_sound: u8,

    reg_pc: u16,
    reg_sp: u8,

    stack: [u16; 16],
}

impl fmt::Debug for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..8 {
            try!(writeln!(f, "V{}: {:#x}", i, self.reg_gp[i]));
        }

        for i in 8..15 {
            try!(writeln!(f, "V{}: {:#x}", i, self.reg_gp[i]));
        }

        try!(write!(f, "VF reg: {:#x} | ", self.reg_gp[15]));
        try!(write!(f, "I-register: {:#4x} | ", self.reg_i));
        try!(writeln!(f,
                      "Timers: Sound({:#4x}) Delay({:#4x})",
                      self.reg_sound,
                      self.reg_delay));

        try!(writeln!(f, "PC: {:#4x} | SP: {:#2x}", self.reg_pc, self.reg_sp));
        for i in 0..16 {
            writeln!(f, "SLVL {}: {:#4x}", i, self.stack[i]);
        }
        writeln!(f, "")
    }
}

impl Registers {
    pub fn new() -> Registers {
        let mut reg = Registers::default();
        reg.reg_pc = ROM_ADDR as u16;
        reg
    }

    pub fn write_register(&mut self, target_reg: u8, data_value: u8) {
        self.reg_gp[target_reg as usize] = data_value;
    }

    pub fn write_register_i(&mut self, data_value: u16) {
        self.reg_i = data_value;
    }

    pub fn write_delay_timer(&mut self, data_value: u8) {
        self.reg_delay = data_value;
    }

    pub fn write_sound_timer(&mut self, data_value: u8) {
        self.reg_sound = data_value;
    }

    pub fn read_register(&self, target_reg: u8) -> u8 {
        self.reg_gp[target_reg as usize]
    }

    pub fn read_register_i(&self) -> u16 {
        self.reg_i
    }

    pub fn read_delay_timer(&self) -> u8 {
        self.reg_delay
    }

    pub fn read_sound_timer(&self) -> u8 {
        self.reg_sound
    }

    pub fn read_pc(&self) -> u16 {
        self.reg_pc
    }

    pub fn increment_pc(&mut self) {
        self.reg_pc += 2;
    }

    pub fn set_vf(&mut self) {
        self.reg_gp[15] = 1;
    }

    pub fn clear_vf(&mut self) {
        self.reg_gp[15] = 0;
    }

    pub fn jump_to_address(&mut self, addr: u16, jump_type: JumpType) {
        match jump_type {
            JumpType::SUBROUTINE => {
                self.stack[self.reg_sp as usize] = self.reg_pc;
                self.reg_sp += 1;
            }
            JumpType::NORMAL => {}
        }
        self.reg_pc = addr;
    }

    pub fn return_from_subroutine(&mut self) {
        self.reg_pc = self.stack[(self.reg_sp - 1) as usize];
        self.reg_sp -= 1;
    }
}
