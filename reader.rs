use ops::{Add,Drop,Sub,Mul,Div,Modulo,Neg,BitAnd};

// the Reader monad
enum Reader<T,U> = fn@(T) -> U;

pure fn map_add<t:Copy, u2, u3, u1: Add<u2, u3>>
  (f: fn@(t) -> u1, g: fn@(t) -> u2) -> fn@(t) -> u3
{
  |x| f(x) + g(x)
}

impl<T:Copy, U2, U3, U1: Add<U2, U3>>
    fn@(T) -> U1: Add<fn@(T) -> U2, fn@(T) -> U3>
{
  pure fn add(rhs: &@fn(T) -> U2) -> fn@(T) -> U3 {
    let g = *rhs;
    |x| self(x) + g(x)
  }
}

impl<T:Copy, U2, U3, U1:Add<U2, U3>>
    Reader<T,U1>: Add<Reader<T,U2>, Reader<T,U3>>
{
  pure fn add(rhs: &Reader<T,U2>) -> Reader<T,U3> {
    // this fails because it can't be sure map_add doesn't call self or rhs,
    // which are impure fns.
    //map_add(*self, **rhs)

    // this fails for the same reason, except for Reader instead of map_add,
    // even though Reader is a constructor.
    //let g = *rhs;
    //Reader(fn@(x: T) -> U3 { (*self)(x) + (*g)(x) })

    let _ = rhs; fail;
  }
}

fn main() {}
