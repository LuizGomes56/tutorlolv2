use super::*;

impl Generator<ItemData> for AntiTowerSocks {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
