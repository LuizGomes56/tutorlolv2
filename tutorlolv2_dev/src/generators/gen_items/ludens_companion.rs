use super::*;

impl Generator<ItemData> for LudensCompanion {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
