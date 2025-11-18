use super::*;

impl Generator<ItemData> for Morellonomicon {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
