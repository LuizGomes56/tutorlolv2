use super::*;

impl Generator<Item> for DetonationOrb {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }