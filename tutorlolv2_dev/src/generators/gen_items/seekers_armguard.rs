use super::*;

impl Generator<ItemData> for SeekersArmguard {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
