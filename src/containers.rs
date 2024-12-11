#[derive(Clone, Debug)]
pub struct IngredientsContainer {
    pub water: f32,
    pub coffee: f32,
    pub milk: f32,
    pub sugar: f32,
    pub cacao: f32,
}

#[derive(Debug)]
pub struct GarbageContainer {
    pub coffee_grounds: f32,
}
