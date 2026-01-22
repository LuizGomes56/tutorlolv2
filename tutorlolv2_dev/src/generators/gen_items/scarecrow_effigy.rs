use super::*;

impl Generator<ItemData> for ScarecrowEffigy {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
