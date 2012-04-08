#[crate_type = "lib"];
#[link(name="parsort", vers="0.0", author="rntz")];

use comm_util;
use future;
use iter;
use mapreduce;
use std;

import comm_util::*;

export parsort;

type le<T> = fn~(T,T) -> bool;

fn merge<T:copy>(le: le<T>, a: [T], b: [T]) -> [T] {
    let al = vec::len(a), bl = vec::len(b);
    let len = al + bl;
    let mut r = [], i = 0u, j = 0u;
    vec::reserve(r, len);

    while i < al && j < bl {
        let x = a[i], y = b[j];
        r += [if le(x,y) { i += 1u; x } else { j += 1u; y }]
    }
    if i < al { r += vec::slice(a, i, al); }
    if j < bl { r += vec::slice(b, j, bl); }

    ret r
}

// TODO: think about interaction when T is linear.
fn sortfn<T:copy>(le: le<T>, elts: ~[T]) -> ~[T] {
    ~std::sort::merge_sort(le, *elts)
}

fn joinfn<T:copy>(le: le<T>, a: ~[T], b: ~[T]) -> ~[T] {
    ~merge(le, *a, *b)
}

fn parsort<T:send>(le: le<T>, nthreads: uint, elts: [T]) -> [T] {
    *mapreduce::mapreduce_chunks(
        elts, nthreads,
        {|x| sortfn(le, x)},
        {|x,y| joinfn(le, x, y)})
}
