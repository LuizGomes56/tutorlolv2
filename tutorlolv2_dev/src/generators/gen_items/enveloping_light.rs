use super::*;

impl Generator<ItemData> for EnvelopingLight {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
