use super::*;

impl Generator for RanduinsOmen {
    fn generate(&mut self) -> MayFail {
        self.min(Active).min(Passive).end()
    }
}
