use super::*;

impl Generator<Item> for ElixirofSkill {
    #[item_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Item> {}
}
