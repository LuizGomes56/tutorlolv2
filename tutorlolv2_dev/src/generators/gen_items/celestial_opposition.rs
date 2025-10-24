use super::*;

impl Generator<Item> for CelestialOpposition {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }