use super::*;

impl Generator<Item> for RealityFracture {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }