// This file deliberately fails to compile. That IS the demonstration:
// Swift forbids passing the same variable to two `inout` parameters
// of a single function call.
//
//   error: inout arguments are not allowed to alias each other
//   note: previous aliasing argument

func add(_ a: inout Int, _ b: inout Int) {
    a += b
}

var x = 5
add(&x, &x)
