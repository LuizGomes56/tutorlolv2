use super::*;

impl Generator<ItemData> for PenetratingBullets {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
