use super::*;

impl Generator<Item> for GlowingMote {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }