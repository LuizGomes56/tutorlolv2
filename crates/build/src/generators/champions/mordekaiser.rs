use super::*;

impl Generator for Mordekaiser {
    fn generate(&mut self) -> MayFail {
        let deaths_grasp_magic_penetration = self
            .value("Magic Penetration", Key::E)?
            .default_values()
            .ok_or("[Mordekaiser] Magic Penetration has no values")?;

        self.constant("DEATHS_GRASP_MAGIC_PENETRATION", deaths_grasp_magic_penetration)?;

        let isolated = self.value("Isolated Damage Increase", Key::Q)?;

        self.take_nth("Darkness Rise", 1, P(Void))?
            .comment(P(Void), "Darkness Rise magic damage per second")
            .take("Magic Damage", Q(Min))?
            .derive(Q(Max), Q(Min), |q| f![q * (1 + isolated / 100)])?
            .comment(Q(Max), "Isolated target")
            .take("Magic Damage", E(Void))?
            .combos([
                &[Ability(E(Void)), Ability(Q(Max)), Attack, Attack],
                &[Ability(Q(Max)), Attack, Attack, Ability(E(Void)), Attack],
                &[Ability(E(Void)), Attack, Ability(Q(Max)), Attack],
                &[Attack, Ability(Q(Max)), Attack],
                &[Ability(E(Void)), Ability(Q(Max)), Attack],
            ])?
            .end()
    }
}
