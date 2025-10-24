use super::*;

impl Generator<Item> for BaseTurretReinforcedArmorTurretItem {
    #[item_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Item> {}
}
