extern mod rutil;
use rutil::stream;
use stream::{Stream,cons,tail,map2,map2_consume,fix,each};

fn main() {
  let fibs = fix(fn@(fibs: Stream<uint>) -> Stream<uint> {
    cons(1, cons(1, map2_consume(fibs, tail(fibs), |x,y| x + y)))
  });
  for each(fibs) |x| {
    io::println(fmt!("%u", *x));
  }
}
