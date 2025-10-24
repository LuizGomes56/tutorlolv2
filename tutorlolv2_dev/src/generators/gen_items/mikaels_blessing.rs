use super::*;

impl Generator<Item> for MikaelsBlessing {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }