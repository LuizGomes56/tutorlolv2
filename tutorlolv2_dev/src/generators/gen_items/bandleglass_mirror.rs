use super::*;

impl Generator<Item> for BandleglassMirror {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }