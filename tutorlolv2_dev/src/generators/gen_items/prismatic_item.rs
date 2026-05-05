use super::*;

impl Generator for PrismaticItem {
    fn generate(&mut self) -> MayFail {
        self.end()
    }
}
