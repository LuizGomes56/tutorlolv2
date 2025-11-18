use super::*;

impl Generator<ItemData> for ExecutionersCalling {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
