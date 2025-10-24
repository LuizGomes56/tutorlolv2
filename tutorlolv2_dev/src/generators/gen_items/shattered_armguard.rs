use super::*;

impl Generator<Item> for ShatteredArmguard {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }