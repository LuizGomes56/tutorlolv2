use super::*;

impl Generator<Item> for InnervatingLocket {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }