use super::*;

impl Generator<ItemData> for HauntingGuise {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
