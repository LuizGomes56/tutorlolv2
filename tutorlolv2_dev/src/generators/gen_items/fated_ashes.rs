use super::*;

impl Generator<Item> for FatedAshes {
            #[item_gen_v2]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }