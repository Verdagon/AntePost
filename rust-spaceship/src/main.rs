use std::cell::RefCell;
use std::rc::Rc;

struct Engine {
    fuel: i32,
}

#[allow(dead_code)]
struct Spaceship {
    engine: Engine,
    name: String,
}

fn launch(ship: Rc<RefCell<Spaceship>>) {
    set_fuel(&mut ship.borrow_mut().engine);
    // borrow_mut() panics at runtime if any other borrow is already active
}

fn set_fuel(engine: &mut Engine) {
    engine.fuel = 100;
}

fn main() {
    let ship = Rc::new(RefCell::new(Spaceship {
        engine: Engine { fuel: 0 },
        name: "Enterprise".to_string(),
    }));

    launch(Rc::clone(&ship));
    assert_eq!(ship.borrow().engine.fuel, 100);
    println!("Fuel: {}", ship.borrow().engine.fuel);

    // Now trigger the runtime panic ("already borrowed: BorrowMutError"):
    let _guard = ship.borrow();
    launch(Rc::clone(&ship));
}
