use super::*;

impl Generator<ItemData> for HearthboundAxe {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
