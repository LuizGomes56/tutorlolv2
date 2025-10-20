use super::*;

impl Generator for Olaf {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1), (3, 0, _2), (3, 1, _3)];
        ability![E, (0, 0, _1)];
        ability![R, (0, 0, _1)];
    }
}
