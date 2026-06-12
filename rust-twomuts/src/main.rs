// Rust forbids two `&mut` references to the same data at the same time.
// The borrow checker enforces this statically.

fn main() {
    let mut x = 5;
    let a = &mut x;
    *a = 1;
    println!("x = {}", a);

    // Uncomment to see the compile error:
    //
    //   error[E0499]: cannot borrow `y` as mutable more than once at a time
    //
    // let mut y = 5;
    // let a = &mut y;
    // let b = &mut y;
    // *a = 1;
    // *b = 2;
}
