use super::*;pub static RUNE_ID_TO_RIOT_ID:[u32;72]=[9994,8233,9101,9990,8214,8439,8410,9992,8224,8345,8473,8321,8234,8126,8229,8429,8010,8347,8014,8017,8128,8141,8446,8112,8120,8369,8021,8463,8236,8136,8351,8437,8140,8465,9923,9993,9991,8306,8316,8299,9104,9103,9105,8008,8304,8226,9996,8275,8451,8230,8124,8009,8005,8105,8453,8237,8444,8401,8137,8143,8139,9995,8352,8210,8135,8313,9111,8106,8242,8360,8232,8138];#[derive(Debug,Copy,Clone,Ord,Eq,PartialOrd,PartialEq,Decode,Encode)]#[repr(u8)]pub enum RuneId {AbilityHaste,AbsoluteFocus,AbsorbLife,AdaptiveForce,Aery,Aftershock,ApproachVelocity,AttackSpeed,AxiomArcanist,BiscuitDelivery,BonePlating,CashBack,Celerity,CheapShot,Comet,Conditioning,Conqueror,CosmicInsight,CoupdeGrace,CutDown,DarkHarvest,DeepWard,Demolish,Electrocute,EyeballCollection,FirstStrike,FleetFootwork,FontofLife,GatheringStorm,GhostPoro,GlacialAugment,Grasp,GrislyMementos,Guardian,HailofBlades,Health,HealthScaling,HextechFlashtraption,JackOfAllTrades,LastStand,LegendAlacrity,LegendBloodline,LegendHaste,LethalTempo,MagicalFootwear,ManaflowBand,MoveSpeed,NimbusCloak,Overgrowth,PhaseRush,Predator,PresenceofMind,PressTheAttack,RelentlessHunter,Revitalize,Scorch,SecondWind,ShieldBash,SixthSense,SuddenImpact,TasteofBlood,TenacityandSlowResist,TimeWarpTonic,Transcendence,TreasureHunter,TripleTonic,Triumph,UltimateHunter,Unflinching,UnsealedSpellbook,Waterwalking,ZombieWard}
        impl RuneId {pub const fn to_riot_id(&self)->u32{RUNE_ID_TO_RIOT_ID[*self as usize]}
        pub const fn from_riot_id(id:u32)->Self{match id{9994=>Self::AbilityHaste,8233=>Self::AbsoluteFocus,9101=>Self::AbsorbLife,9990=>Self::AdaptiveForce,8214=>Self::Aery,8439=>Self::Aftershock,8410=>Self::ApproachVelocity,9992=>Self::AttackSpeed,8224=>Self::AxiomArcanist,8345=>Self::BiscuitDelivery,8473=>Self::BonePlating,8321=>Self::CashBack,8234=>Self::Celerity,8126=>Self::CheapShot,8229=>Self::Comet,8429=>Self::Conditioning,8010=>Self::Conqueror,8347=>Self::CosmicInsight,8014=>Self::CoupdeGrace,8017=>Self::CutDown,8128=>Self::DarkHarvest,8141=>Self::DeepWard,8446=>Self::Demolish,8112=>Self::Electrocute,8120=>Self::EyeballCollection,8369=>Self::FirstStrike,8021=>Self::FleetFootwork,8463=>Self::FontofLife,8236=>Self::GatheringStorm,8136=>Self::GhostPoro,8351=>Self::GlacialAugment,8437=>Self::Grasp,8140=>Self::GrislyMementos,8465=>Self::Guardian,9923=>Self::HailofBlades,9993=>Self::Health,9991=>Self::HealthScaling,8306=>Self::HextechFlashtraption,8316=>Self::JackOfAllTrades,8299=>Self::LastStand,9104=>Self::LegendAlacrity,9103=>Self::LegendBloodline,9105=>Self::LegendHaste,8008=>Self::LethalTempo,8304=>Self::MagicalFootwear,8226=>Self::ManaflowBand,9996=>Self::MoveSpeed,8275=>Self::NimbusCloak,8451=>Self::Overgrowth,8230=>Self::PhaseRush,8124=>Self::Predator,8009=>Self::PresenceofMind,8005=>Self::PressTheAttack,8105=>Self::RelentlessHunter,8453=>Self::Revitalize,8237=>Self::Scorch,8444=>Self::SecondWind,8401=>Self::ShieldBash,8137=>Self::SixthSense,8143=>Self::SuddenImpact,8139=>Self::TasteofBlood,9995=>Self::TenacityandSlowResist,8352=>Self::TimeWarpTonic,8210=>Self::Transcendence,8135=>Self::TreasureHunter,8313=>Self::TripleTonic,9111=>Self::Triumph,8106=>Self::UltimateHunter,8242=>Self::Unflinching,8360=>Self::UnsealedSpellbook,8232=>Self::Waterwalking,8138=>Self::ZombieWard,_=>Self::AbilityHaste}}}pub static INTERNAL_RUNES:[&CachedRune;72]=[&ABILITY_HASTE_9994,&ABSOLUTE_FOCUS_8233,&ABSORB_LIFE_9101,&ADAPTIVE_FORCE_9990,&AERY_8214,&AFTERSHOCK_8439,&APPROACH_VELOCITY_8410,&ATTACK_SPEED_9992,&AXIOM_ARCANIST_8224,&BISCUIT_DELIVERY_8345,&BONE_PLATING_8473,&CASH_BACK_8321,&CELERITY_8234,&CHEAP_SHOT_8126,&COMET_8229,&CONDITIONING_8429,&CONQUEROR_8010,&COSMIC_INSIGHT_8347,&COUP_DE_GRACE_8014,&CUT_DOWN_8017,&DARK_HARVEST_8128,&DEEP_WARD_8141,&DEMOLISH_8446,&ELECTROCUTE_8112,&EYEBALL_COLLECTION_8120,&FIRST_STRIKE_8369,&FLEET_FOOTWORK_8021,&FONT_OF_LIFE_8463,&GATHERING_STORM_8236,&GHOST_PORO_8136,&GLACIAL_AUGMENT_8351,&GRASP_8437,&GRISLY_MEMENTOS_8140,&GUARDIAN_8465,&HAIL_OF_BLADES_9923,&HEALTH_9993,&HEALTH_SCALING_9991,&HEXTECH_FLASHTRAPTION_8306,&JACK_OF_ALL_TRADES_8316,&LAST_STAND_8299,&LEGEND_ALACRITY_9104,&LEGEND_BLOODLINE_9103,&LEGEND_HASTE_9105,&LETHAL_TEMPO_8008,&MAGICAL_FOOTWEAR_8304,&MANAFLOW_BAND_8226,&MOVE_SPEED_9996,&NIMBUS_CLOAK_8275,&OVERGROWTH_8451,&PHASE_RUSH_8230,&PREDATOR_8124,&PRESENCE_OF_MIND_8009,&PRESS_THE_ATTACK_8005,&RELENTLESS_HUNTER_8105,&REVITALIZE_8453,&SCORCH_8237,&SECOND_WIND_8444,&SHIELD_BASH_8401,&SIXTH_SENSE_8137,&SUDDEN_IMPACT_8143,&TASTE_OF_BLOOD_8139,&TENACITYAND_SLOW_RESIST_9995,&TIME_WARP_TONIC_8352,&TRANSCENDENCE_8210,&TREASURE_HUNTER_8135,&TRIPLE_TONIC_8313,&TRIUMPH_9111,&ULTIMATE_HUNTER_8106,&UNFLINCHING_8242,&UNSEALED_SPELLBOOK_8360,&WATERWALKING_8232,&ZOMBIE_WARD_8138,];pub static ABILITY_HASTE_9994: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::AbilityHaste,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static ABSOLUTE_FOCUS_8233: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::AbsoluteFocus,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static ABSORB_LIFE_9101: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::AbsorbLife,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static ADAPTIVE_FORCE_9990: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::AdaptiveForce,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static AERY_8214: CachedRune = CachedRune {
                    damage_type: DamageType::Adaptative,
                    metadata: TypeMetadata {
                    kind: RuneId::Aery,
                    damage_type: DamageType::Adaptative,
                    attributes: Attrs::None
                },
                    melee_closure: |_| 0.0f32,
                    range_closure: |_| 0.0f32,
                };pub static AFTERSHOCK_8439: CachedRune = CachedRune {
                    damage_type: DamageType::Adaptative,
                    metadata: TypeMetadata {
                    kind: RuneId::Aftershock,
                    damage_type: DamageType::Adaptative,
                    attributes: Attrs::None
                },
                    melee_closure: |ctx| (25f32 + 95f32 / 17f32 * (ctx.level - 1f32) + 0.08f32 * ctx.bonus_health) * ctx.magic_multiplier,
                    range_closure: |ctx| (25f32 + 95f32 / 17f32 * (ctx.level - 1f32) + 0.08f32 * ctx.bonus_health) * ctx.magic_multiplier,
                };pub static APPROACH_VELOCITY_8410: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::ApproachVelocity,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static ATTACK_SPEED_9992: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::AttackSpeed,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static AXIOM_ARCANIST_8224: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::AxiomArcanist,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static BISCUIT_DELIVERY_8345: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::BiscuitDelivery,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static BONE_PLATING_8473: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::BonePlating,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static CASH_BACK_8321: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::CashBack,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static CELERITY_8234: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::Celerity,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static CHEAP_SHOT_8126: CachedRune = CachedRune {
                    damage_type: DamageType::True,
                    metadata: TypeMetadata {
                    kind: RuneId::CheapShot,
                    damage_type: DamageType::True,
                    attributes: Attrs::None
                },
                    melee_closure: |ctx| 10f32 + 35f32 / 17f32 * (ctx.level - 1f32),
                    range_closure: |ctx| 10f32 + 35f32 / 17f32 * (ctx.level - 1f32),
                };pub static COMET_8229: CachedRune = CachedRune {
                    damage_type: DamageType::Adaptative,
                    metadata: TypeMetadata {
                    kind: RuneId::Comet,
                    damage_type: DamageType::Adaptative,
                    attributes: Attrs::None
                },
                    melee_closure: |_| 0.0f32,
                    range_closure: |_| 0.0f32,
                };pub static CONDITIONING_8429: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::Conditioning,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static CONQUEROR_8010: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::Conqueror,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static COSMIC_INSIGHT_8347: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::CosmicInsight,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static COUP_DE_GRACE_8014: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::CoupdeGrace,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static CUT_DOWN_8017: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::CutDown,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static DARK_HARVEST_8128: CachedRune = CachedRune {
                    damage_type: DamageType::Adaptative,
                    metadata: TypeMetadata {
                    kind: RuneId::DarkHarvest,
                    damage_type: DamageType::Adaptative,
                    attributes: Attrs::None
                },
                    melee_closure: |_| 0.0f32,
                    range_closure: |_| 0.0f32,
                };pub static DEEP_WARD_8141: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::DeepWard,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static DEMOLISH_8446: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::Demolish,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static ELECTROCUTE_8112: CachedRune = CachedRune {
                    damage_type: DamageType::Adaptative,
                    metadata: TypeMetadata {
                    kind: RuneId::Electrocute,
                    damage_type: DamageType::Adaptative,
                    attributes: Attrs::None
                },
                    melee_closure: |_| 0.0f32,
                    range_closure: |_| 0.0f32,
                };pub static EYEBALL_COLLECTION_8120: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::EyeballCollection,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static FIRST_STRIKE_8369: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::FirstStrike,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static FLEET_FOOTWORK_8021: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::FleetFootwork,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static FONT_OF_LIFE_8463: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::FontofLife,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static GATHERING_STORM_8236: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::GatheringStorm,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static GHOST_PORO_8136: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::GhostPoro,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static GLACIAL_AUGMENT_8351: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::GlacialAugment,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static GRASP_8437: CachedRune = CachedRune {
                    damage_type: DamageType::Magic,
                    metadata: TypeMetadata {
                    kind: RuneId::Grasp,
                    damage_type: DamageType::Magic,
                    attributes: Attrs::None
                },
                    melee_closure: |ctx| 0.035f32 * ctx.max_health * ctx.magic_multiplier,
                    range_closure: |ctx| 0.021f32 * ctx.max_health * ctx.magic_multiplier,
                };pub static GRISLY_MEMENTOS_8140: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::GrislyMementos,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static GUARDIAN_8465: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::Guardian,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static HAIL_OF_BLADES_9923: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::HailofBlades,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static HEALTH_9993: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::Health,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static HEALTH_SCALING_9991: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::HealthScaling,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static HEXTECH_FLASHTRAPTION_8306: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::HextechFlashtraption,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static JACK_OF_ALL_TRADES_8316: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::JackOfAllTrades,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static LAST_STAND_8299: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::LastStand,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static LEGEND_ALACRITY_9104: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::LegendAlacrity,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static LEGEND_BLOODLINE_9103: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::LegendBloodline,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static LEGEND_HASTE_9105: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::LegendHaste,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static LETHAL_TEMPO_8008: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::LethalTempo,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static MAGICAL_FOOTWEAR_8304: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::MagicalFootwear,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static MANAFLOW_BAND_8226: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::ManaflowBand,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static MOVE_SPEED_9996: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::MoveSpeed,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static NIMBUS_CLOAK_8275: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::NimbusCloak,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static OVERGROWTH_8451: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::Overgrowth,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static PHASE_RUSH_8230: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::PhaseRush,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static PREDATOR_8124: CachedRune = CachedRune {
                    damage_type: DamageType::Adaptative,
                    metadata: TypeMetadata {
                    kind: RuneId::Predator,
                    damage_type: DamageType::Adaptative,
                    attributes: Attrs::None
                },
                    melee_closure: |_| 0.0f32,
                    range_closure: |_| 0.0f32,
                };pub static PRESENCE_OF_MIND_8009: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::PresenceofMind,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static PRESS_THE_ATTACK_8005: CachedRune = CachedRune {
                    damage_type: DamageType::Adaptative,
                    metadata: TypeMetadata {
                    kind: RuneId::PressTheAttack,
                    damage_type: DamageType::Adaptative,
                    attributes: Attrs::None
                },
                    melee_closure: |_| 0.0f32,
                    range_closure: |_| 0.0f32,
                };pub static RELENTLESS_HUNTER_8105: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::RelentlessHunter,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static REVITALIZE_8453: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::Revitalize,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static SCORCH_8237: CachedRune = CachedRune {
                    damage_type: DamageType::Magic,
                    metadata: TypeMetadata {
                    kind: RuneId::Scorch,
                    damage_type: DamageType::Magic,
                    attributes: Attrs::None
                },
                    melee_closure: |ctx| (20f32 + 20f32 / 17f32 * (ctx.level - 1f32)) * ctx.magic_multiplier,
                    range_closure: |ctx| (20f32 + 20f32 / 17f32 * (ctx.level - 1f32)) * ctx.magic_multiplier,
                };pub static SECOND_WIND_8444: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::SecondWind,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static SHIELD_BASH_8401: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::ShieldBash,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static SIXTH_SENSE_8137: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::SixthSense,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static SUDDEN_IMPACT_8143: CachedRune = CachedRune {
                    damage_type: DamageType::True,
                    metadata: TypeMetadata {
                    kind: RuneId::SuddenImpact,
                    damage_type: DamageType::True,
                    attributes: Attrs::None
                },
                    melee_closure: |ctx| 20f32 + 60f32 / 17f32 * (ctx.level - 1f32),
                    range_closure: |ctx| 20f32 + 60f32 / 17f32 * (ctx.level - 1f32),
                };pub static TASTE_OF_BLOOD_8139: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::TasteofBlood,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static TENACITYAND_SLOW_RESIST_9995: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::TenacityandSlowResist,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static TIME_WARP_TONIC_8352: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::TimeWarpTonic,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static TRANSCENDENCE_8210: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::Transcendence,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static TREASURE_HUNTER_8135: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::TreasureHunter,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static TRIPLE_TONIC_8313: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::TripleTonic,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static TRIUMPH_9111: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::Triumph,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static ULTIMATE_HUNTER_8106: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::UltimateHunter,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static UNFLINCHING_8242: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::Unflinching,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static UNSEALED_SPELLBOOK_8360: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::UnsealedSpellbook,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static WATERWALKING_8232: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::Waterwalking,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static ZOMBIE_WARD_8138: CachedRune = CachedRune {
                damage_type: DamageType::Unknown,
                metadata: TypeMetadata {
                kind: RuneId::ZombieWard,
                damage_type: DamageType::Unknown,
                attributes: Attrs::None
            },
                melee_closure: zero,
                range_closure: zero,    
            };pub static DAMAGING_RUNES:phf::Set<u32>=phf::phf_set!(8214u32,8439u32,8126u32,8229u32,8128u32,8112u32,8437u32,8124u32,8005u32,8237u32,8143u32);