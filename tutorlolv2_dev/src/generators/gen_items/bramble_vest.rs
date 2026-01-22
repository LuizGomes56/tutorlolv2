use super::*;

impl Generator<ItemData> for BrambleVest {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
