use super::*;

impl Generator for Locke {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Innate", 0, P(Min))?
            .comment(P(Min), "Silver Stake on-hit damage")
            .derive(P(Max), P(Min), |p| f![2 * (p)])?
            .comment(
                P(Max),
                "Silver Stake on-hit damage against low health targets",
            )
            .take("Magic Damage per Nail", Q(Min))?
            .take("Maximum Total Magic Damage", Q(Max))?
            .take("Dash Magic Damage", E(Min))?
            .take("Total Magic Damage", E(Max))?
            .take("Blink Magic Damage", E(_1))?
            .take("Magic Damage", R(Void))?
            .damage_type(R(Void), Magic)
            .end()
    }
}
