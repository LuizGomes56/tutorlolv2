use super::*;

impl Generator<ItemData> for JungleQuest {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
