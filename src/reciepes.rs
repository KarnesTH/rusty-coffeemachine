use crate::containers::IngredientsContainer;

#[derive(Clone, Debug)]
pub struct Reciepes {
    pub name: String,
    pub ingredients: IngredientsContainer,
}
