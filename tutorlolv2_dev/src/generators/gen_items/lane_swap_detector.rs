use super::*;

impl Generator<Item> for LaneSwapDetector {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }