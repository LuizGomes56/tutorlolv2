use super::*;

impl Generator<ItemData> for TopLaneQuest {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
