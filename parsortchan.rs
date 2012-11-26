use comm_util;
use conduit;

import comm::{port,chan,send,recv};
import option::option;

import comm_util::*;
import conduit::*;

type le<T> = fn~(T,T) -> bool;

fn sendall<T:send>(out: chan<option<T>>, in: port<option<T>>) {
    loop {
        alt (recv(in)) {
          none { send(out, none); ret; }
          some(v) { send(out, some(v)); }
        }
    }
}

fn merge<T:send>
    (le: le<T>, a: port<option<T>>, b: port<option<T>>, out: chan<option<T>>)
{
    let mut a = a, b = b;

    let mut curr <- alt (recv(a), recv(b)) {
      (none, none) { send(out, none); ret; }
      (some(v), none) { send(out, some(v)); ret sendall(out, a); }
      (none, some(v)) { send(out, some(v)); ret sendall(out, b); }
      (some(x), some(y)) {
        if le(x,y) { send(out, some(x)); a <-> b; y }
        else { send(out, some(y)); x }
      }
    };

    loop {
        // We received `curr' from b. We keep receiving from a and passing it
        // along until we receive something greater than `curr', whereupon we
        // switch.
        let mut next = alt (recv(a)) { some(v) {v} none {break;} };
        if !le(next, curr) {
            a <-> b;
            curr <-> next;
        }
        send(out, some(next));
    }

    // We've exhausted a. Send everything from b.
    ret sendall(out, b);
}

fn main(_args: [str]) {}
