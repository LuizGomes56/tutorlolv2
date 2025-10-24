use super::*;

impl Generator<Item> for HellfireHatchet {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }