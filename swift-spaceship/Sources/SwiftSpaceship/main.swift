struct Engine {
    var fuel = 0
}

class Spaceship {
    var engine = Engine()
    var name = ""
    init(name: String) { self.name = name }
}

func launch(_ ship: Spaceship) {
    setFuel(ship: ship, engine: &ship.engine)
}

func setFuel(ship: Spaceship, engine: inout Engine) {
    engine.fuel = 100
}

// Swift's automatic exclusivity check panics at runtime if ship.engine is
// accessed through any alias while the inout borrow is active.
func launchCrash(_ ship: Spaceship) {
    setFuelCrash(ship: ship, engine: &ship.engine)
}

func setFuelCrash(ship: Spaceship, engine: inout Engine) {
    engine.fuel = 100
    // Runtime crash: simultaneous access to ship.engine
    setFuel(ship: ship, engine: &ship.engine)
}

let ship = Spaceship(name: "Enterprise")
launch(ship)
assert(ship.engine.fuel == 100)
print("Fuel: \(ship.engine.fuel)")

// Now trigger the runtime crash:
launchCrash(ship)
