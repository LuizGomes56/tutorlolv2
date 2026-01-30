use super::*;

impl Generator<ItemData> for VoidStaffArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
