use super::*;

impl Generator<Item> for MortalReminder {
            #[item_gen_v2]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }