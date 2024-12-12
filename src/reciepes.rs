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
    /// # Errors
    ///
    /// This function will return an error if creating the reciepe fails
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_coffeemachine::Reciepes;
    /// use rusty_coffeemachine::containers::IngredientsContainer;
    /// let reciepe = Reciepes::new("Espresso".to_string(), IngredientsContainer {
    ///    water: 30.0,
    ///    coffee: 30.0,
    ///    milk: 0.0,
    ///    sugar: 0.0,
    ///    cacao: 0.0,
    /// });
    /// ```
    pub fn new(name: String, ingredients: IngredientsContainer) -> Result<Self, std::io::Error> {
        let reciepes = Reciepes { name, ingredients };

        Ok(reciepes)
    }

    /// Get a list of reciepes
    ///
    /// # Returns
    ///
    /// A vector containing all the reciepes
    ///
    /// # Errors
    ///
    /// This function will return an error if creating the reciepes fails
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_coffeemachine::Reciepes;
    /// let reciepes = Reciepes::get_reciepes();
    /// ```
    pub fn get_reciepes() -> Result<Vec<Reciepes>, std::io::Error> {
        let init_reciepes = vec![
            Reciepes::new(
                "Espresso".to_string(),
                IngredientsContainer {
                    water: 30.0,
                    coffee: 30.0,
                    milk: 0.0,
                    sugar: 0.0,
                    cacao: 0.0,
                },
            )?,
            Reciepes::new(
                "Americano".to_string(),
                IngredientsContainer {
                    water: 30.0,
                    coffee: 30.0,
                    milk: 0.0,
                    sugar: 0.0,
                    cacao: 0.0,
                },
            )?,
            Reciepes::new(
                "Cappuccino".to_string(),
                IngredientsContainer {
                    water: 30.0,
                    coffee: 30.0,
                    milk: 30.0,
                    sugar: 0.0,
                    cacao: 0.0,
                },
            )?,
            Reciepes::new(
                "Latte".to_string(),
                IngredientsContainer {
                    water: 30.0,
                    coffee: 30.0,
                    milk: 50.0,
                    sugar: 0.0,
                    cacao: 0.0,
                },
            )?,
            Reciepes::new(
                "Mocha".to_string(),
                IngredientsContainer {
                    water: 30.0,
                    coffee: 30.0,
                    milk: 30.0,
                    sugar: 30.0,
                    cacao: 30.0,
                },
            )?,
        ];
        Ok(init_reciepes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_reciepe() {
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

        assert_eq!(reciepe.name, "Espresso");
        assert_eq!(reciepe.ingredients.water, 30.0);
        assert_eq!(reciepe.ingredients.coffee, 30.0);
        assert_eq!(reciepe.ingredients.milk, 0.0);
        assert_eq!(reciepe.ingredients.sugar, 0.0);
        assert_eq!(reciepe.ingredients.cacao, 0.0);
    }

    #[test]
    fn test_get_reciepes() {
        let reciepes = Reciepes::get_reciepes().unwrap();

        assert_eq!(reciepes.len(), 5);
        assert_eq!(reciepes[0].name, "Espresso");
        assert_eq!(reciepes[1].name, "Americano");
        assert_eq!(reciepes[2].name, "Cappuccino");
        assert_eq!(reciepes[3].name, "Latte");
        assert_eq!(reciepes[4].name, "Mocha");
    }
}
