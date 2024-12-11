use crate::containers::IngredientsContainer;

#[derive(Clone, Debug)]
pub struct Reciepes {
    pub name: String,
    pub ingredients: IngredientsContainer,
}

impl Reciepes {
    pub fn new(name: String, ingredients: IngredientsContainer) -> Self {
        Reciepes { name, ingredients }
    }

    pub fn get_reciepes() -> Vec<Reciepes> {
        vec![
            Reciepes::new(
                "Espresso".to_string(),
                IngredientsContainer {
                    water: 30.0,
                    coffee: 30.0,
                    milk: 0.0,
                    sugar: 0.0,
                    cacao: 0.0,
                },
            ),
            Reciepes::new(
                "Americano".to_string(),
                IngredientsContainer {
                    water: 30.0,
                    coffee: 30.0,
                    milk: 0.0,
                    sugar: 0.0,
                    cacao: 0.0,
                },
            ),
            Reciepes::new(
                "Cappuccino".to_string(),
                IngredientsContainer {
                    water: 30.0,
                    coffee: 30.0,
                    milk: 30.0,
                    sugar: 0.0,
                    cacao: 0.0,
                },
            ),
            Reciepes::new(
                "Latte".to_string(),
                IngredientsContainer {
                    water: 30.0,
                    coffee: 30.0,
                    milk: 50.0,
                    sugar: 0.0,
                    cacao: 0.0,
                },
            ),
            Reciepes::new(
                "Mocha".to_string(),
                IngredientsContainer {
                    water: 30.0,
                    coffee: 30.0,
                    milk: 30.0,
                    sugar: 30.0,
                    cacao: 30.0,
                },
            ),
        ]
    }
}
