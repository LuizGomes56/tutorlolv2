use super::*;

impl Generator<Item> for GuardiansShroud {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }