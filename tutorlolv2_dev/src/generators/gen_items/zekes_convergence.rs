use super::*;

impl Generator for ZekesConvergence {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
