use super::*;

impl Generator<ItemData> for GlowingMote {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
