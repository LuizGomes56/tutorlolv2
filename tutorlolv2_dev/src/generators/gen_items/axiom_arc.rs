use super::*;

impl Generator for AxiomArc {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
