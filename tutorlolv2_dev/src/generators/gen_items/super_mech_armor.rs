use super::*;

impl Generator<Item> for SuperMechArmor {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }