

pub type Reg8 = u8;

#[derive(Debug, PartialEq)]
pub struct Reg16 {
  high: Reg8,
  low: Reg8,
}

impl From<u16> for Reg16 {
  fn from(item: u16) -> Self {
    Reg16 {
        high: (item >> 8) as u8,
        low: item as u8 & 0xFF
    }
  }
}

impl Into<u16> for Reg16 {
    fn into(self) -> u16 {
        ((self.high as u16) << 8) + self.low as u16
    }
}

impl Default for Reg16 {
    fn default() -> Self {
        Reg16 {
            high: 0,
            low: 0,
        }
    }
}

pub struct Flags {
    pub Z: bool,
    pub N: bool,
    pub H: bool,
    pub C: bool,
} impl Default for Flags {
    fn default() -> Self {
        Flags {
            Z: false,
            N: false,
            H: false,
            C: false,
        }
    }
}

pub struct Registers {
    pub A: Reg8,
    pub BC: Reg16,
    pub DE: Reg16,
    pub HL: Reg16,
    pub SP: u16,
    pub PC: u16,
    pub F: Flags,
} impl Registers {
    pub fn new() -> Self {
        Registers {
            A: 0,
            BC: Reg16::default(),
            DE: Reg16::default(),
            HL: Reg16::default(),
            SP: 0,
            PC: 0,
            F: Flags::default(),
        }
    }
}



// Tests

#[test]
fn test_reg16_from_u16() {
    let expected = Reg16 {
        high: 0x33,
        low: 0x66,
    };

    assert_eq!(expected, Reg16::from(0x3366));

    let expected = Reg16 {
        high: 0x14,
        low: 0x11,
    };

    assert_eq!(expected, Reg16::from(0x1411));
}

#[test]
fn test_reg16_into_u16() {
    let expected: u16 = Reg16 {
        high: 0x33,
        low: 0x66,
    }.into();

    assert_eq!(expected, 0x3366);

    let expected: u16 = Reg16 {
        high: 0x14,
        low: 0x11,
    }.into();

    assert_eq!(expected, 0x1411);
}