use super::*;

impl Generator for StaffOfFlowingWater {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
