use crate::containers::IngredientsContainer;

#[derive(Clone, Debug)]
pub struct Reciepes {
    pub name: String,
    pub ingredients: IngredientsContainer,
}

impl Reciepes {
    /// Create a new reciepe
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the reciepe
    /// * `ingredients` - The ingredients of the reciepe
    ///
    /// # Returns
    ///
    /// A new reciepe
    ///
    /// # Examples
    ///
    /// ```
    /// let reciepe = Reciepes::new("Espresso".to_string(), IngredientsContainer {
    ///    water: 30.0,
    ///    coffee: 30.0,
    ///    milk: 0.0,
    ///    sugar: 0.0,
    ///    cacao: 0.0,
    /// });
    /// ```
    pub fn new(name: String, ingredients: IngredientsContainer) -> Self {
        Reciepes { name, ingredients }
    }

    /// Get a list of reciepes
    ///
    /// # Returns
    ///
    /// A vector containing all the reciepes
    ///
    /// # Examples
    ///
    /// ```
    /// let reciepes = Reciepes::get_reciepes();
    /// ```
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
