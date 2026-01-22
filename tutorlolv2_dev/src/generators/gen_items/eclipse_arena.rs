use super::*;

impl Generator<ItemData> for EclipseArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
