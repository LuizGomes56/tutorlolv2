use super::*;

impl Generator<ItemData> for FireAtWill {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
