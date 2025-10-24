use super::*;

impl Generator<Item> for HextechGunblade {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }