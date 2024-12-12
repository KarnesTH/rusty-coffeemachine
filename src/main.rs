use rusty_coffeemachine::CoffeeMachine;

/// Main function
///
/// This function creates a new coffee machine and runs it
///
/// # Returns
///
/// An empty result
///
/// # Errors
///
/// This function will return an error if creating the coffee machine or running it fails
fn main() -> Result<(), std::io::Error> {
    let mut machine = CoffeeMachine::new()?;
    machine.run()?;

    Ok(())
}
