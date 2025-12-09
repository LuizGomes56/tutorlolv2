use super::*;

impl Generator<ItemData> for ForceOfNature {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
