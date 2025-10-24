use super::*;

impl Generator<Item> for GuardianAngel {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }