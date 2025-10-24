use super::*;

impl Generator<Item> for EssenceReaver {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }