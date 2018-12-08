use crate::{
  emulator::{atom, mfa::MFArgs, scheduler::Prio, vm::VM},
  term::lterm::*,
};


use std::{thread, time};


/// Entry point for the command-line interface
#[inline]
pub fn entrypoint() {
  if cfg!(feature = "r19") {
    println!("Erlang Runtime (compat OTP 19)");
    panic!("Support for R19 is unfinished and probably never will be finished.")
  }
  if cfg!(feature = "r20") {
    println!("Erlang Runtime (compat OTP 20)");
  }

  let mut beam = VM::new();

  let mfa = MFArgs::new(atom::from_str("test2"), atom::from_str("test"), Vec::new());
  let _rootp = beam
    .create_process(LTerm::nil(), &mfa, Prio::Normal)
    .unwrap();

  println!("Process created. Entering main loop...");
  while beam.tick().unwrap() {
    thread::sleep(time::Duration::from_millis(0));
  }
}
