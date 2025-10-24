use super::*;

impl Generator<Item> for DemonicEmbrace {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }