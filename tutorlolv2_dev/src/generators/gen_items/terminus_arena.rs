use super::*;

impl Generator<ItemData> for TerminusArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
