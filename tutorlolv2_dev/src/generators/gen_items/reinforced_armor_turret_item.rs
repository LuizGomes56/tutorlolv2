use super::*;

impl Generator<Item> for ReinforcedArmorTurretItem {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }