use super::*;

impl Generator<ItemData> for MobilityBoots {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
