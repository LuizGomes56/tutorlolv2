use super::*;

impl Generator<Champion> for Zilean {
    #[generator_v2]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (1, 0, _1)];
    }
}
