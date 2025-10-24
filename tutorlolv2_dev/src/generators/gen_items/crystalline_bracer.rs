use super::*;

impl Generator<Item> for CrystallineBracer {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }