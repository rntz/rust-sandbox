// fails to compile, confusing error message
macro_rules! print(
    ($s:expr, $( $e:expr ),*) => (
        io::println(fmt!($s, $( $e ),*))
    )
)

fn main() {
    print!("%d %d", 0, 1);
}