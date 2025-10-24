use super::*;

impl Generator<Item> for FrozenHeart {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }