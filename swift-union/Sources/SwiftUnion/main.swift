enum Engine {
    case stringTheory(String)
    case impulse(Int32)
}

class Spaceship {
    var engine: Engine
    var name: String
    init(name: String, engine: Engine) {
        self.name = name
        self.engine = engine
    }
}

func launch(_ ship: Spaceship) {
    mutateEngine(&ship.engine)
}

func mutateEngine(_ engine: inout Engine) {
    if case .stringTheory(var s) = engine {
        s.replaceSubrange(s.startIndex...s.startIndex, with: "z")
        engine = .stringTheory(s)
    }
}

// Swift's automatic exclusivity check panics at runtime if ship.engine is
// accessed through any alias while the inout borrow is active.
func launchCrash(_ ship: Spaceship) {
    mutateEngineCrash(ship: ship, engine: &ship.engine)
}

func mutateEngineCrash(ship: Spaceship, engine: inout Engine) {
    // Runtime crash: simultaneous access to ship.engine
    ship.engine = .impulse(42)
    if case .stringTheory(var s) = engine {
        s.replaceSubrange(s.startIndex...s.startIndex, with: "z")
        engine = .stringTheory(s)
    }
}

let ship = Spaceship(name: "Enterprise", engine: .stringTheory("hello"))
launch(ship)
if case let .stringTheory(s) = ship.engine {
    assert(s == "zello")
    print("Engine string: \(s)")
}
print("OK")

// Uncomment to see the runtime crash:
// launchCrash(ship)
