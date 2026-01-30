use super::*;

impl Generator<ItemData> for StridebreakerArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
