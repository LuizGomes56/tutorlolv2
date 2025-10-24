use super::*;

impl Generator<Item> for CosmicDrive {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }