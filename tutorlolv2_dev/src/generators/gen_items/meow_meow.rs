use super::*;

impl Generator<ItemData> for MeowMeow {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
