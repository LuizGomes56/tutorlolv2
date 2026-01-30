use super::*;

impl Generator<ItemData> for DemonicEmbraceArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
