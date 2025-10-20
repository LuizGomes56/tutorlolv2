use super::*;

impl Generator for Cassiopeia {
    #[generator_v2]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1), (0, 1, _2)];
        ability![W, (1, 0, _1), (1, 2, _2)];
        ability![E, (1, 0, _1), (1, 3, _2)];
        ability![R, (0, 0, _1)];
    }
}
