use option::Option;
use cmp::{Eq, Ord};

use order::{Order,LT,EQ,GT,compare};

struct Node<K,V,T> {
  key: K, value: V,
  left: T, right: T,
}

fn Node<K,V,T>(k: K, v: V, l: T, r: T) -> Node<K,V,T> {
  Node{key: k, value: v, left: l, right: r}
}

type Black<K,V> = Tree<K,V>;
type Red<K,V> = Node<K, V, Black<K,V>>;

// A black node.
enum Tree<K,V> {
  Tree(~Node<K, V, BlackChild<K,V>>),
  Empty,
}

// A child of a black node; could be either red or black.
enum BlackChild<K,V> {
  Black(Black<K,V>),
  Red(Red<K,V>),
}

impl<K:Eq Ord, V:Copy> Tree<K,V> {
  fn lookup(&self, key: &K) -> Option<V> {
    match *self {
      Empty => None,
      Tree(node) => match compare(key, &node.key) {
          EQ => Some(node.value),
          LT => node.left.lookup(key),
          GT => node.right.lookup(key),
      },
    }
  }
}

impl<K:Eq Ord, V:Copy> BlackChild<K,V> {
  fn lookup(&self, key: &K) -> Option<V> {
    match *self {
      Black(t) => t.lookup(key),
      Red(node) => match compare(key, &node.key) {
        EQ => Some(node.value),
        LT => node.left.lookup(key),
        GT => node.right.lookup(key),
      },
    }
  }
}

// ---------- INSERTION ----------
//
// Let the "rank" of a node be the number of black nodes on any path to a leaf.
// (It is an RB tree invariant that this number is the same for all paths.)
//
// - inserting into a black node can return a black or a red node, but the
// returned node has the same rank as the original
//
// - inserting into a red node returns either a red node of the same rank, or a
// black node of rank one higher with exactly one black and one red child.
//
// How insertion works follows from these invariants, plus the RB tree
// invariants, plus a bunch of tedious and painful scratchwork.

enum RedInsertResult<K,V> {
  RIRed(Red<K,V>),
  RIBlackLeft(K,V, Red<K,V>, Black<K,V>),
  RIBlackRight(K,V, Black<K,V>, Red<K,V>),
}

priv fn insert_red<K:Eq Ord,V>
  (t: Red<K,V>, key: K, value: V) -> RedInsertResult<K,V>
{
  let Node{key: k, value: v, left: l, right: r} = t;
  match compare(&key, &k) {
    EQ => RIRed(Node(k, value, l, r)),
    LT => match insert_black(l, key, value) {
      Black(t) => RIRed(Node(k, v, t, r)),
      Red(lnew) => RIBlackLeft(k, v, lnew, r),
    },
    GT => match insert_black(r, key, value) {
      Black(t) => RIRed(Node(k, v, l, t)),
      Red(rnew) => RIBlackRight(k, v, l, rnew),
    },
  }
}

priv fn insert_black<K:Eq Ord,V>
  (t: Tree<K,V>, key: K, value: V) -> BlackChild<K,V>
{
  match t {
    Empty => Black(Tree(~Node(key, value, Black(Empty), Black(Empty)))),

    Tree(~Node{key: k, value: v, left: l, right: r}) => {
      match compare(&key, &k) {
        EQ => Black(Tree(~Node(k, value, l, r))),

        LT => {
          match l {
            Black(lb) => Black(Tree(~Node(k,v, insert_black(lb,key,value), r))),
            Red(lr) => match insert_red(lr, key, value) {
              RIRed(lnew) => Black(Tree(~Node(k,v,Red(lnew),r))),
              _ => fail
            },
          }
        }

        GT => fail,
      }
    }
  }
}

fn insert<K:Eq Ord,V>(t: Tree<K,V>, key: K, value: V) -> Tree<K,V> {
  match insert_black(t, key, value) {
    Black(t) => t,
    Red(Node{key: k, value: v, left: l, right: r}) =>
      Tree(~Node(k, v, Black(l), Black(r))),
  }
}

impl<K:Eq Ord,V> Tree<K,V> {
  fn insert(&mut self, key: K, value: V) {
    let mut t = Empty;
    *self <-> t;
    *self = insert(t, key, value);
  }
}
