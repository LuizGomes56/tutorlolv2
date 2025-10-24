use super::*;

impl Generator<Item> for KrakenSlayer {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }