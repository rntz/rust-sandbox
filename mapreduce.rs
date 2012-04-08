#[crate_type = "lib"];
#[link(name="mapreduce", vers="0.0", author="rntz")];

use comm_util;
use future;

export tree, mapreduce, reduce;
export vec_tree, vec_chunks, mapreduce_vec, mapreduce_chunks;

enum tree<T> {
    leaf(T),
    node(~tree<T>, ~tree<T>)
}

// NB. A lot of copying of Ts and Us happens.
fn mapreduce<T:send,U:send>
    (t: ~tree<T>,
     mapf: fn~(T) -> U,
     joinf: fn~(U, U) -> U)
    -> U
{
    alt(*t) {
      leaf(x) { mapf(x) }
      node(l,r) {
        let fut = future::fork() {|| mapreduce(l, mapf, joinf) };
        let vr = mapreduce(r, mapf, joinf);
        let vl = future::join(fut);
        joinf(vl, vr)
      }
    }
}

fn reduce<T:send>(t: ~tree<T>, joinf: fn~(T,T) -> T) -> T {
    mapreduce(t, {|x| x}, joinf)
}

// Converting a vector into a tree.
fn vec_tree<T:copy>(v: [T]) -> ~tree<T> {
    fn folder<T:copy>(v: [T], start: uint, end: uint) -> ~tree<T> {
        assert start < end;
        if end - start == 1u { ~leaf(v[start]) }
        else {
            let mid = end + (end-start)/2u;
            ~node(folder(v, start, mid),
                  folder(v, mid, end))
        }
    }
    folder(v, 0u, vec::len(v))
}

fn mapreduce_vec<T:send,U:send>
    (input: [T], mapf: fn~(T) -> U, joinf: fn~(U, U) -> U) -> U
{
    mapreduce(vec_tree(input), mapf, joinf)
}

fn vec_chunks<T:copy>(v: [T], parts: uint) -> ~tree<~[T]> {
    import vec::*;
    assert parts > 0u;

    fn folder<T:copy>(v: [T], parts: uint, start: uint, end: uint)
        -> ~tree<~[T]>
    {
        assert parts > 0u;
        if parts == 1u { ~leaf(~slice(v, start, end)) }
        else {
            let half = parts/2u;
            let mid = start + (end-start)/2u;
            ~node(folder(v, half, start, mid),
                  folder(v, parts-half, mid, end))
        }
    }
    folder(v, parts, 0u, len(v))
}

fn mapreduce_chunks<T:send, U:send>
    (v: [T], parts: uint, mapf: fn~(~[T]) -> U, joinf: fn~(U,U) -> U) -> U
{
    mapreduce::<~[T],U>(vec_chunks(v, parts),
                        // need to wrap due to argument-passing style mismatch
                        {|x| mapf(x)}, {|x,y| joinf(x,y)})
}
