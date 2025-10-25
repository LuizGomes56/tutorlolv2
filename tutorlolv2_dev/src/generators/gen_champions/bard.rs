use super::*;

impl Generator<Champion> for Bard {
    #[champion_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 1, _1)];
    }
}
