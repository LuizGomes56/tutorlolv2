use super::*;

impl Generator<ItemData> for TheAnnihilator {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
