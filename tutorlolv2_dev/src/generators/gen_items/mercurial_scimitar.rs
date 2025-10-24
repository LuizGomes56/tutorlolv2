use super::*;

impl Generator<Item> for MercurialScimitar {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }