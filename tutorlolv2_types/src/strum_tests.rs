use crate::*;
use std::string::ToString;
use strum::{IntoEnumIterator, VariantNames};

#[test]
fn statname_parsing_preserves_aliases_and_errors() {
    for (text, expected) in [
        ("ah", StatName::AbilityHaste),
        ("hp", StatName::Health),
        ("mr", StatName::MagicResist),
        ("ap", StatName::AbilityPower),
        ("mana", StatName::Mana),
        ("ms", StatName::MoveSpeedPercent),
        ("hsp", StatName::HealAndShieldPower),
        ("mp5", StatName::BaseManaRegen),
        ("armor", StatName::Armor),
        ("msflat", StatName::MoveSpeed),
        ("crit", StatName::CritChance),
        ("ad", StatName::AttackDamage),
        ("armpen", StatName::ArmorPenetration),
        ("lethality", StatName::Lethality),
        ("as", StatName::AttackSpeed),
        ("lifesteal", StatName::LifeSteal),
        ("mpen", StatName::MagicPenetrationPercent),
        ("mpenflat", StatName::MagicPenetration),
        ("gp10", StatName::GoldPer10Seconds),
        ("hp5", StatName::BaseHealthRegen),
        ("tenacity", StatName::Tenacity),
        ("spec", StatName::AdaptiveForce),
        ("omnivamp", StatName::Omnivamp),
        ("hp5flat", StatName::BaseHealthRegen),
        ("critdamage", StatName::CritDamage),
    ] {
        assert_eq!(text.parse::<StatName>(), Ok(expected));
    }
    for text in ["", "invalid", "mElEe", "physical", "top"] {
        assert_eq!(
            text.parse::<StatName>(),
            Err("Unknown stat in StatName::from_str")
        );
    }
}

#[test]
fn attacktype_parsing_preserves_aliases_and_errors() {
    for (text, expected) in [
        ("Melee", AttackType::Melee),
        ("MELEE", AttackType::Melee),
        ("Ranged", AttackType::Ranged),
        ("RANGED", AttackType::Ranged),
    ] {
        assert_eq!(text.parse::<AttackType>(), Ok(expected));
    }
    for text in ["", "invalid", "mElEe", "physical", "top"] {
        assert_eq!(
            text.parse::<AttackType>(),
            Err("No matches when calling AttackType::from_str")
        );
    }
}

#[test]
fn adaptivetype_parsing_preserves_aliases_and_errors() {
    for (text, expected) in [
        ("Magic", AdaptiveType::Magic),
        ("MAGIC_DAMAGE", AdaptiveType::Magic),
        ("Physical", AdaptiveType::Physical),
        ("PHYSICAL_DAMAGE", AdaptiveType::Physical),
    ] {
        assert_eq!(text.parse::<AdaptiveType>(), Ok(expected));
    }
    for text in ["", "invalid", "mElEe", "physical", "top"] {
        assert_eq!(
            text.parse::<AdaptiveType>(),
            Err("No matches when calling AdaptiveType::from_str")
        );
    }
}

#[test]
fn position_parsing_preserves_aliases_and_errors() {
    for (text, expected) in [
        ("Top", Position::Top),
        ("TOP", Position::Top),
        ("Jungle", Position::Jungle),
        ("JUNGLE", Position::Jungle),
        ("Middle", Position::Middle),
        ("MIDDLE", Position::Middle),
        ("Bottom", Position::Bottom),
        ("BOTTOM", Position::Bottom),
        ("Support", Position::Support),
        ("SUPPORT", Position::Support),
    ] {
        assert_eq!(text.parse::<Position>(), Ok(expected));
    }
    for text in ["", "invalid", "mElEe", "physical", "top"] {
        assert_eq!(
            text.parse::<Position>(),
            Err("No matches when calling Position::from_str")
        );
    }
}

#[test]
fn display_and_variant_names_stay_unchanged() {
    for (value, name) in [(AttackType::Melee, "Melee"), (AttackType::Ranged, "Ranged")] {
        assert_eq!(value.to_string(), name);
    }
    assert_eq!(AttackType::VARIANTS, &["Melee", "Ranged"]);
    assert_eq!(AdaptiveType::VARIANTS, &["Physical", "Magic"]);
    assert_eq!(
        Position::VARIANTS,
        &["Top", "Jungle", "Middle", "Bottom", "Support"]
    );
    assert_eq!(AdaptiveType::Physical.to_string(), "Physical");
    assert_eq!(AdaptiveType::Magic.to_string(), "Magic");
    assert_eq!(Position::Bottom.to_string(), "Bottom");
}

#[test]
fn context_variables_parse_both_spellings_but_not_expressions() {
    for value in CtxVar::iter() {
        let pascal = std::format!("{value:?}");
        assert_eq!(pascal.parse::<CtxVar>(), Ok(value));
        assert_eq!(value.to_snake_case().parse::<CtxVar>(), Ok(value));
        assert_eq!(
            value.as_var().parse::<CtxVar>(),
            Err("CtxVar::from_str: Invalid variable provided")
        );
        assert_eq!(value.to_string(), value.as_var());
    }
}

#[test]
fn ability_name_conversion_accepts_the_last_variant() {
    assert_eq!(
        AbilityName::from_repr(AbilityName::MonsterMax as u8),
        Some(AbilityName::MonsterMax)
    );
}

#[test]
fn numeric_conversions_accept_exactly_the_defined_discriminants() {
    for byte in 0..=u8::MAX {
        assert_eq!(
            StatName::from_repr(byte),
            StatName::iter().find(|value| *value as u8 == byte)
        );
        assert_eq!(
            AbilityName::from_repr(byte),
            AbilityName::iter().find(|value| *value as u8 == byte)
        );
        assert_eq!(
            AttackType::from_repr(byte),
            AttackType::iter().find(|value| *value as u8 == byte)
        );
        assert_eq!(
            DamageIndex::from_repr(byte),
            DamageIndex::iter().find(|value| *value as u8 == byte)
        );
    }
}

#[test]
fn numeric_conversions_remain_const() {
    const LAST_STAT: Option<StatName> = StatName::from_repr(23);
    const INVALID_STAT: Option<StatName> = StatName::from_repr(24);
    const LAST_ABILITY: Option<AbilityName> = AbilityName::from_repr(AbilityName::MonsterMax as u8);
    const RANGED: Option<AttackType> = AttackType::from_repr(1);
    const MAX: Option<DamageIndex> = DamageIndex::from_repr(1);
    assert_eq!(LAST_STAT, Some(StatName::Tenacity));
    assert_eq!(INVALID_STAT, None);
    assert_eq!(LAST_ABILITY, Some(AbilityName::MonsterMax));
    assert_eq!(RANGED, Some(AttackType::Ranged));
    assert_eq!(MAX, Some(DamageIndex::Max));
}

#[test]
fn ability_discriminants_are_available_without_bake() {
    for (ability, expected) in [
        (AbilityId::P(AbilityName::Void), "P"),
        (AbilityId::Q(AbilityName::_1Min), "Q1min"),
        (AbilityId::E(AbilityName::Max), "Emax"),
    ] {
        assert_eq!(ability.discriminant(), expected);
    }
}
