use super::*;

impl Generator<Item> for ScorchclawPup {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }