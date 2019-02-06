use arm_opcode::*;

#[test]
fn test_conditional_branch() {
  for offset in core::i8::MIN..=core::i8::MAX {
    for cond in Cond::array().iter().cloned() {
      if cond == Cond::AL || cond == Cond::EXT {
        continue;
      }
      let start = ThumbOp::ConditionalBranch { cond, offset };
      let decoded = ThumbOp::decode(start.encode());
      assert_eq!(start, decoded);
    }
  }
}

#[test]
fn test_undefined() {
  assert_eq!(ThumbOp::decode(ThumbOp::UndefinedInstruction.encode()), ThumbOp::UndefinedInstruction);
  // TODO: allow `decode` to generate `UndefinedFutureUse`
  //assert_eq!(ThumbOp::decode(ThumbOp::UndefinedFutureUse.encode()), ThumbOp::UndefinedFutureUse);
}
