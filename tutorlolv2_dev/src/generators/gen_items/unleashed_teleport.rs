use super::*;

impl Generator<ItemData> for UnleashedTeleport {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
