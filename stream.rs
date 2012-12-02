use option::Option;
use either::{Either,Left,Right};

extern mod std;

// this should probably be "pure fn@() -> Stream_<T>", but until it's easy to
// make pure lambdas, this would be *extremely* obnoxious.
type Stream<T> = fn@() -> Stream_<T>;
enum Stream_<T> {
  Empty,
  Cons(T, Stream<T>)
}

pure fn cons<T:Copy Owned>(x: T, xs: Stream<T>) -> Stream<T> {
  fn@() -> Stream_<T> { Cons(x,xs) }
}

pure fn tail<T>(s: Stream<T>) -> Stream<T> {
  || match s() {
    Empty => fail ~"tried to take tail of empty stream",
    Cons(_, rest) => rest()
  }
}

// do we really need the Owned bound?
fn vec_to_stream<T:Copy Owned>(f: @[T]) -> Stream<T> {
  generator_to_stream(vec_to_generator(f))
}

fn fix<T:Copy Owned>(f: fn@(s: Stream<T>) -> Stream<T>) -> Stream<T> {
  let g = @mut || fail;
  *g = memoize(|| { f(*g)() });
  *g
}

fn each<T>(s_: Stream<T>, f: fn&(&T) -> bool) {
  let mut s = s_;
  loop {
    match s() {
      Empty => return,
      Cons(x,xs) => {
        if !f(&x) { return }
        s = xs;
      }
    }
  }
}

fn eachi<T>(s: Stream<T>, f: fn&(uint, &T) -> bool) {
  let mut i = 0;
  for each(s) |x| {
    if !f(i,x) { return; }
    i += 1;
  }
}

pure fn filter<T>(s: Stream<T>, pred: fn@(&T) -> bool) -> Stream<T> {
  || match move s() {
    Empty => Empty,
    Cons(move x, move xs) =>
      if pred(&x) { Cons(move x, filter(xs, pred)) }
      else { filter(xs, pred)() }
  }
}

pure fn map<T,U>(s: Stream<T>, f: fn@(&T) -> U) -> Stream<U> {
  || match s() {
    Empty => Empty,
    Cons(x, xs) => Cons(f(&x), map(xs,f)),
  }
}

fn map_consume<T:Copy,U>(s: Stream<T>, f: fn@(T) -> U) -> Stream<U> {
  map(s, |x| f(*x))
}

pure fn map2<T,U,V>(xs: Stream<T>, ys: Stream<U>, f: fn@(&T, &U) -> V) -> Stream<V> {
  || match (xs(), ys()) {
    (Empty, _) | (_, Empty) => Empty,
    (Cons(x,xs), Cons(y,ys)) => Cons(f(&x,&y), map2(xs, ys, f)),
  }
}

fn map2_consume<T:Copy,U:Copy,V>
  (xs: Stream<T>, ys: Stream<U>, f: fn@(T, U) -> V) -> Stream<V>
{
  map2(xs, ys, |x,y| f(*x, *y))
}

pure fn unfold<T:Copy Owned, U:Copy Owned>
  (seed: T, gen: fn@(T) -> Option<(T,U)>) -> Stream<U>
{
  || match gen(seed) {
    None => Empty,
    Some((next,elt)) => Cons(elt, unfold(next, gen)),
  }
}

pure fn unfold_memoized<T:Owned,U:Copy Owned>
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

// should be pure but can't
fn memoize<T:Copy Owned>(s: Stream<T>) -> Stream<T> {
  do unfold_memoized(s) |str| {
    match str() {
      Empty => None,
      Cons(x,xs) => Some((xs,x))
    }
  }
}


// ---------- Infinite vectors ----------
type Infvec<T> = fn@(uint) -> T;

pure fn infvec_to_generator<T>(f: Infvec<T>) -> Generator<T> {
  let i = @mut 0;
  || { let x = f(*i); *i += 1; Some(move x) }
}

pure fn infvec_to_stream<T>(f: Infvec<T>) -> Stream<T> {
  pure fn iv2s_from<T>(f: Infvec<T>, from: uint) -> Stream<T> {
    || Cons(f(from), iv2s_from(f, from+1))
  }
  iv2s_from(f, 0)
}


// ---------- Stateful generators ----------
use pipes::{Port, Chan}; //TODO?: custom proto

type Generator<T> = fn@() -> Option<T>;

fn vec_to_generator<T:Copy Owned>(v: @[T]) -> Generator<T> {
  let i = @mut 0;
  || {
    if *i >= v.len() { None }
    else {
      let x = v[*i];
      *i += 1;
      Some(x)
    }
  }
}

fn generator_to_stream<T:Copy Owned>(g: Generator<T>) -> Stream<T> {
  unfold_memoized(g, |g| option::map_consume(g(), |x| (g,x)))
}

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

fn spawn_generator<T:Send>(thunk: fn~(fn&(x: T))) -> Generator<T> {
  let (chan, port) = pipes::stream();
  do task::spawn |move chan, move thunk| {
    do thunk |x| { chan.send(Some(move x)) }
    chan.send(None);
  }
  let done = @mut false;
  fn@(move port) -> Option<T> {
    if *done { return None; }
    match port.recv() {
      Some(move x) => Some(move x),
      None => { *done = true; None }
    }
  }
}

// proto! finite {
//   open: send<T:Send> {
//     send(T) -> open<T>,
//     close -> !
//   }
// }

// fn spawn_generator2<T:Send>(thunk: fn~(fn&(x: T))) -> Generator<T> {
//   use std::cell;
//   use finite::client;
//   use finite::server;

//   let (chan_, port_) = finite::init();

//   // Client
//   let chan = cell::Cell(move chan_);
//   do task::spawn |move chan, move thunk| {
//     do thunk |x| {
//       chan.put_back(client::send(chan.take(), Some(move x)))
//     }
//     client::close(chan.take());
//   }

//   // Server
//   let port = cell::Cell(Some(move port_));
//   fn@(move port) -> Option<T> {
//     match port.take() {
//       None => { port.put_back(None); None },
//       Some(_p) => {
//         fail
//         // select! {
//         //   p => {
//         //     send(msg) -> p_new {
//         //       port.put_back(Some(p_new));
//         //       Some(move msg)
//         //     },
//         //     close -> _x {
//         //       port.put_back(None);
//         //       None
//         //     }
//         //   }
//         // }
//       }
//     }
//   }
// }

fn main() {}
