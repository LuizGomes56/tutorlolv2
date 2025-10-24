use super::*;

impl Generator<Item> for RaiseMorale {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }