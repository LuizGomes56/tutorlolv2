use super::*;

impl Generator<ItemData> for ControlWard {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
