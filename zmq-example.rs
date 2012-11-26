extern mod zmq;

use result::{Result, get, unwrap};
use task::{spawn};
use to_str::{ToStr};
use to_bytes::{ToBytes};

use zmq::{Context, Socket, Error};

fn main() {
    let cx = unwrap(zmq::init(1));

    {
        let pubs = unwrap(cx.socket(zmq::PUB));
        unwrap(pubs.bind("inproc://foo"));

        let subs = unwrap(cx.socket(zmq::SUB));
        unwrap(subs.connect("inproc://foo"));
        // subscribe to everything
        subs.set_subscribe([]);

        // Spawn off a listener child.
        do spawn |move subs| {
            loop {
                let msg = match subs.recv_str(0) {
                    Ok(move msg) => move msg,
                    Err(zmq::ETERM) => break,
                    Err(err) => fail err.to_str(),
                };
                io::println(fmt!("got message: %s", msg));
            }
        };

        // Send some messages.
        unwrap(pubs.send_str("first", 0));
        unwrap(pubs.send_str("second", 0));
        unwrap(pubs.send_str("last", 0));
        unwrap(pubs.close());
    }

    cx.term();
}
