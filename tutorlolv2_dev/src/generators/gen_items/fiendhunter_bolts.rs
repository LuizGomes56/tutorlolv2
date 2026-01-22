use super::*;

impl Generator<ItemData> for FiendhunterBolts {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
