use super::*;

impl Generator<ItemData> for HorizonFocus {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
