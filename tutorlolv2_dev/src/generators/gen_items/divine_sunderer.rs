use super::*;

impl Generator<ItemData> for DivineSunderer {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
