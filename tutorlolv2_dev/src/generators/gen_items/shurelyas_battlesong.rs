use super::*;

impl Generator<Item> for ShurelyasBattlesong {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }