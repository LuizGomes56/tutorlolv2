use super::*;

impl Generator<ItemData> for MoonstoneRenewerU32 {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
