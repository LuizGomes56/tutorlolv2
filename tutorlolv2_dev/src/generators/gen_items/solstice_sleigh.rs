use super::*;

impl Generator<ItemData> for SolsticeSleigh {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
