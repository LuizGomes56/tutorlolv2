use super::*;

impl Generator<Item> for SeryldasGrudge {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }