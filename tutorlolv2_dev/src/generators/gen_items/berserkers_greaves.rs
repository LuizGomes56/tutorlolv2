use super::*;

impl Generator<Item> for BerserkersGreaves {
    #[item_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Item> {}
}
