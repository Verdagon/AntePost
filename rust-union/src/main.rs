use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
enum Engine {
    StringTheory(String),
    Impulse(i32),
}

#[allow(dead_code)]
struct Spaceship {
    engine: Engine,
    name: String,
}

fn launch(ship: Rc<RefCell<Spaceship>>) {
    let mut guard = ship.borrow_mut();
    if let Engine::StringTheory(s) = &mut guard.engine {
        // Equivalent to Ante's `str.[0] := 'z'`
        s.replace_range(0..1, "z");
    }
    // borrow_mut() panics at runtime if any other borrow is already active.
}

fn main() {
    let ship = Rc::new(RefCell::new(Spaceship {
        engine: Engine::StringTheory("hello".to_string()),
        name: "Enterprise".to_string(),
    }));

    launch(Rc::clone(&ship));
    if let Engine::StringTheory(s) = &ship.borrow().engine {
        assert_eq!(s, "zello");
        println!("Engine string: {}", s);
    }
    println!("OK");

    // Uncomment to see the runtime panic ("already borrowed: BorrowMutError"):
    // let _guard = ship.borrow();
    // launch(Rc::clone(&ship));
}
