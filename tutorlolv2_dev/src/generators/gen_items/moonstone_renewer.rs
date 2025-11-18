use super::*;

impl Generator<ItemData> for MoonstoneRenewer {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
