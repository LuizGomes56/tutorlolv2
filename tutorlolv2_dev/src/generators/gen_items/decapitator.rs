use super::*;

impl Generator<ItemData> for Decapitator {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
