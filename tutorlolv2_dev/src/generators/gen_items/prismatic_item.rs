use super::*;

impl Generator<Item> for PrismaticItem {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }