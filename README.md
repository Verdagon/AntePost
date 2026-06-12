# AntePost

Companion code for an article on [Ante](https://antelang.org/)'s approach to blending reference counting and borrow checking.

## Contents

- `python-rbtree/` — Okasaki red-black tree in Python (structural pattern matching).
- `rust-rbtree/` — Same tree in Rust using `Arc<RbTree<T>>`.
- `cpp-rbtree/` — Same tree in C++ using `std::variant` + `std::shared_ptr`.
- `rust-spaceship/` — `Rc<RefCell<Spaceship>>` example showing the `.borrow_mut()` runtime panic risk.
- `swift-spaceship/` — Swift `inout` example showing the exclusive-access runtime crash.
- `rust-union/` — same as `rust-spaceship` but with `Engine` as an `enum`, matching the union case.
- `swift-union/` — same as `swift-spaceship` but with `Engine` as a Swift `enum`.
- `rust-twomuts/` — minimal demo of Rust's compile-time rule against two simultaneous `&mut` to the same data. **Intentionally does not compile.**
- `swift-twomuts/` — minimal demo of Swift's compile-time rule against two `inout` arguments aliasing. **Intentionally does not compile.**

## Running

```sh
python3 python-rbtree/rbtree.py

clang++ -std=c++17 cpp-rbtree/main.cpp -o cpp-rbtree/rbtree && ./cpp-rbtree/rbtree

cargo run --manifest-path rust-rbtree/Cargo.toml
cargo run --manifest-path rust-spaceship/Cargo.toml
cargo run --manifest-path rust-union/Cargo.toml

swift run --package-path swift-spaceship
swift run --package-path swift-union

```
