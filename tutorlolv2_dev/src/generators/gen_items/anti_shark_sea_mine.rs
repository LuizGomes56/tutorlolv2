use super::*;

impl Generator<ItemData> for AntiSharkSeaMine {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
