use super::*;

impl Generator<ItemData> for HellfireHatchet {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
