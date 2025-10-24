use super::*;

impl Generator<Champion> for Zyra {
    #[generator_v2]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1)];
        ability![E, (0, 0, _1)];
        ability![R, (0, 0, _1)];
    }
}
