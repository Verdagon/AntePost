// This file deliberately fails to compile. That IS the demonstration:
// Rust's borrow checker forbids two `&mut` references to the same data
// at the same time.
//
//   error[E0499]: cannot borrow `x` as mutable more than once at a time

fn main() {
    let mut x = 5;
    let a = &mut x;
    let b = &mut x;
    *a = 1;
    *b = 2;
}
