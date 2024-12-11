use crate::containers::{GarbageContainer, IngredientsContainer};
use crate::reciepes::Reciepes;

const LINE_AMOUNT: usize = 50;

#[derive(Debug)]
pub struct CoffeeMachine {
    pub ingredients_container: IngredientsContainer,
    pub garbage_container: GarbageContainer,
    pub reciepes: Vec<Reciepes>,
}

impl CoffeeMachine {
    pub fn new() -> Self {
        CoffeeMachine {
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
            reciepes: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            self.print_menu();
            let choice = self.get_choice();
            self.make_coffee(choice);
        }
    }

    fn print_menu(&self) {
        self.print_line();
        println!("Choose a coffee:");
        for (i, reciepe) in self.reciepes.iter().enumerate() {
            println!("{}. {}", i + 1, reciepe.name);
        }
        self.print_line();
    }

    fn print_line(&self) {
        for _ in 0..LINE_AMOUNT {
            print!("~");
        }
        println!();
    }

    fn get_choice(&self) -> usize {
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();
        choice.trim().parse().unwrap()
    }

    fn make_coffee(&mut self, choice: usize) {
        let reciepe = &self.reciepes.clone()[choice - 1];
        if self.check_ingredients(&reciepe.ingredients) {
            self.use_ingredients(&reciepe.ingredients);
            println!("Here is your {}", reciepe.name);
        } else {
            println!("Not enough ingredients");
        }
    }

    fn check_ingredients(&self, ingredients: &IngredientsContainer) -> bool {
        self.ingredients_container.water >= ingredients.water
            && self.ingredients_container.coffee >= ingredients.coffee
            && self.ingredients_container.milk >= ingredients.milk
            && self.ingredients_container.sugar >= ingredients.sugar
            && self.ingredients_container.cacao >= ingredients.cacao
    }

    fn use_ingredients(&mut self, ingredients: &IngredientsContainer) {
        self.ingredients_container.water -= ingredients.water;
        self.ingredients_container.coffee -= ingredients.coffee;
        self.ingredients_container.milk -= ingredients.milk;
        self.ingredients_container.sugar -= ingredients.sugar;
        self.ingredients_container.cacao -= ingredients.cacao;
    }
}
