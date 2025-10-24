use super::*;

impl Generator<Champion> for Garen {
    #[generator_v2]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (1, 0, _1)];
        ability![W, (0, 0, _1)];
        ability![E, (0, 0, _1), (3, 0, _2)];
        ability![R, (0, 0, _1)];
    }
}
