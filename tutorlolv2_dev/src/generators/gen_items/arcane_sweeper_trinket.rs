use super::*;

impl Generator<Item> for ArcaneSweeperTrinket {
    #[item_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Item> {}
}
