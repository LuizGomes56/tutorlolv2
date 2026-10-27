use super::*;

impl Generator for LudensEcho {
    fn generate(&mut self) -> MayFail {
        // Echo: damage of one orb, and the damage to the primary target when no
        // other enemy is nearby to consume the remaining stacks
        let total = self.render(self.total(Passive)?)?;
        self.damage_type(Magic)
            .min(Passive)?
            .set_max(total)
            .end()
    }
}
