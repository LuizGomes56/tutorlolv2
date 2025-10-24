use super::*;

impl Generator<Item> for TitanicHydra {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }