use super::*;

impl Generator<ItemData> for BlackSpear {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
