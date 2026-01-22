use super::*;

impl Generator<ItemData> for IceblastArmor {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
