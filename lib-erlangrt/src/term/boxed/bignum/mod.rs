use core::{mem::size_of, ptr};

use crate::{defs::{ByteSize, WordSize}, defs, emulator::heap::Heap, fail::{RtErr, RtResult}, term::{
  boxed::{
    BoxHeader,
    boxtype::{self, BoxType},
    BOXTYPETAG_BIGINTEGER, trait_interface::TBoxed,
  },
  classify,
  value::*,
}};

use self::sign::*;

pub mod endianness;
pub mod sign;

pub const BIG_DIGIT_SIZE: usize = defs::WORD_BYTES;
pub type Digit = usize;

#[allow(dead_code)]
pub struct Bignum {
  header: BoxHeader,

  /// Negative size points that the number is negative.
  pub size: isize,
  /// First limb of digits is here, remaining digits follow in the memory after
  pub digits: Digit,
}

impl TBoxed for Bignum {
  fn get_class(&self) -> classify::TermClass {
    classify::CLASS_NUMBER
  }

  fn get_type(&self) -> BoxType {
    boxtype::BOXTYPETAG_BIGINTEGER
  }
}

impl Bignum {
  const fn storage_size() -> WordSize {
    // This impl stores bignum in dynamic heap with the num library
    ByteSize::new(size_of::<Bignum>()).get_words_rounded_up()
  }

  /// Create bignum for one isize
  pub fn with_isize(hp: &mut Heap, val: isize) -> RtResult<*mut Bignum> {
    let (sign, positive_val) = Sign::split(val);

    // Create slice of one limb
    let limbs = unsafe {
      let data = &positive_val as *const usize;
      core::slice::from_raw_parts(data, 1)
    };
    unsafe { Self::create_into(hp, sign, limbs) }
  }

  /// Consume bytes as either big- or little-endian stream, and build a big
  /// integer on heap.
  pub unsafe fn create_into(
    hp: &mut Heap,
    sign: Sign,
    limbs: &[Digit],
  ) -> RtResult<*mut Self> {
    let n_words = Self::storage_size();
    let this = hp.alloc::<Self>(n_words, false)?;

    ptr::write(
      this,
      Self {
        header: BoxHeader::new::<Self>(n_words),
        size: if sign == Sign::Negative {
          -(limbs.len() as isize)
        } else {
          limbs.len() as isize
        },
        digits: 0,
      },
    );
    ptr::copy_nonoverlapping(
      limbs.as_ptr(),
      &mut (*this).digits as *mut Digit,
      limbs.len(),
    );

    Ok(this)
  }

  pub fn get_digits(&self) -> &[Digit] {
    unsafe { core::slice::from_raw_parts(&self.digits as *const Digit, self.get_size()) }
  }

  pub fn is_negative(&self) -> bool {
    self.size < 0
  }

  /// Return how many digits are stored (abs value of self.size)
  #[inline]
  pub fn get_size(&self) -> usize {
    if self.size >= 0 {
      self.size
    } else {
      -self.size
    } as usize
  }

  /// Return how many bytes are used to store the digits. Multiple of word size.
  #[inline]
  pub fn get_byte_size(&self) -> ByteSize {
    ByteSize::new(self.get_size() * DIGIT_BYTES)
  }

  #[allow(dead_code)]
  pub unsafe fn const_from_term(t: Term) -> RtResult<*const Self> {
    helper_get_const_from_boxed_term::<Self>(
      t,
      BOXTYPETAG_BIGINTEGER,
      RtErr::BoxedIsNotABigint,
    )
  }

  #[allow(dead_code)]
  pub unsafe fn mut_from_term(t: Term) -> RtResult<*mut Self> {
    helper_get_mut_from_boxed_term::<Self>(
      t,
      BOXTYPETAG_BIGINTEGER,
      RtErr::BoxedIsNotABigint,
    )
  }
}