use super::*;

impl Generator<ItemData> for ShatteredArmguard {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
