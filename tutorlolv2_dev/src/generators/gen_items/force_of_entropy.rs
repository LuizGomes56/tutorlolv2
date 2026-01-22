use super::*;

impl Generator<ItemData> for ForceOfEntropy {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
