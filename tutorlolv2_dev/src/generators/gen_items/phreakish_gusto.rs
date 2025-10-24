use super::*;

impl Generator<Item> for PhreakishGusto {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }