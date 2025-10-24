use super::*;

impl Generator<Champion> for Zyra {
    #[champion_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1)];
        ability![E, (0, 0, _1)];
        ability![R, (0, 0, _1)];
    }
}
