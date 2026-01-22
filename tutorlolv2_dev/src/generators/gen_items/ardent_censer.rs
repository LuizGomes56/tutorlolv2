use super::*;

impl Generator<ItemData> for ArdentCenser {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
