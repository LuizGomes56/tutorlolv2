use super::*;

impl Generator<ItemData> for ArchangelsStaff {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
