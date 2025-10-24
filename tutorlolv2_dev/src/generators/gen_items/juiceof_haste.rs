use super::*;

impl Generator<Item> for JuiceofHaste {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }