use super::*;

impl Generator<ItemData> for PartyFavor {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
