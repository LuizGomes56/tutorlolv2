use super::*;

impl Generator<Item> for ExecutionersCalling {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }