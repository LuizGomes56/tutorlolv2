use super::*;

impl Generator<ItemData> for Fortification {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
