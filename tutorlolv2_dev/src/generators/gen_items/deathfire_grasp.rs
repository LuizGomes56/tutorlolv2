use super::*;

impl Generator<ItemData> for DeathfireGrasp {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
