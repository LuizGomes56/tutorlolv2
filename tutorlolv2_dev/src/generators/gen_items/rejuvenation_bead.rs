use super::*;

impl Generator<Item> for RejuvenationBead {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }