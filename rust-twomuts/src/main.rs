fn main() {
    let mut x = 5;
    let a = &mut x;
    let b = &mut x;
    *a = 1;
    *b = 2;
}

//   error[E0499]: cannot borrow `x` as mutable more than once at a time