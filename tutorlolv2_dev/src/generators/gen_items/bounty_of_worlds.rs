use super::*;

impl Generator for BountyOfWorlds {
    fn generate(&mut self) -> MayFail {
        self.min(Active).min(Passive).end()
    }
}
