use super::*;

impl Generator<Item> for ZhonyasHourglass {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }