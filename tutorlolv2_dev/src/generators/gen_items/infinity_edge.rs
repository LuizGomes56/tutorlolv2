use super::*;

impl Generator<Item> for InfinityEdge {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }