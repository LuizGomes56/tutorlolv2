use super::*;

impl Generator<Item> for WorldAtlas {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }