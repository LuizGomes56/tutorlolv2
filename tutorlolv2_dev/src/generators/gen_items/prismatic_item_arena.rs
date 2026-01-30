use super::*;

impl Generator<ItemData> for PrismaticItemArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
