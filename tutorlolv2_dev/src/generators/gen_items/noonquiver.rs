use super::*;

impl Generator<ItemData> for Noonquiver {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
