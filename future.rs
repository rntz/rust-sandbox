// Optimistically evaluated futures.
#[crate_type = "lib"];
#[link(name="future", vers="0.0", author="rntz")];

use comm_util;
import comm::*;
import comm_util::*;

export future, fork, join;

// Futures
enum future<T:send> { fut(port<T>) }

fn fork<T:send>(-f: fn~() -> T) -> future<T> {
    fut(spawn_src({|ch| send(ch, f()) }))
}

fn join<T:send>(-x: future<T>) -> T {
    recv(*x)
}
