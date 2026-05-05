use super::*;

impl Generator for SolsticeSleigh {
    fn generate(&mut self) -> MayFail {
        self.min(Active).end()
    }
}
