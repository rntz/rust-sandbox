use core::iter::Buildable;

pub trait SeqView<A>: Index<uint,A> {
  pure fn len(&self) -> uint;
}

// Trait for immutable ("value-like") finite sequence types
pub trait Seq<A>: SeqView<A>, Buildable<A> {
  //pure fn map<U>(&self, fn&(A) -> U) -> self<U>;
  //pure fn append(&self, other: &self) -> self;
}

// pub trait Seq<A>: Index<uint, A> {
//   pure fn len(&self) -> uint;

//   pure fn append(&self, other: &self) -> self;

//   pure fn each(&self, fn&(&A) -> bool);

//   static pure fn build(fn&(pure fn&(v: A))) -> self;
//   static pure fn tabulate(size: uint, func: fn&(uint) -> A) -> self;
// }

// pub trait SeqUtil<A>: Seq<A> {
//   pure fn eachi(&self, fn&(uint, &A) -> bool);
//   pure fn is_empty(&self) -> bool;
//   pure fn head(&self) -> A;
//   pure fn last(&self) -> A;
// }

// pub impl<A,T:Seq<A>> T: SeqUtil<A> {
//   pure fn is_empty(&self) { self.len() == 0 }
//   pure fn head(&self) -> A { self[0] }
//   pure fn last(&self) -> A { self[self.len()-1] }
// }
