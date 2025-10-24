use super::*;

impl Generator<Item> for SynchronizedSouls {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }