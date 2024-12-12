use crate::containers::{GarbageContainer, IngredientsContainer};
use crate::reciepes::Reciepes;
use crate::{get_input, print_line, ProgressBar};

#[derive(Debug)]
pub struct CoffeeMachine {
    pub ingredients_container: IngredientsContainer,
    pub garbage_container: GarbageContainer,
    pub reciepes: Vec<Reciepes>,
}

impl CoffeeMachine {
    /// Create a new coffee machine
    ///
    /// # Returns
    ///
    /// A new coffee machine
    ///
    /// # Errors
    ///
    /// This function will return an error if creating the coffee machine fails
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_coffeemachine::CoffeeMachine;
    /// let machine = CoffeeMachine::new();
    /// ```
    pub fn new() -> Result<Self, std::io::Error> {
        let machine = CoffeeMachine {
            ingredients_container: IngredientsContainer {
                water: 100.0,
                coffee: 100.0,
                milk: 100.0,
                sugar: 100.0,
                cacao: 100.0,
            },
            garbage_container: GarbageContainer {
                coffee_grounds: 0.0,
            },
            reciepes: Reciepes::get_reciepes()?,
        };

        Ok(machine)
    }

    /// Run the coffee machine
    ///
    /// This function runs the coffee machine and allows the user to interact with it
    ///
    /// # Returns
    ///
    /// An empty result
    ///
    /// # Errors
    ///
    /// This function will return an error if running the coffee machine fails
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rusty_coffeemachine::CoffeeMachine;
    /// let mut machine = CoffeeMachine::new().unwrap();
    /// machine.run().unwrap();
    /// ```
    pub fn run(&mut self) -> Result<(), std::io::Error> {
        self.start_up()?;
        loop {
            self.print_main_menu()?;
            let choice = get_input()?
                .parse::<usize>()
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;
            match choice {
                1 => {
                    self.print_menu()?;
                    let choice = get_input()?
                        .parse::<usize>()
                        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;
                    self.make_coffee(choice)?;
                }
                2 => {
                    self.print_ingredients()?;
                }
                3 => {
                    self.print_garbage()?;
                }
                4 => {
                    println!("Servicing...");
                    self.take_service()?;
                    self.draw_progress(200)?;
                    println!("Service done.");
                }
                5 => {
                    println!("Shutting down...");
                    self.draw_progress(50)?;
                    break;
                }
                _ => {
                    println!("Invalid choice");
                }
            }
        }

        Ok(())
    }

    /// Start up the coffee machine
    ///
    /// This function prints the starts up the coffee machine to the terminal.
    ///
    /// # Returns
    ///
    /// An empty result
    ///
    /// # Errors
    ///
    /// This function will return an error if writing to the terminal fails
    fn start_up(&self) -> Result<(), std::io::Error> {
        println!("Welcome to the coffee machine");
        println!("Starting machine...");
        self.draw_progress(50)?;
        println!("Machine ready.");

        Ok(())
    }

    /// Draw a progress bar
    ///
    /// This function draws a progress bar to the terminal
    ///
    /// # Arguments
    ///
    /// * `duration` - The duration of the progress bar
    ///
    /// # Returns
    ///
    /// An empty result
    ///
    /// # Errors
    ///
    /// This function will return an error if writing to the terminal fails
    fn draw_progress(&self, duration: u64) -> Result<(), std::io::Error> {
        let mut progress_bar = ProgressBar::new(100.0);
        for i in 0..=100 {
            let _ = progress_bar.set_progress(i as f32);
            progress_bar.draw()?;
            std::thread::sleep(std::time::Duration::from_millis(duration));
        }
        println!();

        Ok(())
    }

    /// Print the main menu
    ///
    /// This function prints the main menu to the terminal
    ///
    /// # Returns
    ///
    /// An empty result
    ///
    /// # Errors
    ///
    /// This function will return an error if writing to the terminal fails
    fn print_main_menu(&self) -> Result<(), std::io::Error> {
        print_line()?;
        println!("1. Make coffee");
        println!("2. Check ingredients");
        println!("3. Check garbage");
        println!("4. Service");
        println!("5. Exit");
        print_line()?;

        Ok(())
    }

