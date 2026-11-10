use super::*;

impl Generator for RunaansHurricane {
    fn generate(&mut self) -> MayFail {
        // Wind's Fury: damage of each bolt
        self.damage_type(Physical).min(Passive)?.end()
    }
}
