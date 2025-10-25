use super::*;

impl Generator<Champion> for Braum {
    #[champion_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1)];
        ability![E, (0, 1, _1)];
        ability![R, (1, 0, _1)];
    }
}
