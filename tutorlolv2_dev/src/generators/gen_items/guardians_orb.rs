use super::*;

impl Generator<Item> for GuardiansOrb {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }