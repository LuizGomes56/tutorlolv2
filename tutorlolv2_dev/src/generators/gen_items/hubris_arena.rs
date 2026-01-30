use super::*;

impl Generator<ItemData> for HubrisArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
