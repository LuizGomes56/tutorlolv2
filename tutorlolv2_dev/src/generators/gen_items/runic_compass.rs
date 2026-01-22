use super::*;

impl Generator<ItemData> for RunicCompass {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
