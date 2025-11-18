use super::*;

impl Generator<ItemData> for LudensCompanion {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
