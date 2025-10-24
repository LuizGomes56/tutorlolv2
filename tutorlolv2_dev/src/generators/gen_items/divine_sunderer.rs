use super::*;

impl Generator<Item> for DivineSunderer {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }