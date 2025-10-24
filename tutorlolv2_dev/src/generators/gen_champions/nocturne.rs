use super::*;

impl Generator<Champion> for Nocturne {
    #[champion_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1), (1, 0, _2)];
        ability![E, (0, 0, _1), (0, 1, _2)];
        ability![R, (2, 0, _1)];
    }
}
