use super::*;

impl Generator<Item> for BamisCinder {
    #[item_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Item> {}
}
