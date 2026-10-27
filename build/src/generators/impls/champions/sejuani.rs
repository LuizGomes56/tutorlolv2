use super::*;

impl Generator for Sejuani {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(2, Void) /* Innate - Icebreaker */])
            .ability(Key::Q, [(0, Void) /* Magic Damage */])
            .ability(
                Key::W,
                [
                    (0, Min), /* Physical Damage */
                    (1, Max), /* Total Physical Damage */
                ],
            )
            .ability(Key::E, [(0, Void) /* Magic Damage */])
            .ability(
                Key::R,
                [
                    (0, Max), /* Increased Damage */
                    (1, Min), /* Magic Damage */
                ],
            )
            .combo([
                Attack,
                Ability(W(Max)),
                Attack,
                Ability(Q(Void)),
                Ability(E(Void)),
                Attack,
                Ability(R(Max)),
                Attack,
            ])?
            .combo([Ability(Q(Void)), Ability(R(Max)), Ability(W(Max)), Attack])?
            .combo([
                Ability(R(Max)),
                Ability(Q(Void)),
                Ability(W(Max)),
                Attack,
                Attack,
            ])?
            .combo([
                Ability(Q(Void)),
                Ability(R(Max)),
                Ability(W(Max)),
                Attack,
                Attack,
            ])?
            .combo([Ability(R(Max)), Ability(W(Max)), Ability(Q(Void)), Attack])?
            .combo([
                Ability(E(Void)),
                Ability(Q(Void)),
                Attack,
                Ability(R(Max)),
                Attack,
                Ability(W(Max)),
                Attack,
            ])?
            .combo([
                Ability(Q(Void)),
                Ability(R(Max)),
                Attack,
                Ability(W(Max)),
                Attack,
            ])?
            .combo([
                Ability(Q(Void)),
                Attack,
                Ability(W(Max)),
                Attack,
                Ability(E(Void)),
                Attack,
                Ability(R(Max)),
                Attack,
            ])?
            .combo([
                Ability(W(Max)),
                Attack,
                Ability(Q(Void)),
                Attack,
                Ability(E(Void)),
                Attack,
            ])?
            .combo([
                Ability(Q(Void)),
                Ability(W(Max)),
                Attack,
                Attack,
                Ability(E(Void)),
                Attack,
                Ability(R(Max)),
                Attack,
            ])?
            .combo([
                Ability(Q(Void)),
                Attack,
                Ability(W(Max)),
                Attack,
                Ability(E(Void)),
                Attack,
            ])?
            .end()
    }
}
