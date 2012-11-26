#[crate_type = "lib"];
#[link(name="conduit", vers="0.0", author="rntz")];

use comm_util;

import option::option;
import comm::*;
import comm_util::*;

export msg, src, sink, conduit;
export is_msg, is_done;
export connect;
export fuse, fuse_src, fuse_sink;
export src_from, sink_to;
export cond_map, cond_filter, cond_map_partial;

// Design problem: Sources can either be:
//
// - fn~s, letting us execute them remotely.
// - fn@s, letting us construct them from ports and other task-local data.
//
// Obviously, both of these are desirable. I think letting us use task-local
// data is more important, so I've made sources fn@s.
//
// There might be some better or more flexible way to approach this problem.
// TODO: think about it.

enum msg<T, F> { msg(T), done(F) }

pure fn is_msg<T,F>(m : msg<T,F>) -> bool
    { alt (m) { msg(_) {true} _ {false} } }

pure fn is_done<T,F>(m : msg<T,F>) -> bool
    { alt (m) { msg(_) {false} done(_) {true} } }

type src_local<T:send, F:send> = fn@(chan<msg<T,F>>);
// type src_mobile<T:send> = fn~(chan<T>);

// Conduits.
type src<T:send, F:send> = src_local<T,F>;
type sink<T:send, F:send> = fn~(port<msg<T,F>>, chan<F>);
type conduit<T:send, U:send, F:send> =
    fn~(port<msg<T,F>>, chan<msg<U,F>>);


// Runs the pipe to completion. Blocking.
fn connect<T:send, F:send>
    (src: src<T,F>, sink: sink<T,F>) -> F
{
    let (fpo, fch) = portchan();
    let ch = spawn_sink {|po| sink(po, fch) };
    src(ch);
    ret recv(fpo)
}


// Fusing: conduits to conduits, srcs to conduits, conduits to sinks.
fn fuse<T:send, U:send, V:send, F:send>
   (fst: conduit<T,U,F>, snd: conduit<U,V,F>) -> conduit<T,V,F>
{
    fn~(from: port<msg<T,F>>, to: chan<msg<V,F>>) {
        let to_mid = spawn_sink {|from_mid| snd(from_mid, to); };
        fst(from, to_mid);
    }
}

fn fuse_src<T:send, U:send, F:send>
    (src: src<T,F>, pipe: conduit<T,U,F>) -> src<U,F>
{
    ret fn@(ch: chan<msg<U,F>>) {
        let chnew = spawn_sink {|po| pipe(po, ch) };
        src(chnew);
    }
}

fn fuse_sink<T:send, U:send, F:send>
    (pipe: conduit<T,U,F>, sink: sink<U,F>) -> sink<T,F>
{
    ret fn~(po: port<msg<T,F>>, donech: chan<F>) {
        let ch = spawn_sink {|po| sink(po, donech) };
        pipe(po, ch);
    }
}


// Constructing sinks, srcs, conduits.
fn src_from<T:send, F:send>(p : port<msg<T,F>>) -> src<T, F>
{
    {|ch| loop {
            let m = recv(p);
            let d = is_done(m);
            send(ch, m);
            if d { break }
    }}
}

fn sink_to<T:send, F:send>(ch: chan<T>) -> sink<T,F>
{
    fn~ (po: port<msg<T,F>>, donech: chan<F>) {
        loop {
            alt (recv(po)) {
              // I have to copy here to avoid moving out of a binding, even
              // though I know I have the only reference to m, f.
              // TODO: use option::unwrap for this.
              msg(m) { send(ch, copy m); }
              done(f) { send(donech, copy f); break; }
            }
        }
    }
}

fn cond_map<T:send, U:send, F:send>(f: fn~(T) -> U) -> conduit<T,U,F>
{
    fn~ (from: port<msg<T,F>>, to: chan<msg<U,F>>) {
        loop {
            alt (recv(from)) {
              msg(m) { send(to, msg(f(m))); }
              done(f) { send(to, done(f)); break; }
            }
        }
    }
}

fn cond_filter<T:send, F:send>(pred: fn~(T)->bool) -> conduit<T,T,F>
{
    fn~ (from: port<msg<T,F>>, to: chan<msg<T,F>>) {
        loop {
            let m = recv(from);
            alt (m) {
              msg(v) { if pred(v) { send(to, m); } }
              done(_) { send(to, m); break; }
            };
        }
    }
}

fn cond_map_partial<T:send, U:send, F:send>
    (f: fn~(T) -> option<U>) -> conduit<T,U,F>
{
    ret {|from, to| loop {
        alt (recv(from)) {
          msg(m) {
            alt (f(m)) {
              some(v) { send(to, msg(v)); }
              none {}
            }
          }
          done(f) { send(to, done(f)); break; }
        }
    }}
}
