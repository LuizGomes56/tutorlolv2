use super::*;

impl Generator<ItemData> for SupportQuest {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
