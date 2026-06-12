// Swift forbids passing the same variable to two `inout` parameters
// of a single function call. The compiler enforces this statically.

func add(_ a: inout Int, _ b: inout Int) {
    a += b
}

var y = 1
var z = 2
add(&y, &z)
print("y = \(y)")

// Uncomment to see the compile error:
//
//   error: inout arguments are not allowed to alias each other
//   note: previous aliasing argument
//
// var x = 5
// add(&x, &x)
