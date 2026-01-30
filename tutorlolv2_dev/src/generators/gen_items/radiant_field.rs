use super::*;

impl Generator<ItemData> for RadiantField {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
