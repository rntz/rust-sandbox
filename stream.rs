use option::Option;
use either::{Either,Left,Right};

extern mod std;
use std::cell::{Cell,empty_cell};

type Stream<T> = fn@() -> Stream_<T>;
enum Stream_<T> {
  Empty,
  Cons(T, Stream<T>)
}

fn cons<T:Copy Owned>(x: T, xs: Stream<T>) -> Stream<T> {
  fn@() -> Stream_<T> { Cons(x,xs) }
}

fn unfold<T:Copy Owned, U:Copy Owned>
  (seed: T, gen: fn@(T) -> Option<(T,U)>) -> Stream<U>
{
  || match gen(seed) {
    None => Empty,
    Some((next,elt)) => Cons(elt, unfold(next, gen)),
  }
}

fn unfold_memoized<T:Owned,U:Copy Owned>
  (seed: T, gen: fn@(T) -> Option<(T,U)>) -> Stream<U>
{
  let cell: @mut Either<T, Stream_<U>> = @mut Left(move seed);
  || {
    // hack.
    let mut x = Right(Empty);
    x <-> *cell;
    match move x {
      Right(r) => { *cell = Right(r); r }
      Left(move seed) => {
        let res = match gen(move seed) {
          None => Empty,
          Some((move next, move elt)) =>
            Cons(elt, unfold_memoized(move next, gen)),
        };
        *cell = Right(res);
        res
      }
    }
  }
}

fn memoize<T:Copy Owned>(s: Stream<T>) -> Stream<T> {
  do unfold_memoized(s) |str| {
    match str() {
      Empty => None,
      Cons(x,xs) => Some((xs,x))
    }
  }
}

fn map<T,U>(s: Stream<T>, f: fn@(&T) -> U) -> Stream<U> {
  || match s() {
    Empty => Empty,
    Cons(x, xs) => Cons(f(&x), map(xs,f)),
  }
}

fn map_consume<T:Copy,U>(s: Stream<T>, f: fn@(T) -> U) -> Stream<U> {
  || match s() {
    Empty => Empty,
    Cons(x,xs) => Cons(f(x), map_consume(xs, f)),
  }
}

// Stateful generators
type Generator<T> = fn@() -> Option<T>;

fn stream_to_generator<T>(s: Stream<T>) -> Generator<T> {
  let cell = @mut s;
  || {
    let mut str = fn@() -> Stream_<T> { fail };
    str <-> *cell;
    match move str() {
      Empty => None,
      Cons(move x, move xs) => { *cell = xs; Some(move x) }
    }
  }
}

fn generator_to_stream<T:Copy Owned>(g: Generator<T>) -> Stream<T> {
  unfold_memoized(g, |g| option::map_consume(g(), |x| (g,x)))
}

fn main() {}
