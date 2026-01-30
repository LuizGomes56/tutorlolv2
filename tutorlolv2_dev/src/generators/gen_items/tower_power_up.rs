use super::*;

impl Generator<ItemData> for TowerPowerUp {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
