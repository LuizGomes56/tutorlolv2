use super::*;

impl Generator<ItemData> for InnervatingLocket {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
