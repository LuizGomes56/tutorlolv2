use super::*;

impl Generator<Item> for SunderedSky {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }