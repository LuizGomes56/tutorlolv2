use wiki::rules::{AramMayhem, ByLevel, Drakes, MoveSpeedCaps, Shards, Turret};

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
| [[File:Rune shard Movement Speed.png|30px|link=]]<br />{{as|{{fd|2.5}}% '''bonus''' movement speed}}
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
            move_speed: 2.5,
        }
    );
}

#[test]
fn movement_speed_soft_caps_are_read_from_their_sentences() {
    let src = "\
When the raw movement speed is greater than 415, there are two soft caps applied:
* The raw speed between 415 and 490 gets multiplied by 80%.
* The raw speed over 490 gets multiplied by 50%.
When the raw movement speed is less than 220 another two soft caps are used:
* If the raw speed is below 220, final speed is increased to {{tt|110|220*(1-0.5)}}{{plus}}(Raw MS){{times}}{{fd|0.5}}.
";

    assert_eq!(
        MoveSpeedCaps::from_page(src).unwrap(),
        MoveSpeedCaps {
            first_cap: 415.0,
            first_ratio: 0.8,
            second_cap: 490.0,
            second_ratio: 0.5,
            lower_cap: 220.0,
            lower_floor: 110.0,
            lower_ratio: 0.5,
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

#[test]
fn turret_reads_the_outer_turret_and_bulwark_with_one_enemy_nearby() {
    let src = "\
{{Infobox turret|
|name         = Outer turret
|armor        = 60
|magicresist  = 60
}}
{{Infobox turret|
|name         = Inner turret
|armor        = 70
|magicresist  = 70
}}
* {{sbc|Unique Passive - Bulwark:}} Each stack grants the turret {{pp|30;30 to 50|0 to 5 by 1|type=number of nearby enemy champions}} {{sti|{{as|'''bonus''' armor}}}}, up to a maximum of {{pp|30*4;30*4 to 50*4|0 to 5 by 1|type=number of nearby enemy champions}} '''bonus''' resistances at 4 stacks.
== Damaging turrets ==
Unmodified basic attacks' damage against turrets is equal to {{as|100% '''base''' AD}} {{as|(+ 100% '''bonus''' AD)}} {{as|(+ 60% AP)}}.
";

    assert_eq!(
        Turret::from_page(src).unwrap(),
        Turret {
            resists: 60.0,
            ap_ratio: 0.6,
            bulwark_resists_per_stack: 30.0,
            bulwark_max_stacks: 4,
        }
    );
}
