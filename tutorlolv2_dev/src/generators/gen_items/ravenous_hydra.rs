use super::*;

impl Generator<Item> for RavenousHydra {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }