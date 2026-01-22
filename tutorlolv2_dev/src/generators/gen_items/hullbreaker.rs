use super::*;

impl Generator<ItemData> for Hullbreaker {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
