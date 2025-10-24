use super::*;

impl Generator<Item> for LightningRod {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }