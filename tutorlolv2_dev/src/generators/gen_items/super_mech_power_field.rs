use super::*;

impl Generator<Item> for SuperMechPowerField {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }