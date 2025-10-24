use super::*;

impl Generator<Item> for DeathsDance {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }