use rusty_coffeemachine::CoffeeMachine;

/// Main function
///
/// This function creates a new coffee machine and runs it
fn main() -> Result<(), std::io::Error> {
    let mut machine = CoffeeMachine::new();

    machine.run()?;

    Ok(())
}
