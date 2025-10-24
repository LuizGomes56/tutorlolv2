use super::*;

impl Generator<Item> for TwilightsEdge {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }