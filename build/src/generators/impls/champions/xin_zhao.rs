use super::*;

impl Generator for XinZhao {
    fn generate(&mut self) -> MayFail {
        // 15% / 30% / 45% / 60% AD (+ 5% / 10% / 15% / 20% AP) at levels 1 / 6 / 11 / 16
        let tier = stringify!(match ctx.level {
            ..6 => 1,
            6..11 => 2,
            11..16 => 3,
            16.. => 4,
        });

        self.ability(Key::P, [(0, Void) /* Innate */])
            .modify(P(Void), |_| {
                f![(tier) * (0.15 * AttackDamage + 0.05 * AbilityPower)]
            })?
            .comment(P(Void), "Determination third stack bonus damage")?
            .ability(
                Key::Q,
                [
                    (0, Min), /* Bonus Physical Damage */
                    (1, Max), /* Total Bonus Physical Damage */
                ],
            )
            .ability(
                Key::W,
                [
                    (1, Min), /* Slash Total Physical Damage */
                    (3, Max), /* Total Physical Damage */
                    (2, _1),  /* Thrust Physical Damage */
                ],
            )
            .ability(Key::E, [(1, Void) /* Magic Damage */])
            .ability(Key::R, [(0, Void) /* Physical Damage */])
            .combo([
                Ability(R(Void)),
                Ability(E(Void)),
                Ability(Q(Max)),
                Attack,
                Attack,
                Attack,
                Ability(W(Max)),
                Attack,
            ])?
            .combo([Ability(E(Void)), Attack, Ability(R(Void)), Attack])?
            .combo([
                Ability(E(Void)),
                Attack,
                Ability(Q(Max)),
                Attack,
                Attack,
                Attack,
                Ability(W(Max)),
                Attack,
                Ability(R(Void)),
                Attack,
            ])?
            .combo([
                Attack,
                Ability(Q(Max)),
                Attack,
                Attack,
                Attack,
                Ability(E(Void)),
                Attack,
                Ability(W(Max)),
                Attack,
            ])?
            .combo([
                Ability(E(Void)),
                Attack,
                Ability(W(Max)),
                Attack,
                Ability(Q(Max)),
                Attack,
                Attack,
                Attack,
            ])?
            .combo([
                Ability(E(Void)),
                Ability(Q(Max)),
                Attack,
                Attack,
                Attack,
                Ability(W(Max)),
                Attack,
            ])?
            .end()
    }
}
