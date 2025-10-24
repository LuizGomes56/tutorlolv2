use super::*;

impl Generator<Item> for GuardiansHorn {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }