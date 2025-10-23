use super::*;

impl Generator<Champion> for TahmKench {
    #[generator_v2]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 1, _1)];
        ability![W, (2, 1, _1)];
        ability![E, (1, 0, _1), (1, 1, _2)];
        ability![R, (0, 0, _1)];
    }
}
