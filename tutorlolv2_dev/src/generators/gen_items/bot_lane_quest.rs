use super::*;

impl Generator<ItemData> for BotLaneQuest {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
