use super::*;

impl Generator<Champion> for Volibear {
    #[generator_v2]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (1, 0, _1)];
        ability![W, (0, 0, _1)];
        ability![E, (1, 0, _1)];
        ability![R, (4, 0, _1)];
    }
}
