use super::*;

impl Generator for Ekko {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(1, Void) /* Innate - Resonance */])
            .ability(
                Key::Q,
                [
                    (0, _1Min), /* Initial Magic Damage */
                    (1, _1Max), /* Return Magic Damage */
                    (3, Max),   /* Total Magic Damage */
                ],
            )
            .ability(Key::E, [(0, Void) /* Bonus Magic Damage */])
            .ability(Key::R, [(0, Void) /* Magic Damage */])
            .combo([
                Ability(E(Void)),
                Ability(Q(Max)),
                Ability(R(Void)),
                Ability(E(Void)),
                Attack,
                Attack,
            ])?
            .combo([
                Ability(E(Void)),
                Ability(E(Void)),
                Attack,
                Ability(Q(Max)),
                Attack,
                Attack,
            ])?
            .combo([
                Ability(R(Void)),
                Ability(E(Void)),
                Ability(E(Void)),
                Attack,
                Ability(Q(Max)),
                Attack,
            ])?
            .combo([
                Attack,
                Ability(Q(Max)),
                Ability(E(Void)),
                Ability(E(Void)),
                Attack,
            ])?
            .combo([
                Ability(E(Void)),
                Ability(Q(Max)),
                Ability(R(Void)),
                Ability(E(Void)),
                Attack,
                Attack,
                Attack,
            ])?
            .combo([
                Ability(E(Void)),
                Ability(Q(Max)),
                Ability(E(Void)),
                Attack,
                Attack,
            ])?
            .combo([
                Ability(Q(Max)),
                Ability(E(Void)),
                Ability(R(Void)),
                Ability(E(Void)),
                Attack,
            ])?
            .combo([
                Ability(E(Void)),
                Ability(E(Void)),
                Attack,
                Ability(Q(Max)),
                Attack,
                Attack,
                Attack,
                Attack,
            ])?
            .combo([
                Ability(Q(Max)),
                Ability(E(Void)),
                Ability(E(Void)),
                Attack,
                Attack,
            ])?
            .combo([
                Ability(E(Void)),
                Ability(Q(Max)),
                Ability(E(Void)),
                Attack,
                Attack,
                Attack,
            ])?
            .combo([
                Ability(E(Void)),
                Ability(E(Void)),
                Attack,
                Ability(Q(Max)),
                Attack,
            ])?
            .combo([
                Ability(Q(Max)),
                Attack,
                Ability(E(Void)),
                Ability(E(Void)),
                Attack,
                Attack,
                Attack,
                Attack,
            ])?
            .combo([Ability(Q(Max)), Ability(E(Void)), Ability(E(Void)), Attack])?
            .end()
    }
}
