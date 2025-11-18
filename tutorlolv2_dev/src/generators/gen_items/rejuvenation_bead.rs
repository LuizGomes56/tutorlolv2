use super::*;

impl Generator<ItemData> for RejuvenationBead {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
