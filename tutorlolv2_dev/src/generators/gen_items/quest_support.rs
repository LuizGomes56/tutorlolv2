use super::*;

impl Generator<ItemData> for QuestSupport {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
