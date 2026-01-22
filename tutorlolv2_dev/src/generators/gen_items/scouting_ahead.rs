use super::*;

impl Generator<ItemData> for ScoutingAhead {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
