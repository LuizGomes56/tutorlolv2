use super::*;

impl Generator<ItemData> for MirageBlade {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
