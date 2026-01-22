use super::*;

impl Generator<ItemData> for StealthWard {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
