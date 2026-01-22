use super::*;

impl Generator<ItemData> for ManamuneArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
