use super::*;

impl Generator<Item> for NightHarvester {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }