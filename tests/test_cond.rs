use arm_opcode::*;

#[test]
fn test_cond_mask() {
  for i in 0 .. 16 {
    assert_eq!(Cond::from_mask(i).to_mask(), i);
  }
}

#[test]
#[should_panic]
fn test_cond_mask_panic() {
  Cond::from_mask(16);
}

#[test]
fn test_mnemonic() {
  // note: 1 less than full, EXT has no real mnemonic
  for i in 0 .. 15 {
    let c = Cond::from_mask(i);
    assert_eq!(Cond::from_mnemonic(c.to_mnemonic()), c);
  }
}
