use super::*;

impl Generator<Item> for WatchfulWardstone {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }