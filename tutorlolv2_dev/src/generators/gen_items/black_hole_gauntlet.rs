use super::*;

impl Generator<ItemData> for BlackHoleGauntlet {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
