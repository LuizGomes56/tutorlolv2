use wiki::rules::{AramMayhem, ByLevel, Drakes, Shards};

#[test]
fn drake_buffs_read_the_ratio_of_one_stack() {
    let src = "|description  = Gain {{pp|key=%|type=stacks granted by [[File: Infernal DrakeSquare.png|20px|link=]] '''Infernal Drake'''|3 to 3*4 for 4}} {{sti|{{as|attack damage}}}} and {{sti|{{as|ability power}}}}.";

    assert_eq!(Drakes::per_stack(src).unwrap(), 0.03);
}

#[test]
fn drake_buffs_that_do_not_stack_linearly_fail() {
    let src = "Gain {{pp|key=%|type=stacks|3 to 5*4 for 4}} {{sti|{{as|armor}}}}.";

    assert!(Drakes::per_stack(src).is_err());
}

#[test]
fn shards_are_read_from_their_section_only() {
    let src = "\
=== Shards ===
| [[File:Rune shard Adaptive Force.png|30px|link=]]<br />{{adaptive|9}}
| [[File:Rune shard Attack Speed.png|30px|link=]]<br />{{as|10% '''bonus''' attack speed}}
| [[File:Rune shard Health Scaling.png|30px|link=]]<br />{{pp|10 * x for 20|color=health}} {{as|'''bonus''' health}}
| [[File:Rune shard Health.png|30px|link=]]<br />{{as|65 '''bonus''' health}}
== Patch history ==
* [[File:Rune shard Health.png|20px|link=]]<br />{{as|15 '''bonus''' health}}
";

    assert_eq!(
        Shards::from_page(src).unwrap(),
        Shards {
            adaptive_force: 9.0,
            attack_speed: 10.0,
            health: 65.0,
            health_scaling_per_level: 10.0,
        }
    );
}

#[test]
fn aram_mayhem_reads_its_global_stat_changes() {
    let src = "\
=== Global stat changes ===
* Every {{as|1% critical strike chance}} in excess of {{as|100%|critical strike chance}} grants {{adaptive|0.75}}.
** {{rd|pp=true|20 to 600|20 to 400|levels=1 to 18|label=Health|color=hp|formula1=20 + ((600 - 20) / 17) * (level - 1)}} {{sti|health|{{as|'''base''' health}}}}
** {{rd|15|{{pp|2 to 25|1 to 18|label=Resistances}}}} {{sti|{{as|'''bonus''' armor}}}}

=== Combo Breaker ===
";

    assert_eq!(
        AramMayhem::from_page(src).unwrap(),
        AramMayhem {
            excess_crit_adaptive_force: 0.75,
            melee_base_health: ByLevel {
                min: 20.0,
                max: 600.0
            },
            ranged_base_health: ByLevel {
                min: 20.0,
                max: 400.0
            },
            melee_bonus_resists: 15.0,
            ranged_bonus_resists: ByLevel {
                min: 2.0,
                max: 25.0
            },
        }
    );
}
