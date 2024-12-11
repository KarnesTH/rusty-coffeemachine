use rusty_coffeemachine::CoffeeMachine;

/// Main function
///
/// This function creates a new coffee machine and runs it
fn main() {
    let mut machine = CoffeeMachine::new();

    machine.run();
}
