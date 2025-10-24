use super::*;

impl Generator<Item> for ShieldofMoltenStone {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }