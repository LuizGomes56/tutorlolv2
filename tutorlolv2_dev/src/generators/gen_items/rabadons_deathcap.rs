use super::*;

impl Generator<ItemData> for RabadonsDeathcap {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
