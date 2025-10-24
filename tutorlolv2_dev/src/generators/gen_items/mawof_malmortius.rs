use super::*;

impl Generator<Item> for MawofMalmortius {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }