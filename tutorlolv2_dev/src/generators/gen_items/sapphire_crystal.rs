use super::*;

impl Generator<ItemData> for SapphireCrystal {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
