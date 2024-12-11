#![allow(unused)]

use crate::containers::{GarbageContainer, IngredientsContainer};
use crate::reciepes::Reciepes;
use crate::{get_input, ProgressBar, LINE_AMOUNT};

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
            reciepes: Reciepes::get_reciepes(),
        }
    }

    fn start_up(&self) {
        println!("Welcome to the coffee machine");
        println!("Starting machine...");
        self.draw_progress(50);
        println!("Machine ready.");
    }

    fn draw_progress(&self, duration: u64) {
        let mut progress_bar = ProgressBar::new(100.0);
        for i in 0..=100 {
            progress_bar.set_progress(i as f32);
            progress_bar.draw();
            std::thread::sleep(std::time::Duration::from_millis(duration));
        }
        println!();
    }

    pub fn run(&mut self) {
        self.start_up();
        loop {
            self.print_main_menu();
            let choice = get_input().parse::<usize>().unwrap();
            match choice {
                1 => {
                    self.print_menu();
                    let choice = get_input().parse::<usize>().unwrap();
                    self.make_coffee(choice);
                }
                2 => {
                    self.print_ingredients();
                }
                3 => {
                    self.print_garbage();
                }
                4 => {
                    println!("Servicing...");
                    self.take_service();
                    self.draw_progress(200);
                    println!("Service done.");
                }
                5 => {
                    println!("Shutting down...");
                    self.draw_progress(50);
                    break;
                }
                _ => {
                    println!("Invalid choice");
                }
            }
        }
    }

    fn print_main_menu(&self) {
        self.print_line();
        println!("1. Make coffee");
        println!("2. Check ingredients");
        println!("3. Check garbage");
        println!("4. Service");
        println!("5. Exit");
        self.print_line();
    }

    fn print_menu(&self) {
        self.print_line();
        println!("Choose a coffee:");
        for (i, reciepe) in self.reciepes.iter().enumerate() {
            println!("{}. {}", i + 1, reciepe.name);
        }
        self.print_line();
    }

    fn print_ingredients(&self) {
        self.print_line();
        println!("Ingredients:");
        println!("Water: {}", self.ingredients_container.water);
        println!("Coffee: {}", self.ingredients_container.coffee);
        println!("Milk: {}", self.ingredients_container.milk);
        println!("Sugar: {}", self.ingredients_container.sugar);
        println!("Cacao: {}", self.ingredients_container.cacao);
        self.print_line();
    }

    fn print_garbage(&self) {
        self.print_line();
        println!("Garbage:");
        println!("Coffee grounds: {}", self.garbage_container.coffee_grounds);
        self.print_line();
    }

    fn print_line(&self) {
        println!("{}", "~".repeat(LINE_AMOUNT))
    }

    fn make_coffee(&mut self, choice: usize) {
        let reciepe = &self.reciepes.clone()[choice - 1];
        if self.check_ingredients(&reciepe.ingredients) {
            self.use_ingredients(&reciepe.ingredients);
            println!("Make your {}", reciepe.name);
            self.draw_progress(100);
            println!("{} ready to go.", reciepe.name);
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

        if ingredients.coffee > 0.0 {
            self.garbage_container.coffee_grounds += ingredients.coffee;
        }
    }

    fn take_service(&mut self) {
        self.ingredients_container.water = 100.0;
        self.ingredients_container.coffee = 100.0;
        self.ingredients_container.milk = 100.0;
        self.ingredients_container.sugar = 100.0;
        self.ingredients_container.cacao = 100.0;
        self.garbage_container.coffee_grounds = 0.0;
    }
}
