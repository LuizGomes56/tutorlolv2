use super::*;

impl Generator<Champion> for Ekko {
    #[champion_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1), (1, 0, _2), (1, 1, _3)];
        ability![E, (0, 0, _1)];
        ability![R, (0, 1, _1)];
    }
}
