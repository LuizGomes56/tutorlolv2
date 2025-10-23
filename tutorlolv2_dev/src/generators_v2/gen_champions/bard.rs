use super::*;

impl Generator<Champion> for Bard {
    #[generator_v2]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 1, _1)];
    }
}
