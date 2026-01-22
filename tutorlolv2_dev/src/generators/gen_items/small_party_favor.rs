use super::*;

impl Generator<ItemData> for SmallPartyFavor {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
