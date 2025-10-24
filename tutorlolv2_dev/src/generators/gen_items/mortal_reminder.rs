use super::*;

impl Generator<Item> for MortalReminder {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }