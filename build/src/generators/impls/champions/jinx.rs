use super::*;

impl Generator for Jinx {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Void)])
            .modify(Q(Void), |_| f![1.1 * AttackDamage])?
            .ability(Key::W, [(0, Void) /* Physical Damage */])
            .ability(Key::E, [(0, Void) /* Magic Damage */])
            .ability(
                Key::R,
                [
                    (0, Max),   /* Maximum Physical Damage */
                    (1, _1Max), /* Maximum Secondary Damage */
                    (2, Min),   /* Minimum Physical Damage */
                    (3, _1Min), /* Minimum Secondary Damage */
                ],
            )
            .combo([
                Ability(E(Void)),
                Attack,
                Ability(W(Void)),
                Ability(R(Max)),
                Attack,
                Attack,
            ])?
            .combo([
                Ability(R(Max)),
                Ability(E(Void)),
                Ability(W(Void)),
                Attack,
                Attack,
                Attack,
            ])?
            .combo([
                Attack,
                Ability(W(Void)),
                Attack,
                Attack,
                Ability(E(Void)),
                Ability(Q(Void)),
                Attack,
                Attack,
                Attack,
            ])?
            .combo([
                Ability(W(Void)),
                Ability(E(Void)),
                Ability(Q(Void)),
                Attack,
                Attack,
                Attack,
            ])?
            .combo([Ability(E(Void)), Ability(W(Void)), Attack, Attack, Attack])?
            .combo([
                Ability(E(Void)),
                Attack,
                Attack,
                Attack,
                Ability(W(Void)),
                Ability(Q(Void)),
                Attack,
                Attack,
                Attack,
            ])?
            .end()
    }
}
