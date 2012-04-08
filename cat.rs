fn say(s: str) {
    import io::*;
    stderr().write_line(s);
}

fn main(args: [str]) {
    let infile = args[1u];

    let contents = str::from_bytes(result::get(io::read_whole_file(infile)));
    say("read file");

    // This is fast.
    io::print(contents);

    /* This is very slow, probably due to a lack of buffering. A source dive
     * indicates that io::println just calls write(2) directly. */
    // str::lines_iter(contents) {|line|
    //     io::println(line)
    // }
}
