use super::*;

impl Generator<ItemData> for Morellonomicon {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
