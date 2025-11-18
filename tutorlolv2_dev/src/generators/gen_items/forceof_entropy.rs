use super::*;

impl Generator<ItemData> for ForceofEntropy {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
