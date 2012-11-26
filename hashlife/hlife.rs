// Core imports
import io::{print,println};
import option::{option};

// Std imports
use std;

import map = std::map;
import map::hashmap;

// Note: When representing a grid in a unsigned integer, states are stored
// top-to-bottom, left-to-right. When there are excess bits (ie. using a u8 to
// represent a 2x2 grid, the low bits are significant and the high bits
// ignored). so for example 0b_abcd_u8 (where a,b,c,d are bits) represents the
// 2x2 grid:
//
//     |a b|
//     |c d|

// When representing a grid by a list of quadrants or other things, they are
// listed from bottom-to-top, right-to-left. This may seem odd, but it in fact
// corresponds to the ordering of the bits if you view them little-endian rather
// than big-endian. So a cell with quadrants [a,b,c,d] looks like the following:
//
//     |d c|
//     |b a|

type cell = @mut cts;
type quads = (cell, cell, cell, cell);
enum cts {
    // quads, result
    node(quads, option<cell>),
    // 4x4 grid: bits, result
    four(u16, option<u8>)
}

enum result { cell(cell), two(u8) }

type env = @{
    // we have different cons-caches for each size of cell. this makes gc nicer.
    // the base-level cache.
    cache4x4 : hashmap<u16, cell>,
    // higher-level caches.
    caches : [hashmap<[cell], cell>]
};

pure fn cell_rank(c : cell) -> uint {
    alt c {
      @four(_,_) { 0u }
      @node(quads, result) {
        let (a,b,c,d) = quads;
        let rnk = cell_rank(a);
        check cell_has_rank(b, rnk);
        check cell_has_rank(c, rnk);
        check cell_has_rank(d, rnk);
        alt result {
          some(r) { check cell_has_rank(r, rnk); }
          none {}
        }
        ret 1u + rnk
      }
    }
}

pure fn cell_has_rank(c: cell, rank: uint) -> bool { cell_rank(c) == rank }

pure fn rank_to_size(rank: uint) -> uint { 4u << rank }
pure fn cell_size(c: cell) -> uint { rank_to_size(cell_rank(c)) }


// Macrocell ops
fn result(e : env, rank : uint, c : cell) -> result {
    check cell_has_rank(c, rank);
    alt c {
      // Return memorized results if present.
      @node(_, some(r)) { cell(r) }
      @four(_, some(r)) { two(r) }

      // Compute via life algo.
      @four(bits, none) {
        // TODO: hand-optimize this?
        let ll = next_state_hack(bits),
            lr = next_state_hack(bits >> 1u),
            ul = next_state_hack(bits >> 4u),
            ur = next_state_hack(bits >> 5u);
        let res = (ul << 3u) + (ur << 2u) + (lr << 1u) + ll;
        *c = four(bits, some(res));
        ret two(res)
      }

      // The interesting case. Uses the algorithm as described by Bill Gosper in
      // "Exploiting regularities in large cellular spaces", Physica 10D, 1984.
      @node(quads, none) {
        // Compute overlapping nonads from quads.
        let nonads = quads_to_nonads(e, quads);

        // Compute results for each nonad.
        let rs = results(e, rank-1u, nonads);

        // Combine results from nonads to form overlapping future quadrants.
        let fquads = [merge(e, [rs[0], rs[1], rs[3], rs[4]]),
                      merge(e, [rs[1], rs[2], rs[4], rs[5]]),
                      merge(e, [rs[3], rs[4], rs[6], rs[7]]),
                      merge(e, [rs[4], rs[5], rs[7], rs[8]])];

        let res = merge(e, results(e, rank-1u, fquads));
        *c = node(quads, some(res));
        ret cell(res)
      }
    }
}

fn results(e: env, rank: uint, cs: [cell]) -> [result] {
    vec::map(cs) {|c| result(e, rank, c)}
}

// Given se, sw, ne, nw quadrants, compute north, south, east, west, and center
// cells and combine these in proper order (see comment at beginning offile)
// into a vector of nonads.
fn quads_to_nonads(e : env, quads : quads) -> [cell] {
    // A vector of results splitting our quads into sixteenths.
    let rs = vec::map(*quads, split);
    ret [quads[0u], merge(e, [rs[0u][1u], rs[1u][0u],
                              rs[0u][3u], rs[1u][2u]]), quads[1u],
         merge(e, [rs[0u][2u], rs[0u][3u],
                   rs[2u][0u], rs[2u][1u]]),
         merge(e, [rs[0u][3u], rs[1u][2u],
                   rs[2u][1u], rs[3u][0u]]),
         merge(e, [rs[1u][2u], rs[1u][3u],
                   rs[3u][0u], rs[3u][1u]]),
         quads[2u], merge(e, [rs[2u][1u], rs[3u][0u],
                              rs[2u][3u], rs[3u][2u]]), quads[3u]];
}

// Merges four results into a cell. This is the only function that performs
// hash-consing.
fn merge(_e : env, _quads : [result]) -> cell {
    fail "unimplemented"
}

// Does the inverse of merge.
fn split(&&c : cell) -> [result] {
    alt c {
      @node(quads, _) { vec::map(*quads) {|x| cell(x)} }
      @four(bits, _) {
        const mask : u16 = 0b0111_0111_0111_u16;
        [        two((bits & mask) as u8), two(((bits >> 1u) & mask) as u8),
         two(((bits >> 4u) & mask) as u8), two(((bits >> 5u) & mask) as u8)]
      }
    }
}

// Some nice bit-ops.

// Determines the next state of the middle lower-right cell in a four-by-four
// grid, ie the cell marked "x" below:
//
//     |_ _ _ _|
//     |_ _ _ _|
//     |_ _ x _|
//     |_ _ _ _|

pure fn next_state_hack(grid : u16) -> u8 {
    let nalive = bitset_hack(grid) + bitset_hack(grid >> 4u) +
                 bitset_hack(grid >> 8u);
    alt (grid & 0b10_0000_u16 > 0u16, nalive) {
      (true, 3u16 to 4u16) | (false, 3u16) { 1u8 }
      _ { 0u8 }
    }
}

// Returns the number of bits set among the low 3 bits.
pure fn bitset_hack(grid : u16) -> u16 {
    ret (grid & 1u16) + ((grid & 2u16) >> 1u) + ((grid & 4u16) >> 2u);
}

// Main program
fn main() {
    println("done");
}
