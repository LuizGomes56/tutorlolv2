use super::*;

impl Generator<ItemData> for TheGoldenSpatula {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
