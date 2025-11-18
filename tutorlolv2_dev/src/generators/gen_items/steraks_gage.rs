use super::*;

impl Generator<ItemData> for SteraksGage {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
