use super::*;

impl Generator<ItemData> for Trailblazer {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
