use super::*;

impl Generator<Item> for MosstomperSeedling {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }