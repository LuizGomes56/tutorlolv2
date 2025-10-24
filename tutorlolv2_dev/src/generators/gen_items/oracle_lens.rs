use super::*;

impl Generator<Item> for OracleLens {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }