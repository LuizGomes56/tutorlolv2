use super::*;

impl Generator<Item> for StealthWard {
            #[item_gen_v2]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }