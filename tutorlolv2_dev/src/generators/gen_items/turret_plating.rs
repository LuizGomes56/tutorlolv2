use super::*;

impl Generator<ItemData> for TurretPlating {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
