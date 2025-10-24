use super::*;

impl Generator<Item> for VampiricScepter {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }