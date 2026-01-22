use super::*;

impl Generator<ItemData> for BloodlettersCurse {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
