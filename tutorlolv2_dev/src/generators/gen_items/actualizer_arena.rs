use super::*;

impl Generator<ItemData> for ActualizerArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
