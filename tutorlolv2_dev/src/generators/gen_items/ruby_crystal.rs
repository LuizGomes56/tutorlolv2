use super::*;

impl Generator<ItemData> for RubyCrystal {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
