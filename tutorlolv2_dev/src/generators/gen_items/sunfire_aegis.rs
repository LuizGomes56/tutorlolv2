use super::*;

impl Generator<Item> for SunfireAegis {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }