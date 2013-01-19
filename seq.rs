//use core::iter::{Buildable,build_sized,build};
use core::iter::{Buildable,build};
use core::iter::Buildable::build_sized;

pub trait SeqView<A>: Index<uint,A> {
  pure fn len(&self) -> uint;
}

impl<A> &[A]: SeqView<A> {
  pure fn len(&self) -> uint { vec::len(*self) }
}

// Trait for immutable ("value-like") finite sequence types
pub trait Seq<A>: SeqView<A> Buildable<A> {
  // TODO: default implementations for *everything*
  pure fn append(self, other: self) -> self;
  pure fn filter(self, fn&(&A) -> bool) -> self;

  // [`start'..`end')
  pure fn slice(self, start: uint, end: uint) -> self;
  // // This default method trips rustc bug #3563
  // pure fn slice(self, start: uint, end: uint) -> self {
  //   assert start <= end;
  //   assert end <= self.len();
  //   from_fn(end-start, |i| self[start+i])
  // }

  pure fn each(&self, f: fn&(&A) -> bool); //TODO: default impl
  pure fn eachi(&self, f: fn&(uint, &A) -> bool);
  // bug #3563 again
  // pure fn eachi(&self, f: fn&(uint, &A) -> bool) {
  //   let mut i = 0;
  //   for self.each |e| {
  //     if !f(i,e) { break }
  //     i += 1
  //   }
  // }

  // fuckin BS where are my goddam default methods
  pure fn is_empty(&self) -> bool;
  //pure fn is_empty(&self) -> bool { self.len() == 0 }
  //pure fn head(self) -> A { self[0] }
  //pure fn last(self) -> A { self[self.len()-1] }

  static pure fn from_fn(size: uint, f: fn&(uint) -> A) -> self;
  // // Another default method that doesn't work.
  // static pure fn from_fn(size: uint, f: fn&(uint) -> A) {
  //   do build_sized(size) |push| {
  //     let mut i = 0;
  //     while i < size { push(f(i)); i += 1u }
  //   }
  // }

  static pure fn concat_from(SeqView<self>) -> self;
  static pure fn concat(vs: &[self]) -> self;
  // // This default method trips a rustc bug.
  // static pure fn concat(vs: &[self]) -> self {
  //   concat_from(vs as SeqView::<self>);
  // }
}

impl<A:Copy> @[A]: Buildable<A> {
  #[always_inline]
  static pure fn build_sized(size: uint, builder: fn(push: pure fn(v: A)))
    -> @[A]
  {
    at_vec::build_sized(size, builder)
  }
}

impl<A:Copy> @[A]: Seq<A> {
  pure fn append(self, other: @[A]) -> @[A] { self + other }

  pure fn filter(self, pred: fn&(&A) -> bool) -> @[A] {
    do build |push| {
      for vec::each(self) |x| {
        if pred(x) { push(*x) }
      }
    }
  }

  pure fn slice(self, start: uint, end: uint) -> @[A] {
    assert start <= end;
    assert end <= self.len();
    do build_sized(end-start) |push| {
      for uint::range(start, end) |i| {
        push(self[i])
      }
    }
  }

  pure fn each(&self, f: fn&(&A) -> bool) { vec::each(*self, f) }
  pure fn eachi(&self, f: fn&(uint, &A) -> bool) { vec::eachi(*self, f) }

  #[always_inline]
  pure fn is_empty(&self) -> bool { self.len() == 0 }

  static pure fn from_fn(n_elts: uint, f: fn&(uint) -> A) -> @[A] {
    at_vec::from_fn(n_elts, f)
  }

  static pure fn concat_from(_vs: SeqView<@[A]>) -> @[A] {
    fail
  }

  static pure fn concat(_vs: &[@[A]]) -> @[A] {
    fail
  }
}
