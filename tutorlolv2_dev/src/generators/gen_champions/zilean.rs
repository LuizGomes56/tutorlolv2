use super::*;

impl Generator<Champion> for Zilean {
    #[champion_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (1, 0, _1)];
    }
}
