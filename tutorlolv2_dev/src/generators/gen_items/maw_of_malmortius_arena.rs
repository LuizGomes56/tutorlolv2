use super::*;

impl Generator<ItemData> for MawOfMalmortiusArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
