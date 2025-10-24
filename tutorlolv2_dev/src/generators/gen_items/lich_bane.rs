use super::*;

impl Generator<Item> for LichBane {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }