use super::*;

impl Generator<ItemData> for MortalReminderArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
