use super::*;

impl Generator<Item> for SteraksGage {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }