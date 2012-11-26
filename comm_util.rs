#[crate_type = "lib"];
#[link(name="comm_util", vers="0.0", author="rntz")];

import comm::*;
import spawn_sink = task::spawn_listener;

export portchan, spawn_src, spawn_sink, spawn_bidir;

fn portchan<T:send>() -> (port<T>, chan<T>) {
    let po = port();
    (po, chan(po))
}

fn spawn_src<T:send>(f: fn~(chan<T>)) -> port<T> {
    let (po,ch) = portchan();
    task::spawn({|| f(ch)});
    ret po
}

fn spawn_bidir<T:send, U:send>(f: fn~(chan<T>, port<U>)) -> (port<T>, chan<U>) {
    let (from_child, to_parent) = portchan();
    let to_child = spawn_sink({|from_parent| f(to_parent, from_parent) });
    ret (from_child, to_child);
}

// Spawns a communicating pair of child tasks.
fn spawn_pair<T:send>(src: fn~(chan<T>), sink: fn~(port<T>)) {
    let to_sink = spawn_sink(sink);
    task::spawn() {|| src(to_sink) };
}
