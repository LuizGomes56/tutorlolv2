use super::*;

impl Generator<Item> for EmpyreanPromise {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }