use super::*;

impl Generator<ItemData> for MortalReminder {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
