//! Generated by `codegen/create_vm_dispatch.py`
//! Dispatch for all opcode types.
//! Config used: OTP20
#![allow(dead_code)]

use beam::gen_op;
use beam::opcodes::*;
use rt_defs::{DispatchResult};
use emulator::code::opcode::RawOpcode;
use emulator::process::Process;
use emulator::runtime_ctx::Context;


#[inline]
pub fn dispatch_op_inline(op: RawOpcode, ctx: &mut Context, curr_p: &mut Process) -> DispatchResult {
  match op {

    gen_op::OPCODE_FUNC_INFO => return opcode_func_info(ctx, curr_p),
    gen_op::OPCODE_CALL => return opcode_call(ctx, curr_p),
    gen_op::OPCODE_CALL_ONLY => return opcode_call_only(ctx, curr_p),
    gen_op::OPCODE_BIF0 => return opcode_bif0(ctx, curr_p),
    gen_op::OPCODE_BIF1 => return opcode_bif1(ctx, curr_p),
    gen_op::OPCODE_BIF2 => return opcode_bif2(ctx, curr_p),
    gen_op::OPCODE_ALLOCATE => return opcode_allocate(ctx, curr_p),
    gen_op::OPCODE_ALLOCATE_ZERO => return opcode_allocate_zero(ctx, curr_p),
    gen_op::OPCODE_TEST_HEAP => return opcode_test_heap(ctx, curr_p),
    gen_op::OPCODE_DEALLOCATE => return opcode_deallocate(ctx, curr_p),
    gen_op::OPCODE_RETURN => return opcode_return(ctx, curr_p),
    gen_op::OPCODE_IS_LT => return opcode_is_lt(ctx, curr_p),
    gen_op::OPCODE_IS_EQ_EXACT => return opcode_is_eq_exact(ctx, curr_p),
    gen_op::OPCODE_IS_NIL => return opcode_is_nil(ctx, curr_p),
    gen_op::OPCODE_IS_NONEMPTY_LIST => return opcode_is_nonempty_list(ctx, curr_p),
    gen_op::OPCODE_MOVE => return opcode_move(ctx, curr_p),
    gen_op::OPCODE_GET_LIST => return opcode_get_list(ctx, curr_p),
    gen_op::OPCODE_PUT_LIST => return opcode_put_list(ctx, curr_p),
    gen_op::OPCODE_CALL_EXT_ONLY => return opcode_call_ext_only(ctx, curr_p),
    gen_op::OPCODE_GC_BIF1 => return opcode_gc_bif1(ctx, curr_p),
    gen_op::OPCODE_GC_BIF2 => return opcode_gc_bif2(ctx, curr_p),
    gen_op::OPCODE_GC_BIF3 => return opcode_gc_bif3(ctx, curr_p),
    other => unknown_opcode(other, ctx),
  }
  DispatchResult::Yield
}

