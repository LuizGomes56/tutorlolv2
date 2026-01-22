use super::*;

impl Generator<ItemData> for ImmortalShieldbow {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
