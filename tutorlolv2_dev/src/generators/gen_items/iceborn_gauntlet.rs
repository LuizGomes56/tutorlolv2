use super::*;

impl Generator<Item> for IcebornGauntlet {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }