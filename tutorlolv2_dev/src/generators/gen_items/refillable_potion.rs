use super::*;

impl Generator<ItemData> for RefillablePotion {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
