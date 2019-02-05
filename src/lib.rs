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
  /// * Meaning: ARMv5 and above unconditional extensions (invalid ARMv4!)
  /// * Flags: -
  EXT = 15,
}
impl Cond {
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

pub enum ThumbOp {
  Unknown(String),
}
