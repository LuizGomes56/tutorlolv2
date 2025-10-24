use super::*;

impl Generator<Item> for TrinityForce {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }