use super::*;

impl Generator<ItemData> for AmplifyingTome {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
