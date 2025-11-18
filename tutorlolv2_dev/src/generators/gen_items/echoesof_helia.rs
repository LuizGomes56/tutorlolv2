use super::*;

impl Generator<ItemData> for EchoesofHelia {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
