use super::*;

impl Generator<ItemData> for EssenceReaver {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
