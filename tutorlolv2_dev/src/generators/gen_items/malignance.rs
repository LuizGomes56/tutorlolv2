use super::*;

impl Generator<ItemData> for Malignance {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
