use std::fmt::{self, Debug, Formatter};

use wrapping::{w, cvt, w8, w16};

pub struct Flags(pub u8);
impl Flags {
  pub fn new() -> Self { Flags(0) }

  pub fn s(&self) -> bool  { self.0 & 0b1000_0000 != 0 }
  pub fn z(&self) -> bool  { self.0 & 0b0100_0000 != 0 }
  pub fn f5(&self) -> bool { self.0 & 0b0010_0000 != 0 }
  #[allow(dead_code)]
  pub fn h(&self) -> bool  { //self.0 & 0b0001_0000 != 0
    unimplemented!()
  }
  pub fn f3(&self) -> bool { self.0 & 0b0000_1000 != 0 }
  pub fn v(&self) -> bool { self.0 & 0b0000_0100 != 0 }
  pub fn n(&self) -> bool  { self.0 & 0b0000_0010 != 0 }
  pub fn c(&self) -> bool  { self.0 & 0b0000_0001 != 0 }

  #[inline(always)]
  fn set_bit(&mut self, n: usize, to: bool) {
    self.0 |= (to as u8) << n;
    self.0 &= !((!to as u8) << n);
  }

  pub fn set_s(&mut self, to: bool)  { self.set_bit(7, to) }
  pub fn set_z(&mut self, to: bool)  { self.set_bit(6, to) }
  pub fn set_5(&mut self, to: bool) { self.set_bit(5, to) }
  #[allow(dead_code)]
  pub fn set_h(&mut self, _to: bool)  { //self.set_bit(4, to)
    unimplemented!()
  }
  pub fn set_3(&mut self, to: bool) { self.set_bit(3, to) }
  pub fn set_v(&mut self, to: bool) { self.set_bit(2, to) }
  pub fn set_n(&mut self, to: bool)  { self.set_bit(1, to) }
  pub fn set_c(&mut self, to: bool)  { self.set_bit(0, to) }
}
impl Debug for Flags {
  fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
    f.debug_struct("ConditionCodes")
      .field("s", &self.s())
      .field("z", &self.z())
      .field("5", &self.f5())
      .field("3", &self.f3())
      .field("v", &self.v())
      .field("n", &self.n())
      .field("c", &self.c())
      .finish()
  }
}

#[derive(Debug)]
pub struct Regs {
  pub a: w8,
  pub flags: Flags,
  pub b: w8,
  pub c: w8,
  pub d: w8,
  pub e: w8,
  pub h: w8,
  pub l: w8,
  pub w: w8,
  pub z: w8,
}

impl Regs {
  pub fn new() -> Self {
    Regs {
      a: w(0),
      b: w(0),
      c: w(0),
      d: w(0),
      e: w(0),
      h: w(0),
      l: w(0),
      w: w(0),
      z: w(0),
      flags: Flags::new(),
    }
  }

  pub fn hl(&self) -> w16 {
    (cvt(self.h) as w16) << 8 | (cvt(self.l) as w16)
  }
}

