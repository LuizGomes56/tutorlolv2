use super::*;

impl Generator<ItemData> for BlastingWand {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
