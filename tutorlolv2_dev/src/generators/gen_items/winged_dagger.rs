use super::*;

impl Generator<ItemData> for WingedDagger {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
