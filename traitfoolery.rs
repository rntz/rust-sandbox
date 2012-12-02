use option::Option;

// A type constructor T is a Functor iff it has an impl like so:
//
//   impl<A,B> T<A>: Functor<A,B,T<B>> { ... }
trait Functor<A, B, TB> {
  fn map(&self, fn&(&A) -> B) -> TB;
}

// An example.
impl<A,B> ~[A]: Functor<A, B, ~[B]> {
  fn map(&self, f: fn&(&A) -> B) -> ~[B] {
    vec::map(*self, f)
  }
}

// Another example
impl<A,B> Option<A>: Functor<A, B, Option<B>> {
  fn map(&self, f: fn&(&A) -> B) -> Option<B> {
    option::map(self, f)
  }
}

fn main() {}
