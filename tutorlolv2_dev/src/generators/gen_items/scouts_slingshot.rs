use super::*;

impl Generator<Item> for ScoutsSlingshot {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }