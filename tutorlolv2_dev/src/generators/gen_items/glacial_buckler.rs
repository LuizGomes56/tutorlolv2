use super::*;

impl Generator<Item> for GlacialBuckler {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }