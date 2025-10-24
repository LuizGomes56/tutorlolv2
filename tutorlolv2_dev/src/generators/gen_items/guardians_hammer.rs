use super::*;

impl Generator<Item> for GuardiansHammer {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }