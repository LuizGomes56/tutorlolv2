use super::*;

impl Generator for Zed {
    fn generate(&mut self) -> MayFail {
        // 5% / 7.5% / 10% at levels 1 / 7 / 17
        let ratio = stringify!(match ctx.level {
            ..7 => 0.05,
            7..17 => 0.075,
            17.. => 0.1,
        });

        self.ability(Key::P, [(2, Void) /* Innate */])
            .modify(P(Void), |_| f![(ratio) * EnemyMaxHealth])?
            .ability(
                Key::Q,
                [
                    (0, Max), /* Physical Damage */
                    (1, Min), /* Reduced Damage */
                ],
            )
            .ability(Key::E, [(1, Void) /* Physical Damage */])
            .ability(Key::R, [(0, Void) /* Physical Damage */])
            .combo([
                Ability(R(Void)),
                Ability(E(Void)),
                Ability(Q(Max)),
                Ability(R(Void)),
                Attack,
            ])?
            .combo([
                Ability(R(Void)),
                Ability(E(Void)),
                Ability(Q(Max)),
                Ability(R(Void)),
                Attack,
                Ability(E(Void)),
            ])?
            .combo([
                Ability(R(Void)),
                Ability(E(Void)),
                Attack,
                Ability(R(Void)),
                Ability(Q(Max)),
                Attack,
                Ability(E(Void)),
            ])?
            .combo([
                Ability(R(Void)),
                Attack,
                Ability(Q(Max)),
                Ability(E(Void)),
                Attack,
                Ability(R(Void)),
            ])?
            .combo([
                Ability(Q(Max)),
                Ability(R(Void)),
                Ability(E(Void)),
                Attack,
                Attack,
            ])?
            .combo([
                Ability(R(Void)),
                Ability(Q(Max)),
                Ability(E(Void)),
                Attack,
                Ability(R(Void)),
            ])?
            .combo([
                Ability(E(Void)),
                Ability(R(Void)),
                Ability(Q(Max)),
                Attack,
                Ability(R(Void)),
            ])?
            .combo([Ability(E(Void)), Attack, Ability(Q(Max))])?
            .combo([
                Ability(E(Void)),
                Ability(Q(Max)),
                Ability(R(Void)),
                Ability(E(Void)),
                Attack,
            ])?
            .combo([
                Ability(R(Void)),
                Ability(E(Void)),
                Attack,
                Ability(Q(Max)),
                Attack,
                Ability(R(Void)),
            ])?
            .combo([
                Ability(E(Void)),
                Ability(Q(Max)),
                Attack,
                Attack,
                Ability(R(Void)),
                Attack,
                Attack,
                Ability(E(Void)),
            ])?
            .combo([Ability(E(Void)), Ability(Q(Max)), Attack])?
            .combo([Ability(Q(Max)), Ability(E(Void)), Attack])?
            .combo([Ability(E(Void)), Ability(Q(Max))])?
            .end()
    }
}
