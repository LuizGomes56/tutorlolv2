use super::*;

impl Generator<ItemData> for AniMines {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
