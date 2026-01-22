use super::*;

impl Generator<ItemData> for VoidStaff {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
