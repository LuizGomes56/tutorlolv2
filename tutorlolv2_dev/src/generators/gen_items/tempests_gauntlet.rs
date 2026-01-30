use super::*;

impl Generator<ItemData> for TempestsGauntlet {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
