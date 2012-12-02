extern mod rutil;
use rutil::stream;
use stream::{Stream,cons,tail,map2,map2_consume};

fn main() {
  let fibs = do stream::fix |fibs| {
    cons(1, cons(1, do map2_consume(fibs, tail(fibs), |x,y| x + y)))
  }
  for stream::each(fibs) |x| {
    io::println(fmt!("%u", *x));
  }
}
