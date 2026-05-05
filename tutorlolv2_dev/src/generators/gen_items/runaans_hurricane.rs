use super::*;

impl Generator for RunaansHurricane {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.damage_type(Physical).end()
    }
}