    /// Print the coffee menu
    ///
    /// This function prints the coffee menu to the terminal
    ///
    /// # Returns
    ///
    /// An empty result
    ///
    /// # Errors
    ///
    /// This function will return an error if writing to the terminal fails
    fn print_menu(&self) -> Result<(), std::io::Error> {
        print_line()?;
        println!("Choose a coffee:");
        for (i, reciepe) in self.reciepes.iter().enumerate() {
            println!("{}. {}", i + 1, reciepe.name);
        }
        print_line()?;

        Ok(())
    }

    /// Print the ingredients
    ///
    /// This function prints the ingredients to the terminal
    ///
    /// # Returns
    ///
    /// An empty result
    ///
    /// # Errors
    ///
    /// This function will return an error if writing to the terminal fails
    fn print_ingredients(&self) -> Result<(), std::io::Error> {
        print_line()?;
        println!("Ingredients:");
        println!("Water: {}", self.ingredients_container.water);
        println!("Coffee: {}", self.ingredients_container.coffee);
        println!("Milk: {}", self.ingredients_container.milk);
        println!("Sugar: {}", self.ingredients_container.sugar);
        println!("Cacao: {}", self.ingredients_container.cacao);
        print_line()?;

        Ok(())
    }

    /// Print the garbage
    ///
    /// This function prints the garbage to the terminal
    ///
    /// # Returns
    ///
    /// An empty result
    ///
    /// # Errors
    ///
    /// This function will return an error if writing to the terminal fails
    fn print_garbage(&self) -> Result<(), std::io::Error> {
        print_line()?;
        println!("Garbage:");
        println!("Coffee grounds: {}", self.garbage_container.coffee_grounds);
        print_line()?;

        Ok(())
    }

    /// Make a coffee
    ///
    /// This function makes a coffee
    ///
    /// # Arguments
    ///
    /// * `choice` - The choice of coffee to make
    ///
    /// # Returns
    ///
    /// An empty result
    ///
    /// # Errors
    ///
    /// This function will return an error if making the coffee fails
    fn make_coffee(&mut self, choice: usize) -> Result<(), std::io::Error> {
        let reciepe = &self.reciepes.clone()[choice - 1];
        if self.check_ingredients(&reciepe.ingredients)? {
            self.use_ingredients(&reciepe.ingredients)?;
            println!("Make your {}", reciepe.name);
            self.draw_progress(100)?;
            println!("{} ready to go.", reciepe.name);
        } else {
            println!("Not enough ingredients");
        }

        Ok(())
    }

    /// Check if there are enough ingredients
    ///
    /// This function checks if there are enough ingredients to make a coffee
    ///
    /// # Arguments
    ///
    /// * `ingredients` - The ingredients needed to make a coffee
    ///
    /// # Returns
    ///
    /// A boolean indicating if there are enough ingredients
    ///
    /// # Errors
    ///
    /// This function will return an error if checking the ingredients fails
    fn check_ingredients(
        &self,
        ingredients: &IngredientsContainer,
    ) -> Result<bool, std::io::Error> {
        let result = self.ingredients_container.water >= ingredients.water
            && self.ingredients_container.coffee >= ingredients.coffee
            && self.ingredients_container.milk >= ingredients.milk
            && self.ingredients_container.sugar >= ingredients.sugar
            && self.ingredients_container.cacao >= ingredients.cacao;

        Ok(result)
    }

    /// Use ingredients
    ///
    /// This function uses ingredients to make a coffee
    ///
    /// # Arguments
    ///
    /// * `ingredients` - The ingredients to use
    ///
    /// # Returns
    ///
    /// An empty result
    ///
    /// # Errors
    ///
    /// This function will return an error if using the ingredients fails
    fn use_ingredients(
        &mut self,
        ingredients: &IngredientsContainer,
    ) -> Result<(), std::io::Error> {
        self.ingredients_container.water -= ingredients.water;
        self.ingredients_container.coffee -= ingredients.coffee;
        self.ingredients_container.milk -= ingredients.milk;
        self.ingredients_container.sugar -= ingredients.sugar;
        self.ingredients_container.cacao -= ingredients.cacao;

        if ingredients.coffee > 0.0 {
            self.garbage_container.coffee_grounds += ingredients.coffee;
        }

        Ok(())
    }

