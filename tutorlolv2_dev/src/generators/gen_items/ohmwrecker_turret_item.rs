use super::*;

impl Generator<Item> for OhmwreckerTurretItem {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }