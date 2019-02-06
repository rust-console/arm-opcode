/// Mnemonics for the conditionals.
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cond {
  /// * Meaning: Equal
  /// * Flags: Z set
  EQ = 0,
  /// * Meaning: Not equal
  /// * Flags: Z clear
  NE = 1,
  /// * Meaning: Carry set / Unsigned higher or same
  /// * Flags: C set
  CSHS = 2,
  /// * Meaning: Carry clear / Unsigned lower
  /// * Flags: C clear
  CCLO = 3,
  /// * Meaning: Minus / negative
  /// * Flags: N set
  MI = 4,
  /// * Meaning: Plus / positive or zero
  /// * Flags: N clear
  PL = 5,
  /// * Meaning: Overflow
  /// * Flags: V set
  VS = 6,
  /// * Meaning: No overflow
  /// * Flags: V clear
  VC = 7,
  /// * Meaning: Unsigned higher
  /// * Flags: C set && Z clear
  HI = 8,
  /// * Meaning: Unsigned lower or same
  /// * Flags: C clear || Z set
  LS = 9,
  /// * Meaning: Signed greater than or equal to
  /// * Flags: N == V
  GE = 10,
  /// * Meaning: Signed less than
  /// * Flags: N != V
  LT = 11,
  /// * Meaning: Signed greater than
  /// * Flags: Z == 0 && N == V
  GT = 12,
  /// * Meaning: Signed less than or equal
  /// * Flags: Z == 1 || N != V
  LE = 13,
  /// * Meaning: Always
  /// * Flags: -
  AL = 14,
  /// * Meaning: ARMv5+ extensions, invalid condition for ARMv4
  /// * Flags: -
  EXT = 15,
}
impl Cond {
  pub fn array() -> [Self; 16] {
    use Cond::*;
    [EQ, NE, CSHS, CCLO, MI, PL, VS, VC, HI, LS, GE, LT, GT, LE, AL, EXT]
  }

  pub fn from_mask(mask: u16) -> Self {
    if mask < 16 {
      unsafe { core::mem::transmute(mask) }
    } else {
      panic!("Invalid condition mask: {}", mask)
    }
  }

  pub fn to_mask(self) -> u16 {
    self as u16
  }

  pub fn from_mnemonic(m: &str) -> Self {
    match m {
      "eq" => Cond::EQ,
      "ne" => Cond::NE,
      "cs" | "hs" => Cond::CSHS,
      "cc" | "lo" => Cond::CCLO,
      "mi" => Cond::MI,
      "pl" => Cond::PL,
      "vs" => Cond::VS,
      "vc" => Cond::VC,
      "hi" => Cond::HI,
      "ls" => Cond::LS,
      "ge" => Cond::GE,
      "lt" => Cond::LT,
      "gt" => Cond::GT,
      "le" => Cond::LE,
      "al" => Cond::AL,
      _ => panic!("Invalid condition mnemonic: {}", m),
    }
  }

  pub fn to_mnemonic(self) -> &'static str {
    match self {
      Cond::EQ => "eq",
      Cond::NE => "ne",
      Cond::CSHS => "cs",
      Cond::CCLO => "cc",
      Cond::MI => "mi",
      Cond::PL => "pl",
      Cond::VS => "vs",
      Cond::VC => "vc",
      Cond::HI => "hi",
      Cond::LS => "ls",
      Cond::GE => "ge",
      Cond::LT => "lt",
      Cond::GT => "gt",
      Cond::LE => "le",
      Cond::AL => "al",
      Cond::EXT => "",
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThumbOp {
  /// if cond { PC += (offset << 1).sign_extend_i32() }
  ///
  /// Note that PC is this location +4, net jump: -256 to +254
  ConditionalBranch { cond: Cond, offset: i8 },
  /// Immediate has no meaning except to the interrupt handling code
  SoftwareInterrupt { immediate: u8 },
  /// An Undefined instruction that will specifically never be defined in future
  /// versions. The lower 8 bits are ignored.
  UndefinedInstruction,
  /// An Undefined instruction that might be defined in some future version.
  /// Many bit patterns all map to this pattern.
  UndefinedFutureUse,
  /// Some encodings lead here
  UNPREDICTABLE,
}
impl ThumbOp {
  pub fn decode(opcode: u16) -> ThumbOp {
    match opcode >> 12 {
      0b0000 => panic!(),
      0b0001 => panic!(),
      0b0010 => panic!(),
      0b0011 => panic!(),
      0b0100 => panic!(),
      0b0101 => panic!(),
      0b0110 => panic!(),
      0b0111 => panic!(),
      0b1000 => panic!(),
      0b1001 => panic!(),
      0b1010 => panic!(),
      0b1011 => match (opcode >> 8) & 0b1111 {
        // Miscellaneous instructions
        0b0000 => panic!("adjust stack pointer"),
        0b0001 => ThumbOp::UndefinedFutureUse,
        0b0010 => panic!("sign/zero extend (ARMv6)"),
        0b0011 => ThumbOp::UndefinedFutureUse,
        0b0100 => panic!("push/pop register list, L=0, R=0"),
        0b0101 => panic!("push/pop register list, L=0, R=1"),
        0b0110 => panic!("lots of stuff in here"),
        0b0111 => ThumbOp::UndefinedFutureUse,
        0b1000 => ThumbOp::UndefinedFutureUse,
        0b1001 => ThumbOp::UndefinedFutureUse,
        0b1010 => panic!("Reverse bytes (ARMv6)"),
        0b1011 => ThumbOp::UndefinedFutureUse,
        0b1100 => panic!("push/pop register list, L=1, R=0"),
        0b1101 => panic!("push/pop register list, L=1, R=1"),
        0b1110 => panic!("Software breakpoint (ARMv5)"),
        0b1111 => panic!("UNPREDICTABLE?"),
        _ => unreachable!(),
      },
      0b1100 => panic!(),
      0b1101 => match Cond::from_mask((opcode >> 8) & 0b1111) {
        // Conditional Branch / SWI
        Cond::AL => ThumbOp::UndefinedInstruction,
        Cond::EXT => ThumbOp::SoftwareInterrupt { immediate: opcode as u8 },
        cond => ThumbOp::ConditionalBranch {
          cond,
          offset: (opcode & 0b1111_1111) as u8 as i8,
        },
      },
      0b1110 => panic!(),
      0b1111 => panic!(),
      _ => unreachable!(),
    }
  }

  pub fn encode(self) -> u16 {
    match self {
      ThumbOp::UndefinedFutureUse => 0b1011_0001_0000_0000,
      ThumbOp::UndefinedInstruction => 0b1101_1110_0000_0000,
      ThumbOp::SoftwareInterrupt { immediate } => (0b1101_1111 << 8) | (immediate as u16),
      ThumbOp::ConditionalBranch { cond, offset } => (0b1101 << 12) | cond.to_mask() << 8 | offset as u8 as u16,
      _ => panic!("Can't yet encode {:?}", self),
    }
  }
}
