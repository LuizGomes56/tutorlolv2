use super::*;

impl Generator for SeryldasGrudge {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
