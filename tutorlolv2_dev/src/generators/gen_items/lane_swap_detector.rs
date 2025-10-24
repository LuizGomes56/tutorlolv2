use super::*;

impl Generator<ItemData> for LaneSwapDetector {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
