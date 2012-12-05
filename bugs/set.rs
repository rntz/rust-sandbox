use to_bytes::{IterBytes,Cb};
use hash::{Hash};

fn main() {
  let x: (uint,uint) = (2,3);
  io::println(x.hash_keyed(0,1).to_str());
}

// // from https://github.com/mozilla/rust/pull/4052
// impl<A: IterBytes, B: IterBytes> (A,B): IterBytes {
//   #[inline(always)]
//   pure fn iter_bytes(lsb0: bool, f: to_bytes::Cb) {
//     let (ref a, ref b) = self;
//     a.iter_bytes(lsb0, f);
//     b.iter_bytes(lsb0, f);
//     // match self {
//     //   (ref a, ref b) => {
//     //     a.iter_bytes(lsb0, f);
//     //     b.iter_bytes(lsb0, f);
//     //   }
//     // }
//   }
// }
