use super::*;

impl Generator for Jinx {
    fn generate(&mut self) -> MayFail {
        let q_scaling = self
            .get_meraki_ability(Key::Q, 0)
            .effects
            .first()
            .and_then(|effect| effect.description.capture_percent(0).ok())
            .unwrap_or(1.1);

        self.ability_raw::<5>(Q(Void), |_| q_scaling.times(AttackDamage))?
            .ability(Key::W, [(0, 0, Void)])
            .ability(Key::E, [(0, 0, Void)])
            .ability(
                Key::R,
                [
                    // Surroundings
                    (1, 0, _1Max),
                    (1, 1, _1Min),
                    // Primary Target
                    (2, 0, Max),
                    (2, 1, Min),
                ],
            )
            .attr(AreaOnhitMax, [Q(Void)])?
            .attr(Area, [Q(Void), E(Void), R(_1Max), R(_1Min), R(Max), R(Min)])?
            .combo([
                Ability(E(Void)),
                Attack,
                Ability(R(Min)),
                Attack,
                Ability(W(Void)),
            ])?
            .progress(Stable)
            .end()
    }
}
