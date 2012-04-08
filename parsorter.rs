use parsort;

import parsort::parsort;

// we must force le to take args by reference, so that it matches the general
// calling convention for things of type (fn(T,T) -> bool).
fn intle(&&x: int, &&y: int) -> bool { x <= y }

fn errsay(s:str) { import io::*; stderr().write_line(s); }

fn run_sort(nthreads: uint, nums: [int]) -> [int] {
    parsort(intle, nthreads, nums)
}

fn main(args: [str]) {
    import result::{ok,err};
    import option::*;

    let nthreads = get(uint::from_str(args[1u]));
    let file = args[2u];

    // Read the file
    let contents = str::from_bytes(result::get(io::read_whole_file(file)));
    let mut nums = [];
    str::lines_iter(contents) {|line|
        if !str::is_empty(line) {
            nums += [option::get(int::from_str(line))];
        }
    }
    errsay("done reading");

    // Sort 'em.
    let nums_sorted = run_sort(nthreads, nums);
    errsay("done sorting");
    nums_sorted;

    // for some reason, this is a bottleneck
    // vec::iter(nums_sorted) {|x| io::println(#fmt("%d", x)); }
}
