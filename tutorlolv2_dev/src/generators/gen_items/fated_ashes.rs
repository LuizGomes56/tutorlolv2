use super::*;

impl Generator<ItemData> for FatedAshes {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
