
use super::{
    rom::ROM,
    registers::Registers,
};

use std::{
    path::Path,
};

pub struct CPU {
    rom: ROM,
    memory: Vec<u8>,
    registers: Registers,

} impl CPU {

    pub fn new(romfile: &Path) -> Self {
        Self {
            rom: ROM::new(romfile),
            memory: Vec::with_capacity(30_000),
            registers: Registers::new(),
        }
    }

}