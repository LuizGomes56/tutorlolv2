use super::*;

impl Generator<Champion> for Jax {
    #[generator_v2]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (1, 0, _1)];
        ability![W, (0, 0, _1)];
        ability![E, (1, 0, _1), (1, 1, _2)];
        ability![R, (0, 4, _1), (2, 0, _2)];
    }
}
