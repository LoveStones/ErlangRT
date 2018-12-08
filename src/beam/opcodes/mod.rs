//! Opcodes group of modules provides inline implementations of BEAM opcodes.
pub mod op_bif;
pub mod op_data;
pub mod op_execution;
pub mod op_fun;
pub mod op_list;
pub mod op_memory;
pub mod op_predicates;

pub use crate::beam::opcodes::{
  op_bif::*, op_data::*, op_execution::*, op_fun::*, op_list::*, op_memory::*,
  op_predicates::*,
};


use crate::{
  beam::gen_op,
  emulator::{code::opcode::RawOpcode, runtime_ctx::Context},
  defs::Word,
};


/// Run a check whether opcode is not too large (within the supported range).
// TODO: Maybe #[inline] but now let compiler decide
#[cfg(debug_assertions)]
pub fn assert_arity(op: RawOpcode, val: Word) {
  debug_assert!(op < gen_op::OPCODE_MAX, "Opcode is too large");
  debug_assert_eq!(
    gen_op::ARITY_MAP[op.get() as usize] as Word,
    val,
    "Opcode {}={} arity is expected to be {}",
    gen_op::opcode_name(op),
    op.get(),
    val
  );
}


#[cfg(not(debug_assertions))]
#[inline]
pub fn assert_arity(_op: RawOpcode, _val: Word) {}


/// Display an error about opcode not supported/not implemented.
pub fn unknown_opcode(op: RawOpcode, ctx: &Context) {
  println!("{}", ctx);
  panic!(
    "vm_dispatch: Opcode {:?} '{}' not implemented",
    op.get(),
    gen_op::opcode_name(op)
  )
}
