# AntePost

Companion code for an article on [Ante](https://antelang.org/)'s approach to blending reference counting and borrow checking.

## Contents

- `python-rbtree/` — Okasaki red-black tree in Python (structural pattern matching).
- `rust-rbtree/` — Same tree in Rust using `Arc<RbTree<T>>`.
- `cpp-rbtree/` — Same tree in C++ using `std::variant` + `std::shared_ptr`.
- `rust-spaceship/` — `Rc<RefCell<Spaceship>>` example showing the `.borrow_mut()` runtime panic risk.
- `swift-spaceship/` — Swift `inout` example showing the exclusive-access runtime crash.

## Running

```sh
python3 python-rbtree/rbtree.py

clang++ -std=c++17 cpp-rbtree/main.cpp -o cpp-rbtree/rbtree && ./cpp-rbtree/rbtree

cargo run --manifest-path rust-rbtree/Cargo.toml
cargo run --manifest-path rust-spaceship/Cargo.toml

swift run --package-path swift-spaceship
```
