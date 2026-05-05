use super::*;

impl Generator for ZombieWard {
    fn generate(&mut self) -> MayFail {
        self.min(0) /* Description 1 */
            .min(1) /* Description 2 */
            .min(2) /* Description 3 */
            .min(3) /* Passive */
            .end()
    }
}
