use super::*;

impl Generator for ArchangelsStaff {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
