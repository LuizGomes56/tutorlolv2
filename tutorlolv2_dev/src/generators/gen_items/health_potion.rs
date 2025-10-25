use super::*;

impl Generator<ItemData> for HealthPotion {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
