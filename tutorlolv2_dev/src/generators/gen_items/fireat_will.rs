use super::*;

impl Generator<ItemData> for FireatWill {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
