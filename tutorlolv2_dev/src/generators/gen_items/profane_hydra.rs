use super::*;

impl Generator<Item> for ProfaneHydra {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }