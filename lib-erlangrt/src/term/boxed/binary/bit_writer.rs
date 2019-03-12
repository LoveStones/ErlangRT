//! Generic binary-write bit-aware functions

use crate::{
  beam::opcodes::binary::BsFlags,
  defs::{self, BitSize},
  fail::RtResult,
  term::lterm::Term,
};

pub struct BitWriter {}

impl BitWriter {
  /// For a writable byte buffer, insert an integer of given size. Different cases
  /// are handled for offsets multiple of 8, and for small/big integers.
  pub fn put_integer(
    write_val: Term,
    write_size: BitSize,
    dst: &mut [u8],
    dst_offset: BitSize,
    flags: crate::beam::opcodes::BsFlags,
  ) -> RtResult<()> {
    if write_size.is_empty() {
      // Nothing to do
      return Ok(());
    }

    if write_val.is_small() {
      let inbyte_offset = dst_offset.get_last_byte_bits();
      let rbits = defs::BYTE_BITS - inbyte_offset;

      if inbyte_offset + write_size.bit_count < defs::BYTE_BITS {
        // All bits will land into the same byte
        unsafe {
          let iptr = dst.as_mut_ptr().add(dst_offset.get_bytes_rounded_down());
          return Self::put_bits_one_byte(
            iptr,
            rbits,
            inbyte_offset,
            write_val.get_small_signed(),
            write_size,
          );
        }
      } else if inbyte_offset == 0 {
        // More than one bit, starting at a byte boundary.
        unsafe {
          let dst_ptr = dst.as_mut_ptr().add(dst_offset.get_bytes_rounded_down());
          return Self::fmt_int(write_val, write_size, dst_ptr, flags);
        }
      } else if flags.contains(BsFlags::LITTLE) {
        unsafe {
          return Self::put_bits_unaligned();
        }
      } else {
        unsafe {
          let dst_ptr = dst.as_mut_ptr().add(dst_offset.get_bytes_rounded_down());
          return Self::put_bits_big_endian(
            write_val, write_size, rbits, dst_ptr, dst_offset, flags,
          );
        }
      }
    } else if dst_offset.is_empty() {
      // Big number, aligned on a byte boundary. We can format the
      // integer directly into the binary.
      unsafe {
        let dst_ptr = dst.as_mut_ptr().add(dst_offset.get_bytes_rounded_down());
        return Self::fmt_int(write_val, write_size, dst_ptr, flags);
      }
    } else {
      // unaligned
      unimplemented!("put_integer: unaligned")
    }

    // Ok(())
  }

  unsafe fn put_bits_big_endian(
    write_val: Term,
    write_size: BitSize,
    rbits: usize,
    iptr: *mut u8,
    dst_offset: BitSize,
    flags: crate::beam::opcodes::BsFlags,
  ) -> RtResult<()> {
    // Big-endian, more than one byte, but not aligned on a byte boundary.
    // Handle the bits up to the next byte boundary specially,
    // then let fmt_int() handle the rest.
    let shift_count = write_size.bit_count - rbits;
    let val = write_val.get_small_signed();
    let mut b = core::ptr::read(iptr) & (0xff << rbits);

    // Shifting with a shift count greater than or equal to the word
    // size may be a no-op (instead of 0 the result may be the unshifted
    // value). Therefore, only do the shift and the OR if the shift count
    // is less than the word size if the number is positive; if negative,
    // we must simulate the sign extension.

    if shift_count < defs::WORD_BITS {
      let add_bits = (val >> shift_count) & ((1 << rbits) - 1);
      b |= add_bits as u8;
    } else if val < 0 {
      // Simulate sign extension.
      b |= (!0) & ((1 << rbits) - 1);
    }
    core::ptr::write(iptr, b);

    // fmt_int() can't fail here. Continue to the next byte
    Self::fmt_int(
      write_val,
      write_size - BitSize::with_bits(rbits),
      iptr.add(1),
      flags,
    )
  }

  /// Destination span is entirely inside one byte
  #[inline]
  unsafe fn put_bits_one_byte(
    iptr: *mut u8,
    rbits: usize,
    inbyte_offset: usize,
    write_val: isize,
    write_size: BitSize,
  ) -> RtResult<()> {
    // Read the old value and mask away the bits about to be replaced
    let mut b = core::ptr::read(iptr) & (0xff << rbits);
    let val_mask = (1 << write_size.bit_count) - 1;
    let new_val = (write_val & val_mask) << (8 - inbyte_offset - write_size.bit_count);
    b |= new_val as u8;
    core::ptr::write(iptr, b);
    Ok(())
  }

  unsafe fn put_bits_unaligned() -> RtResult<()> {
    unimplemented!("put_bits_unaligned");
    Ok(())
  }

  /// Writes bits of an integer.
  /// Assumes destination has enough space in it.
  unsafe fn fmt_int(
    write_val: Term,
    write_size: BitSize,
    dst: *mut u8,
    flags: crate::beam::opcodes::BsFlags,
  ) -> RtResult<()> {
    unimplemented!("fmt_int");
    Ok(())
  }
}