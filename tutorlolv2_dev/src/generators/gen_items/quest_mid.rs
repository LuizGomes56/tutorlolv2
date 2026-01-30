use super::*;

impl Generator<ItemData> for QuestMid {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
