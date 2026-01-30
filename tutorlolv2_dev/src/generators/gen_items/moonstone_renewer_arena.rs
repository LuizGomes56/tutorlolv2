use super::*;

impl Generator<ItemData> for MoonstoneRenewerArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
