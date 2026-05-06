use super::*;

impl Generator for RunaansHurricane {
    fn generate(&mut self) -> MayFail {
        self.damage_type(Physical).min(Passive)?.end()
    }
}
