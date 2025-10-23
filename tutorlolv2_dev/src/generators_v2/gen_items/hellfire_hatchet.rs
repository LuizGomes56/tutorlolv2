use super::*;

impl Generator<Item> for HellfireHatchet {
            #[item_gen_v2]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }