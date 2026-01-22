use super::*;

impl Generator<ItemData> for LoversRicochet {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
