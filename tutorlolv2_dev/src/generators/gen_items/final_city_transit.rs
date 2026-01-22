use super::*;

impl Generator<ItemData> for FinalCityTransit {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
