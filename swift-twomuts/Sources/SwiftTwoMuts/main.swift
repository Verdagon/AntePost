func add(_ a: inout Int, _ b: inout Int) {
    a += b
}

var x = 5
add(&x, &x)

//   error: inout arguments are not allowed to alias each other
//   note: previous aliasing argument