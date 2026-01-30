use super::*;

impl Generator<ItemData> for ProtoplasmHarnessArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
