use super::*;

impl Generator<ItemData> for EdgeOfNight {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
