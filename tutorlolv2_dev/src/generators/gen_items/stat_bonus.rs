use super::*;

impl Generator<ItemData> for StatBonus {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
