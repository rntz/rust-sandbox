extern mod rutil;
use rutil::stream;
use stream::{Stream,Stream_,Cons,Empty,
             cons,tail,map,map_consume,map2,map2_consume,fix,each,filter};

// Naturals
fn nats_from(n: uint) -> Stream<uint> {
  || Cons(n, nats_from(n+1))
}

// Fibonaccis
fn genfibs(fibs: Stream<uint>) -> Stream<uint> {
  cons(1, cons(1, map2_consume(fibs, tail(fibs), |x,y| x + y)))
}

fn print_fibs() {
  for each(fix(genfibs)) |x| {
    io::println(fmt!("%u", *x));
  }
}

// Sieve of Eratosthenes
fn eratosthenes(s: Stream<uint>) -> Stream<uint> {
  || match s() {
    Empty => Empty,
    Cons(p, xs) => {
      Cons(p, eratosthenes(filter(xs, |n| *n % p != 0)))
    }
  }
}

fn print_primes() {
  let odds = map_consume(nats_from(1), |x| 2*x + 1);
  let primes = cons(2, eratosthenes(odds));
  for each(primes) |p| {
    io::println(fmt!("%u", *p));
  }
}

fn main() {
  print_fibs();
  //print_primes();
}
