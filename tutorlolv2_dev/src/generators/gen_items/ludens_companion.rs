use super::*;

impl Generator<Item> for LudensCompanion {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }