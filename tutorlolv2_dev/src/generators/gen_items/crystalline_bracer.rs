use super::*;

impl Generator<ItemData> for CrystallineBracer {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
