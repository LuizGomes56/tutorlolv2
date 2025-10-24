use super::*;

impl Generator<Item> for JuiceofHaste {
            #[item_gen_v2]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }