use super::*;

impl Generator<Item> for WingedMoonplate {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }