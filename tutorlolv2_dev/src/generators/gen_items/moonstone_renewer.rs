use super::*;

impl Generator<Item> for MoonstoneRenewer {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }