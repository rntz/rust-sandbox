extern mod rutil;
use rutil::stream;
use stream::{Stream,cons,tail,map2,map2_consume};

fn genfibs(fibs: Stream<uint>) -> Stream<uint> {
  let rest = do map2_consume(fibs, tail(fibs)) |x,y| { x + y };
  cons(1, cons(1, rest))
}

fn main() {
  let fibs = stream::fix(genfibs);
  for stream::each(fibs) |x| {
    io::println(fmt!("%u", *x));
  }
}
