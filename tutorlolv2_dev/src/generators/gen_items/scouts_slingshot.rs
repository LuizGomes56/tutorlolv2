use super::*;

impl Generator<ItemData> for ScoutsSlingshot {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
