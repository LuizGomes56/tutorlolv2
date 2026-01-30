use super::*;

impl Generator<ItemData> for HullbreakerArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
