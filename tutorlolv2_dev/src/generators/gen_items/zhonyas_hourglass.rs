use super::*;

impl Generator for ZhonyasHourglass {
    fn generate(&mut self) -> MayFail {
        self.min(Active)?.end()
    }
}
