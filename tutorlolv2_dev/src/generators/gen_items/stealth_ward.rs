use super::*;

impl Generator<Item> for StealthWard {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }