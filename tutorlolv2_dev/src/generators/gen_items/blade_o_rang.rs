use super::*;

impl Generator<ItemData> for BladeORang {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
