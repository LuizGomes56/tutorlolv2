use super::*;

impl Generator<ItemData> for VigilantWardstone {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