    /// Take service
    ///
    /// This function takes the service of the coffee machine, refilling the ingredients and emptying the garbage
    ///
    /// # Returns
    ///
    /// An empty result
    ///
    /// # Errors
    ///
    /// This function will return an error if taking the service fails
    fn take_service(&mut self) -> Result<(), std::io::Error> {
        self.ingredients_container.water = 100.0;
        self.ingredients_container.coffee = 100.0;
        self.ingredients_container.milk = 100.0;
        self.ingredients_container.sugar = 100.0;
        self.ingredients_container.cacao = 100.0;
        self.garbage_container.coffee_grounds = 0.0;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coffee_machine_new() {
        let machine = CoffeeMachine::new().unwrap();
        assert_eq!(machine.ingredients_container.water, 100.0);
        assert_eq!(machine.ingredients_container.coffee, 100.0);
        assert_eq!(machine.ingredients_container.milk, 100.0);
        assert_eq!(machine.ingredients_container.sugar, 100.0);
        assert_eq!(machine.ingredients_container.cacao, 100.0);
        assert_eq!(machine.garbage_container.coffee_grounds, 0.0);
        assert_eq!(machine.reciepes.len(), 5);
    }

    #[test]
    fn test_coffee_machine_check_ingredients() {
        let machine = CoffeeMachine::new().unwrap();
        let reciepe = Reciepes::new(
            "Espresso".to_string(),
            IngredientsContainer {
                water: 30.0,
                coffee: 30.0,
                milk: 0.0,
                sugar: 0.0,
                cacao: 0.0,
            },
        )
        .unwrap();
        assert_eq!(
            machine.check_ingredients(&reciepe.ingredients).unwrap(),
            true
        );
    }

    #[test]
    fn test_coffee_machine_use_ingredients() {
        let mut machine = CoffeeMachine::new().unwrap();
        let reciepe = Reciepes::new(
            "Espresso".to_string(),
            IngredientsContainer {
                water: 30.0,
                coffee: 30.0,
                milk: 0.0,
                sugar: 0.0,
                cacao: 0.0,
            },
        )
        .unwrap();
        machine.use_ingredients(&reciepe.ingredients).unwrap();
        assert_eq!(machine.ingredients_container.water, 70.0);
        assert_eq!(machine.ingredients_container.coffee, 70.0);
        assert_eq!(machine.ingredients_container.milk, 100.0);
        assert_eq!(machine.ingredients_container.sugar, 100.0);
        assert_eq!(machine.ingredients_container.cacao, 100.0);
        assert_eq!(machine.garbage_container.coffee_grounds, 30.0);
    }

    #[test]
    fn test_coffee_machine_take_service() {
        let mut machine = CoffeeMachine::new().unwrap();
        machine.ingredients_container.water = 50.0;
        machine.ingredients_container.coffee = 100.0;
        machine.ingredients_container.milk = 50.0;
        machine.ingredients_container.sugar = 100.0;
        machine.ingredients_container.cacao = 100.0;
        machine.garbage_container.coffee_grounds = 30.0;
        machine.take_service().unwrap();
        assert_eq!(machine.ingredients_container.water, 100.0);
        assert_eq!(machine.ingredients_container.coffee, 100.0);
        assert_eq!(machine.ingredients_container.milk, 100.0);
        assert_eq!(machine.ingredients_container.sugar, 100.0);
        assert_eq!(machine.ingredients_container.cacao, 100.0);
        assert_eq!(machine.garbage_container.coffee_grounds, 0.0);
    }

    #[test]
    fn test_insufficient_ingrediants() {
        let machine = CoffeeMachine::new().unwrap();
        let reciepe = Reciepes::new(
            "Large".to_string(),
            IngredientsContainer {
                water: 150.0,
                coffee: 30.0,
                milk: 0.0,
                sugar: 0.0,
                cacao: 0.0,
            },
        )
        .unwrap();
        assert_eq!(
            machine.check_ingredients(&reciepe.ingredients).unwrap(),
            false
        );
    }
}
