use option::Option;
use either::{Either,Left,Right};

extern mod std;

type Stream<T> = fn@() -> Stream_<T>;
enum Stream_<T> {
  Empty,
  Cons(T, Stream<T>)
}

fn cons<T:Copy Owned>(x: T, xs: Stream<T>) -> Stream<T> {
  fn@() -> Stream_<T> { Cons(x,xs) }
}

fn tail<T>(s: Stream<T>) -> Stream<T> {
  || match s() {
    Empty => fail ~"tried to take tail of empty stream",
    Cons(_, rest) => rest()
  }
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

fn map<T,U>(s: Stream<T>, f: fn@(&T) -> U) -> Stream<U> {
  || match s() {
    Empty => Empty,
    Cons(x, xs) => Cons(f(&x), map(xs,f)),
  }
}

fn map_consume<T:Copy,U>(s: Stream<T>, f: fn@(T) -> U) -> Stream<U> {
  do map(s) |x| { f(*x) }
}

fn map2<T,U,V>(xs: Stream<T>, ys: Stream<U>, f: fn@(&T, &U) -> V) -> Stream<V> {
  || match (xs(), ys()) {
    (Empty, _) | (_, Empty) => Empty,
    (Cons(x,xs), Cons(y,ys)) => Cons(f(&x,&y), map2(xs, ys, f)),
  }
}

fn map2_consume<T:Copy,U:Copy,V>
  (xs: Stream<T>, ys: Stream<U>, f: fn@(T, U) -> V) -> Stream<V>
{
  do map2(xs,ys) |x,y| { f(*x, *y) }
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

// Stateful generators
use pipes::{Port, Chan}; //TODO?: custom proto

type Generator<T> = fn@() -> Option<T>;

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
