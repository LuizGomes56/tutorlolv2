use super::*;

impl Generator<Item> for BloodlettersCurse {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }