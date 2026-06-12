// With `Cell`, multiple shared references can coexist and each
// can mutate the inner value via `.set()` / `.get()`. That's how
// Rust gets close to "multiple &mut to the same data."
//
// But there's a catch: you can never get a `&mut T` to the inside.
// That makes it awkward for anything but trivial types — see the
// appendix on the swap-back dance Cell forces on you for Strings.

use std::cell::Cell;

fn main() {
    let x = Cell::new(5);
    let a = &x;
    let b = &x;
    a.set(1);
    b.set(2);
    println!("x = {}", x.get());
}
