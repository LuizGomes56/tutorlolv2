use super::*;#[derive(Debug,PartialEq,Ord,Eq,PartialOrd,Copy,Clone,Decode,Encode)]
        #[repr(u8)]pub enum ChampionId {Aatrox,Ahri,Akali,Akshan,Alistar,Ambessa,Amumu,Anivia,Annie,Aphelios,Ashe,AurelionSol,Aurora,Azir,Bard,Belveth,Blitzcrank,Brand,Braum,Briar,Caitlyn,Camille,Cassiopeia,Chogath,Corki,Darius,Diana,DrMundo,Draven,Ekko,Elise,Evelynn,Ezreal,Fiddlesticks,Fiora,Fizz,Galio,Gangplank,Garen,Gnar,Gragas,Graves,Gwen,Hecarim,Heimerdinger,Hwei,Illaoi,Irelia,Ivern,Janna,JarvanIV,Jax,Jayce,Jhin,Jinx,KSante,Kaisa,Kalista,Karma,Karthus,Kassadin,Katarina,Kayle,Kayn,Kennen,Khazix,Kindred,Kled,KogMaw,Leblanc,LeeSin,Leona,Lillia,Lissandra,Lucian,Lulu,Lux,Malphite,Malzahar,Maokai,MasterYi,Mel,Milio,MissFortune,MonkeyKing,Mordekaiser,Morgana,Naafiri,Nami,Nasus,Nautilus,Neeko,Nidalee,Nilah,Nocturne,Nunu,Olaf,Orianna,Ornn,Pantheon,Poppy,Pyke,Qiyana,Quinn,Rakan,Rammus,RekSai,Rell,Renata,Renekton,Rengar,Riven,Rumble,Ryze,Samira,Sejuani,Senna,Seraphine,Sett,Shaco,Shen,Shyvana,Singed,Sion,Sivir,Skarner,Smolder,Sona,Soraka,Swain,Sylas,Syndra,TahmKench,Taliyah,Talon,Taric,Teemo,Thresh,Tristana,Trundle,Tryndamere,TwistedFate,Twitch,Udyr,Urgot,Varus,Vayne,Veigar,Velkoz,Vex,Vi,Viego,Viktor,Vladimir,Volibear,Warwick,Xayah,Xerath,XinZhao,Yasuo,Yone,Yorick,Yunara,Yuumi,Zac,Zed,Zeri,Ziggs,Zilean,Zoe,Zyra}pub static AATROX: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::P(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_2), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_3), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_2Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_3Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::Void), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::Minion), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.level as u8 {1 => 0.04f32 + ctx.enemy_max_health,2 => 0.042352941176470586f32 + ctx.enemy_max_health,3 => 0.04470588235294118f32 + ctx.enemy_max_health,4 => 0.047058823529411764f32 + ctx.enemy_max_health,5 => 0.049411764705882356f32 + ctx.enemy_max_health,6 => 0.05176470588235294f32 + ctx.enemy_max_health,7 => 0.05411764705882353f32 + ctx.enemy_max_health,8 => 0.05647058823529412f32 + ctx.enemy_max_health,9 => 0.058823529411764705f32 + ctx.enemy_max_health,10 => 0.06117647058823529f32 + ctx.enemy_max_health,11 => 0.06352941176470589f32 + ctx.enemy_max_health,12 => 0.06588235294117648f32 + ctx.enemy_max_health,13 => 0.06823529411764706f32 + ctx.enemy_max_health,14 => 0.07058823529411765f32 + ctx.enemy_max_health,15 => 0.07294117647058823f32 + ctx.enemy_max_health,16 => 0.07529411764705882f32 + ctx.enemy_max_health,17 => 0.07764705882352942f32 + ctx.enemy_max_health,18 => 0.08f32 + ctx.enemy_max_health,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 10f32 + 0.6f32 * ctx.ad,2 => 25f32 + 0.675f32 * ctx.ad,3 => 40f32 + 0.75f32 * ctx.ad,4 => 55f32 + 0.825f32 * ctx.ad,5 => 70f32 + 0.9f32 * ctx.ad,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 12.5f32 + 0.75f32 * ctx.ad,2 => 31.25f32 + 0.84375f32 * ctx.ad,3 => 50f32 + 0.9375f32 * ctx.ad,4 => 68.75f32 + 1.03125f32 * ctx.ad,5 => 87.5f32 + 1.125f32 * ctx.ad,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 37.5f32 + 2.25f32 * ctx.ad,2 => 93.75f32 + 2.53125f32 * ctx.ad,3 => 150f32 + 2.8125f32 * ctx.ad,4 => 206.25f32 + 3.09375f32 * ctx.ad,5 => 262.5f32 + 3.375f32 * ctx.ad,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 10f32 + 0.6f32 * ctx.ad + (12.5f32 + 0.75f32 * ctx.ad) + (37.5f32 + 2.25f32 * ctx.ad),2 => 25f32 + 0.675f32 * ctx.ad + (31.25f32 + 0.84375f32 * ctx.ad) + (93.75f32 + 2.53125f32 * ctx.ad),3 => 40f32 + 0.75f32 * ctx.ad + (50f32 + 0.9375f32 * ctx.ad) + (150f32 + 2.8125f32 * ctx.ad),4 => 55f32 + 0.825f32 * ctx.ad + (68.75f32 + 1.03125f32 * ctx.ad) + (206.25f32 + 3.09375f32 * ctx.ad),5 => 70f32 + 0.9f32 * ctx.ad + (87.5f32 + 1.125f32 * ctx.ad) + (262.5f32 + 3.375f32 * ctx.ad),_ => 0.0 }},|ctx| { match ctx.q_level {1 => 17f32 + 1.02f32 * ctx.ad,2 => 42.5f32 + 1.1475f32 * ctx.ad,3 => 68f32 + 1.275f32 * ctx.ad,4 => 93.5f32 + 1.4025f32 * ctx.ad,5 => 119f32 + 1.53f32 * ctx.ad,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 21.25f32 + 1.275f32 * ctx.ad,2 => 53.125f32 + 1.434375f32 * ctx.ad,3 => 85f32 + 1.59375f32 * ctx.ad,4 => 116.875f32 + 1.753125f32 * ctx.ad,5 => 148.75f32 + 1.9125f32 * ctx.ad,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 63.75f32 + 3.825f32 * ctx.ad,2 => 159.375f32 + 4.303125f32 * ctx.ad,3 => 255f32 + 4.78125f32 * ctx.ad,4 => 350.625f32 + 5.259375f32 * ctx.ad,5 => 446.25f32 + 5.7375f32 * ctx.ad,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 60f32 + 0.8f32 * ctx.ad,2 => 80f32 + 0.8f32 * ctx.ad,3 => 100f32 + 0.8f32 * ctx.ad,4 => 120f32 + 0.8f32 * ctx.ad,5 => 140f32 + 0.8f32 * ctx.ad,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 60f32 + 0.8f32 * ctx.ad,2 => 80f32 + 0.8f32 * ctx.ad,3 => 100f32 + 0.8f32 * ctx.ad,4 => 120f32 + 0.8f32 * ctx.ad,5 => 140f32 + 0.8f32 * ctx.ad,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 60f32 + 0.8f32 * ctx.ad,2 => 80f32 + 0.8f32 * ctx.ad,3 => 100f32 + 0.8f32 * ctx.ad,4 => 120f32 + 0.8f32 * ctx.ad,5 => 140f32 + 0.8f32 * ctx.ad,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:650f32,per_level:114f32},mana:CachedChampionStatsMap{flat:0f32,per_level:0f32},armor:CachedChampionStatsMap{flat:38f32,per_level:4.8f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:60f32,per_level:5f32},attack_speed:CachedChampionStatsMap{flat:0.651f32,per_level:2.5f32},movespeed:345f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.651000022888183f32,attack_range:175f32,aram_damage_taken:1f32,aram_damage_dealt:1.05f32,urf_damage_taken:0.7f32,urf_damage_dealt:1.15f32,},
                    merge_data: &[(1, 5),(2, 6),(3, 7),(9, 8)],
                };pub static AHRI: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Middle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Max), 
                damage_type: DamageType::Mixed, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Void), 
                damage_type: DamageType::Unknown, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::Minion), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::MinionMax), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => (40f32 + 0.5f32 * ctx.ap) * ctx.magic_multiplier + (40f32 + 0.5f32 * ctx.ap),2 => (65f32 + 0.5f32 * ctx.ap) * ctx.magic_multiplier + (65f32 + 0.5f32 * ctx.ap),3 => (90f32 + 0.5f32 * ctx.ap) * ctx.magic_multiplier + (90f32 + 0.5f32 * ctx.ap),4 => (115f32 + 0.5f32 * ctx.ap) * ctx.magic_multiplier + (115f32 + 0.5f32 * ctx.ap),5 => (140f32 + 0.5f32 * ctx.ap) * ctx.magic_multiplier + (140f32 + 0.5f32 * ctx.ap),_ => 0.0 }},|ctx| { match ctx.q_level {1 => 40f32 + 0.5f32 * ctx.ap,2 => 65f32 + 0.5f32 * ctx.ap,3 => 90f32 + 0.5f32 * ctx.ap,4 => 115f32 + 0.5f32 * ctx.ap,5 => 140f32 + 0.5f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 12f32 + 0.12f32 * ctx.ap,2 => 18f32 + 0.12f32 * ctx.ap,3 => 24f32 + 0.12f32 * ctx.ap,4 => 30f32 + 0.12f32 * ctx.ap,5 => 36f32 + 0.12f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 64f32 + 0.64f32 * ctx.ap,2 => 96f32 + 0.64f32 * ctx.ap,3 => 128f32 + 0.64f32 * ctx.ap,4 => 160f32 + 0.64f32 * ctx.ap,5 => 192f32 + 0.64f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 40f32 + 0.4f32 * ctx.ap,2 => 60f32 + 0.4f32 * ctx.ap,3 => 80f32 + 0.4f32 * ctx.ap,4 => 100f32 + 0.4f32 * ctx.ap,5 => 120f32 + 0.4f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 80f32 + 0.8f32 * ctx.ap,2 => 120f32 + 0.8f32 * ctx.ap,3 => 160f32 + 0.8f32 * ctx.ap,4 => 200f32 + 0.8f32 * ctx.ap,5 => 240f32 + 0.8f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 24f32 + 0.24f32 * ctx.ap,2 => 36f32 + 0.24f32 * ctx.ap,3 => 48f32 + 0.24f32 * ctx.ap,4 => 60f32 + 0.24f32 * ctx.ap,5 => 72f32 + 0.24f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 0.0f32,2 => 0.0f32,3 => 0.0f32,4 => 0.0f32,5 => 0.0f32,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 3f32 * (75f32 + 0.35f32 * ctx.ap),2 => 3f32 * (125f32 + 0.35f32 * ctx.ap),3 => 3f32 * (175f32 + 0.35f32 * ctx.ap),_ => 0.0 }},|ctx| { match ctx.r_level {1 => 75f32 + 0.35f32 * ctx.ap,2 => 125f32 + 0.35f32 * ctx.ap,3 => 175f32 + 0.35f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:590f32,per_level:104f32},mana:CachedChampionStatsMap{flat:418f32,per_level:25f32},armor:CachedChampionStatsMap{flat:21f32,per_level:4.2f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:53f32,per_level:3f32},attack_speed:CachedChampionStatsMap{flat:0.668f32,per_level:2.2f32},movespeed:330f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:550f32,aram_damage_taken:1f32,aram_damage_dealt:1f32,urf_damage_taken:1.05f32,urf_damage_dealt:0.9f32,},
                    merge_data: &[(1, 0),(4, 3),(5, 6),(9, 8)],
                };pub static AKALI: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Middle,Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 45f32 + 0.65f32 * ctx.ad + 0.6f32 * ctx.ap,2 => 70f32 + 0.65f32 * ctx.ad + 0.6f32 * ctx.ap,3 => 95f32 + 0.65f32 * ctx.ad + 0.6f32 * ctx.ap,4 => 120f32 + 0.65f32 * ctx.ad + 0.6f32 * ctx.ap,5 => 145f32 + 0.65f32 * ctx.ad + 0.6f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 49f32 + 0.7f32 * ctx.ad + 0.77f32 * ctx.ap,2 => 98f32 + 0.7f32 * ctx.ad + 0.77f32 * ctx.ap,3 => 147f32 + 0.7f32 * ctx.ad + 0.77f32 * ctx.ap,4 => 196f32 + 0.7f32 * ctx.ad + 0.77f32 * ctx.ap,5 => 245f32 + 0.7f32 * ctx.ad + 0.77f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 21f32 + 0.3f32 * ctx.ad + 0.33f32 * ctx.ap,2 => 42f32 + 0.3f32 * ctx.ad + 0.33f32 * ctx.ap,3 => 63f32 + 0.3f32 * ctx.ad + 0.33f32 * ctx.ap,4 => 84f32 + 0.3f32 * ctx.ad + 0.33f32 * ctx.ap,5 => 105f32 + 0.3f32 * ctx.ad + 0.33f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 70f32 + ctx.ad + 1.1f32 * ctx.ap,2 => 140f32 + ctx.ad + 1.1f32 * ctx.ap,3 => 210f32 + ctx.ad + 1.1f32 * ctx.ap,4 => 280f32 + ctx.ad + 1.1f32 * ctx.ap,5 => 350f32 + ctx.ad + 1.1f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 110f32 + 0.5f32 * ctx.bonus_ad + 0.3f32 * ctx.ap,2 => 220f32 + 0.5f32 * ctx.bonus_ad + 0.3f32 * ctx.ap,3 => 330f32 + 0.5f32 * ctx.bonus_ad + 0.3f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 70f32 + 0.3f32 * ctx.ap,2 => 140f32 + 0.3f32 * ctx.ap,3 => 210f32 + 0.3f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 210f32 + 0.9f32 * ctx.ap,2 => 420f32 + 0.9f32 * ctx.ap,3 => 630f32 + 0.9f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:600f32,per_level:119f32},mana:CachedChampionStatsMap{flat:200f32,per_level:0f32},armor:CachedChampionStatsMap{flat:23f32,per_level:4.7f32},magic_resist:CachedChampionStatsMap{flat:37f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:62f32,per_level:3.3f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:3.2f32},movespeed:345f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:125f32,aram_damage_taken:0.95f32,aram_damage_dealt:1f32,urf_damage_taken:0.9f32,urf_damage_dealt:1f32,},
                    merge_data: &[(6, 5)],
                };pub static AKSHAN: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Middle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Void), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 10f32 + 1.6f32 * ctx.ad,2 => 50f32 + 1.6f32 * ctx.ad,3 => 90f32 + 1.6f32 * ctx.ad,4 => 130f32 + 1.6f32 * ctx.ad,5 => 170f32 + 1.6f32 * ctx.ad,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 5f32 + 0.8f32 * ctx.ad,2 => 25f32 + 0.8f32 * ctx.ad,3 => 45f32 + 0.8f32 * ctx.ad,4 => 65f32 + 0.8f32 * ctx.ad,5 => 85f32 + 0.8f32 * ctx.ad,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:630f32,per_level:107f32},mana:CachedChampionStatsMap{flat:350f32,per_level:40f32},armor:CachedChampionStatsMap{flat:26f32,per_level:4.7f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:52f32,per_level:3f32},attack_speed:CachedChampionStatsMap{flat:0.638f32,per_level:4f32},movespeed:330f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.4f32,attack_range:500f32,aram_damage_taken:0.95f32,aram_damage_dealt:1.05f32,urf_damage_taken:1f32,urf_damage_dealt:1.05f32,},
                    merge_data: &[],
                };pub static ALISTAR: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Support],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::Void), 
                damage_type: DamageType::Unknown, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 60f32 + 0.8f32 * ctx.ap,2 => 100f32 + 0.8f32 * ctx.ap,3 => 140f32 + 0.8f32 * ctx.ap,4 => 180f32 + 0.8f32 * ctx.ap,5 => 220f32 + 0.8f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 55f32 + ctx.ap,2 => 110f32 + ctx.ap,3 => 165f32 + ctx.ap,4 => 220f32 + ctx.ap,5 => 275f32 + ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 80f32 + 0.7f32 * ctx.ap,2 => 110f32 + 0.7f32 * ctx.ap,3 => 140f32 + 0.7f32 * ctx.ap,4 => 170f32 + 0.7f32 * ctx.ap,5 => 200f32 + 0.7f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 8f32 + 0.07f32 * ctx.ap,2 => 11f32 + 0.07f32 * ctx.ap,3 => 14f32 + 0.07f32 * ctx.ap,4 => 17f32 + 0.07f32 * ctx.ap,5 => 20f32 + 0.07f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 0.55f32,2 => 0.65f32,3 => 0.75f32,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:685f32,per_level:120f32},mana:CachedChampionStatsMap{flat:350f32,per_level:40f32},armor:CachedChampionStatsMap{flat:40f32,per_level:4.7f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:62f32,per_level:3.75f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:2.125f32},movespeed:330f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:125f32,aram_damage_taken:1.05f32,aram_damage_dealt:0.95f32,urf_damage_taken:0.95f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static AMBESSA: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_2), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_2Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::Void), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::Void), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::Void), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 40f32 + 0.6f32 * ctx.bonus_ad + 0.02f32 * ctx.enemy_max_health + 0.03f32 * 0.01f32 * ctx.bonus_ad,2 => 60f32 + 0.6f32 * ctx.bonus_ad + 0.03f32 * ctx.enemy_max_health + 0.03f32 * 0.01f32 * ctx.bonus_ad,3 => 80f32 + 0.6f32 * ctx.bonus_ad + 0.04f32 * ctx.enemy_max_health + 0.03f32 * 0.01f32 * ctx.bonus_ad,4 => 100f32 + 0.6f32 * ctx.bonus_ad + 0.05f32 * ctx.enemy_max_health + 0.03f32 * 0.01f32 * ctx.bonus_ad,5 => 120f32 + 0.6f32 * ctx.bonus_ad + 0.06f32 * ctx.enemy_max_health + 0.03f32 * 0.01f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 20f32 + 0.3f32 * ctx.bonus_ad + 0.01f32 * ctx.enemy_max_health + 0.015f32 * 0.01f32 * ctx.bonus_ad,2 => 30f32 + 0.3f32 * ctx.bonus_ad + 0.015f32 * ctx.enemy_max_health + 0.015f32 * 0.01f32 * ctx.bonus_ad,3 => 40f32 + 0.3f32 * ctx.bonus_ad + 0.02f32 * ctx.enemy_max_health + 0.015f32 * 0.01f32 * ctx.bonus_ad,4 => 50f32 + 0.3f32 * ctx.bonus_ad + 0.025f32 * ctx.enemy_max_health + 0.015f32 * 0.01f32 * ctx.bonus_ad,5 => 60f32 + 0.3f32 * ctx.bonus_ad + 0.03f32 * ctx.enemy_max_health + 0.015f32 * 0.01f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 75f32 + 0.75f32 * ctx.bonus_ad,2 => 112.5f32 + 0.75f32 * ctx.bonus_ad,3 => 150f32 + 0.75f32 * ctx.bonus_ad,4 => 187.5f32 + 0.75f32 * ctx.bonus_ad,5 => 225f32 + 0.75f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 50f32 + 0.5f32 * ctx.bonus_ad,2 => 75f32 + 0.5f32 * ctx.bonus_ad,3 => 100f32 + 0.5f32 * ctx.bonus_ad,4 => 125f32 + 0.5f32 * ctx.bonus_ad,5 => 150f32 + 0.5f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 80f32 + 0.8f32 * ctx.bonus_ad,2 => 120f32 + 0.9f32 * ctx.bonus_ad,3 => 160f32 + ctx.bonus_ad,4 => 200f32 + 1.1f32 * ctx.bonus_ad,5 => 240f32 + 1.2f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 40f32 + 0.4f32 * ctx.bonus_ad,2 => 60f32 + 0.45f32 * ctx.bonus_ad,3 => 80f32 + 0.5f32 * ctx.bonus_ad,4 => 100f32 + 0.55f32 * ctx.bonus_ad,5 => 120f32 + 0.6f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 0.1f32,2 => 0.2f32,3 => 0.3f32,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:630f32,per_level:110f32},mana:CachedChampionStatsMap{flat:200f32,per_level:0f32},armor:CachedChampionStatsMap{flat:35f32,per_level:4.9f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:63f32,per_level:3f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:2.5f32},movespeed:335f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:125f32,aram_damage_taken:1.05f32,aram_damage_dealt:1f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static AMUMU: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Jungle,Position::Support],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 70f32 + 0.85f32 * ctx.ap,2 => 95f32 + 0.85f32 * ctx.ap,3 => 120f32 + 0.85f32 * ctx.ap,4 => 145f32 + 0.85f32 * ctx.ap,5 => 170f32 + 0.85f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 5f32 + 0.005f32 * ctx.enemy_max_health + 0.0025f32 * 0.01f32 * ctx.ap,2 => 5f32 + 0.00625f32 * ctx.enemy_max_health + 0.0025f32 * 0.01f32 * ctx.ap,3 => 5f32 + 0.0075f32 * ctx.enemy_max_health + 0.0025f32 * 0.01f32 * ctx.ap,4 => 5f32 + 0.00875f32 * ctx.enemy_max_health + 0.0025f32 * 0.01f32 * ctx.ap,5 => 5f32 + 0.01f32 * ctx.enemy_max_health + 0.0025f32 * 0.01f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 5f32 + 0.03f32 * ctx.bonus_armor + 0.03f32 * ctx.bonus_magic_resist,2 => 7f32 + 0.03f32 * ctx.bonus_armor + 0.03f32 * ctx.bonus_magic_resist,3 => 9f32 + 0.03f32 * ctx.bonus_armor + 0.03f32 * ctx.bonus_magic_resist,4 => 11f32 + 0.03f32 * ctx.bonus_armor + 0.03f32 * ctx.bonus_magic_resist,5 => 13f32 + 0.03f32 * ctx.bonus_armor + 0.03f32 * ctx.bonus_magic_resist,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 65f32 + 0.5f32 * ctx.ap,2 => 95f32 + 0.5f32 * ctx.ap,3 => 125f32 + 0.5f32 * ctx.ap,4 => 155f32 + 0.5f32 * ctx.ap,5 => 185f32 + 0.5f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 200f32 + 0.8f32 * ctx.ap,2 => 300f32 + 0.8f32 * ctx.ap,3 => 400f32 + 0.8f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:685f32,per_level:94f32},mana:CachedChampionStatsMap{flat:285f32,per_level:40f32},armor:CachedChampionStatsMap{flat:33f32,per_level:4f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:57f32,per_level:3.8f32},attack_speed:CachedChampionStatsMap{flat:0.736f32,per_level:2.18f32},movespeed:335f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.638000011444091f32,attack_range:125f32,aram_damage_taken:1f32,aram_damage_dealt:1f32,urf_damage_taken:0.9f32,urf_damage_dealt:1.1f32,},
                    merge_data: &[],
                };pub static ANIVIA: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Middle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 60f32 + 0.45f32 * ctx.ap,2 => 95f32 + 0.45f32 * ctx.ap,3 => 130f32 + 0.45f32 * ctx.ap,4 => 165f32 + 0.45f32 * ctx.ap,5 => 200f32 + 0.45f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 50f32 + 0.25f32 * ctx.ap,2 => 70f32 + 0.25f32 * ctx.ap,3 => 90f32 + 0.25f32 * ctx.ap,4 => 110f32 + 0.25f32 * ctx.ap,5 => 130f32 + 0.25f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 0.0f32,2 => 0.0f32,3 => 0.0f32,4 => 0.0f32,5 => 0.0f32,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 15f32 + 0.0625f32 * ctx.ap,2 => 22.5f32 + 0.0625f32 * ctx.ap,3 => 30f32 + 0.0625f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:550f32,per_level:92f32},mana:CachedChampionStatsMap{flat:495f32,per_level:45f32},armor:CachedChampionStatsMap{flat:21f32,per_level:4.5f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:51f32,per_level:3.2f32},attack_speed:CachedChampionStatsMap{flat:0.658f32,per_level:1.68f32},movespeed:325f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:600f32,aram_damage_taken:1f32,aram_damage_dealt:1.05f32,urf_damage_taken:0.92f32,urf_damage_dealt:1.15f32,},
                    merge_data: &[],
                };pub static ANNIE: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Middle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 80f32 + 0.8f32 * ctx.ap,2 => 120f32 + 0.8f32 * ctx.ap,3 => 160f32 + 0.8f32 * ctx.ap,4 => 200f32 + 0.8f32 * ctx.ap,5 => 240f32 + 0.8f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 70f32 + 0.8f32 * ctx.ap,2 => 110f32 + 0.8f32 * ctx.ap,3 => 150f32 + 0.8f32 * ctx.ap,4 => 190f32 + 0.8f32 * ctx.ap,5 => 230f32 + 0.8f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 25f32 + 0.4f32 * ctx.ap,2 => 35f32 + 0.4f32 * ctx.ap,3 => 45f32 + 0.4f32 * ctx.ap,4 => 55f32 + 0.4f32 * ctx.ap,5 => 65f32 + 0.4f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 0.1f32,2 => 0.15f32,3 => 0.2f32,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:560f32,per_level:96f32},mana:CachedChampionStatsMap{flat:418f32,per_level:25f32},armor:CachedChampionStatsMap{flat:23f32,per_level:4f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:50f32,per_level:2.65f32},attack_speed:CachedChampionStatsMap{flat:0.61f32,per_level:1.36f32},movespeed:335f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:625f32,aram_damage_taken:1.05f32,aram_damage_dealt:0.95f32,urf_damage_taken:1.1f32,urf_damage_dealt:0.9f32,},
                    merge_data: &[],
                };pub static APHELIOS: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Bottom],
                    metadata: &[],
                    closures: &[],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:600f32,per_level:102f32},mana:CachedChampionStatsMap{flat:348f32,per_level:42f32},armor:CachedChampionStatsMap{flat:26f32,per_level:4.2f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:55f32,per_level:2.3f32},attack_speed:CachedChampionStatsMap{flat:0.665f32,per_level:2.1f32},movespeed:325f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.658f32,attack_range:550f32,aram_damage_taken:1f32,aram_damage_dealt:1f32,urf_damage_taken:1f32,urf_damage_dealt:1.05f32,},
                    merge_data: &[],
                };pub static ASHE: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Bottom,Position::Support],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::W(AbilityName::Void), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.w_level {1 => 60f32 + 1.1f32 * ctx.bonus_ad,2 => 95f32 + 1.1f32 * ctx.bonus_ad,3 => 130f32 + 1.1f32 * ctx.bonus_ad,4 => 165f32 + 1.1f32 * ctx.bonus_ad,5 => 200f32 + 1.1f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 200f32 + 1.2f32 * ctx.ap,2 => 400f32 + 1.2f32 * ctx.ap,3 => 600f32 + 1.2f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:610f32,per_level:101f32},mana:CachedChampionStatsMap{flat:280f32,per_level:35f32},armor:CachedChampionStatsMap{flat:26f32,per_level:4.6f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:59f32,per_level:3.45f32},attack_speed:CachedChampionStatsMap{flat:0.658f32,per_level:3.33f32},movespeed:325f32,critical_strike_damage:100f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.657999992370605f32,attack_range:600f32,aram_damage_taken:1f32,aram_damage_dealt:1.05f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static AURELIONSOL: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Middle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_2), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_2Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::Void), 
                damage_type: DamageType::Unknown, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 5.625f32 + 0.06875f32 * ctx.ap,2 => 7.5f32 + 0.06875f32 * ctx.ap,3 => 9.375f32 + 0.06875f32 * ctx.ap,4 => 11.25f32 + 0.06875f32 * ctx.ap,5 => 13.125f32 + 0.06875f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 45f32 + 0.55f32 * ctx.ap,2 => 60f32 + 0.55f32 * ctx.ap,3 => 75f32 + 0.55f32 * ctx.ap,4 => 90f32 + 0.55f32 * ctx.ap,5 => 105f32 + 0.55f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 146.25f32 + 1.7875f32 * ctx.ap,2 => 195f32 + 1.7875f32 * ctx.ap,3 => 243.75f32 + 1.7875f32 * ctx.ap,4 => 292.5f32 + 1.7875f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 1.08f32,2 => 1.09f32,3 => 1.1f32,4 => 1.11f32,5 => 1.12f32,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 50f32 + 0.6f32 * ctx.ap,2 => 75f32 + 0.6f32 * ctx.ap,3 => 100f32 + 0.6f32 * ctx.ap,4 => 125f32 + 0.6f32 * ctx.ap,5 => 150f32 + 0.6f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 2.5f32 + 0.03f32 * ctx.ap,2 => 3.75f32 + 0.03f32 * ctx.ap,3 => 5f32 + 0.03f32 * ctx.ap,4 => 6.25f32 + 0.03f32 * ctx.ap,5 => 7.5f32 + 0.03f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 150f32 + 0.75f32 * ctx.ap,2 => 250f32 + 0.75f32 * ctx.ap,3 => 350f32 + 0.75f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:600f32,per_level:90f32},mana:CachedChampionStatsMap{flat:530f32,per_level:40f32},armor:CachedChampionStatsMap{flat:22f32,per_level:4f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:58f32,per_level:3.2f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:1.5f32},movespeed:335f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:550f32,aram_damage_taken:1f32,aram_damage_dealt:1f32,urf_damage_taken:0.9f32,urf_damage_dealt:1.1f32,},
                    merge_data: &[],
                };pub static AURORA: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Middle,Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 45f32 + 0.4f32 * ctx.ap,2 => 70f32 + 0.4f32 * ctx.ap,3 => 95f32 + 0.4f32 * ctx.ap,4 => 120f32 + 0.4f32 * ctx.ap,5 => 145f32 + 0.4f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 70f32 + 0.7f32 * ctx.ap,2 => 110f32 + 0.7f32 * ctx.ap,3 => 150f32 + 0.7f32 * ctx.ap,4 => 190f32 + 0.7f32 * ctx.ap,5 => 230f32 + 0.7f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 175f32 + 0.7f32 * ctx.ap,2 => 275f32 + 0.7f32 * ctx.ap,3 => 375f32 + 0.7f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:607f32,per_level:110f32},mana:CachedChampionStatsMap{flat:475f32,per_level:30f32},armor:CachedChampionStatsMap{flat:23f32,per_level:4.5f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:53f32,per_level:3f32},attack_speed:CachedChampionStatsMap{flat:0.668f32,per_level:2f32},movespeed:335f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.668f32,attack_range:550f32,aram_damage_taken:1f32,aram_damage_dealt:1f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static AZIR: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Middle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::R(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.r_level {1 => 200f32 + 0.75f32 * ctx.ap,2 => 400f32 + 0.75f32 * ctx.ap,3 => 600f32 + 0.75f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:575f32,per_level:119f32},mana:CachedChampionStatsMap{flat:320f32,per_level:40f32},armor:CachedChampionStatsMap{flat:25f32,per_level:5f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:56f32,per_level:3.5f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:5f32},movespeed:330f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.694f32,attack_range:525f32,aram_damage_taken:0.95f32,aram_damage_dealt:1f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static BARD: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Support],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 0.0f32,2 => 0.0f32,3 => 0.0f32,4 => 0.0f32,5 => 0.0f32,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:630f32,per_level:103f32},mana:CachedChampionStatsMap{flat:350f32,per_level:50f32},armor:CachedChampionStatsMap{flat:34f32,per_level:5f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:52f32,per_level:3f32},attack_speed:CachedChampionStatsMap{flat:0.658f32,per_level:2f32},movespeed:335f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.658f32,attack_range:500f32,aram_damage_taken:0.85f32,aram_damage_dealt:1.15f32,urf_damage_taken:0.85f32,urf_damage_dealt:1.15f32,},
                    merge_data: &[],
                };pub static BELVETH: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Jungle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_2), 
                damage_type: DamageType::True, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 0f32 + ctx.ad,2 => 5f32 + ctx.ad,3 => 10f32 + ctx.ad,4 => 15f32 + ctx.ad,5 => 20f32 + ctx.ad,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 70f32 + ctx.bonus_ad + 1.25f32 * ctx.ap,2 => 110f32 + ctx.bonus_ad + 1.25f32 * ctx.ap,3 => 150f32 + ctx.bonus_ad + 1.25f32 * ctx.ap,4 => 190f32 + ctx.bonus_ad + 1.25f32 * ctx.ap,5 => 230f32 + ctx.bonus_ad + 1.25f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 0.35f32,2 => 0.4f32,3 => 0.45f32,4 => 0.5f32,5 => 0.55f32,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 150f32 + ctx.ap + 0.25f32 * ctx.enemy_missing_health,2 => 200f32 + ctx.ap + 0.25f32 * ctx.enemy_missing_health,3 => 250f32 + ctx.ap + 0.25f32 * ctx.enemy_missing_health,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:610f32,per_level:99f32},mana:CachedChampionStatsMap{flat:0f32,per_level:0f32},armor:CachedChampionStatsMap{flat:32f32,per_level:4.7f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:60f32,per_level:1.5f32},attack_speed:CachedChampionStatsMap{flat:0.85f32,per_level:0f32},movespeed:340f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.85f32,attack_range:150f32,aram_damage_taken:0.95f32,aram_damage_dealt:1.05f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static BLITZCRANK: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Support],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::Void), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 110f32 + 1.2f32 * ctx.ap,2 => 160f32 + 1.2f32 * ctx.ap,3 => 210f32 + 1.2f32 * ctx.ap,4 => 260f32 + 1.2f32 * ctx.ap,5 => 310f32 + 1.2f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => ctx.ad + 0.25f32 * ctx.ap,2 => ctx.ad + 0.25f32 * ctx.ap,3 => ctx.ad + 0.25f32 * ctx.ap,4 => ctx.ad + 0.25f32 * ctx.ap,5 => ctx.ad + 0.25f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 50f32 + 0.3f32 * ctx.ap + 0.02f32 * ctx.max_mana,2 => 100f32 + 0.4f32 * ctx.ap + 0.02f32 * ctx.max_mana,3 => 150f32 + 0.5f32 * ctx.ap + 0.02f32 * ctx.max_mana,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 275f32 + ctx.ap,2 => 400f32 + ctx.ap,3 => 525f32 + ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:600f32,per_level:109f32},mana:CachedChampionStatsMap{flat:267f32,per_level:40f32},armor:CachedChampionStatsMap{flat:37f32,per_level:4.7f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:62f32,per_level:3.5f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:1.13f32},movespeed:325f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:125f32,aram_damage_taken:1f32,aram_damage_dealt:1f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static BRAND: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Jungle,Position::Middle,Position::Support],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_2Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.w_level {1 => 75f32 + 0.7f32 * ctx.ap,2 => 120f32 + 0.7f32 * ctx.ap,3 => 165f32 + 0.7f32 * ctx.ap,4 => 210f32 + 0.7f32 * ctx.ap,5 => 255f32 + 0.7f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 93.75f32 + 0.875f32 * ctx.ap,2 => 150f32 + 0.875f32 * ctx.ap,3 => 206.25f32 + 0.875f32 * ctx.ap,4 => 262.5f32 + 0.875f32 * ctx.ap,5 => 318.75f32 + 0.875f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:570f32,per_level:105f32},mana:CachedChampionStatsMap{flat:469f32,per_level:21f32},armor:CachedChampionStatsMap{flat:27f32,per_level:4.2f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:57f32,per_level:3f32},attack_speed:CachedChampionStatsMap{flat:0.681f32,per_level:2f32},movespeed:340f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:550f32,aram_damage_taken:1.1f32,aram_damage_dealt:0.95f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static BRAUM: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Support],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 75f32 + 0.025f32 * ctx.max_health,2 => 125f32 + 0.025f32 * ctx.max_health,3 => 175f32 + 0.025f32 * ctx.max_health,4 => 225f32 + 0.025f32 * ctx.max_health,5 => 275f32 + 0.025f32 * ctx.max_health,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 0.0f32,2 => 0.0f32,3 => 0.0f32,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:610f32,per_level:112f32},mana:CachedChampionStatsMap{flat:311f32,per_level:45f32},armor:CachedChampionStatsMap{flat:35f32,per_level:5f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:55f32,per_level:3.2f32},attack_speed:CachedChampionStatsMap{flat:0.644f32,per_level:3.5f32},movespeed:335f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.643999993801116f32,attack_range:125f32,aram_damage_taken:1f32,aram_damage_dealt:1f32,urf_damage_taken:0.82f32,urf_damage_dealt:1.05f32,},
                    merge_data: &[],
                };pub static BRIAR: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Jungle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_3), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_2Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_3Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::Minion1), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 60f32 + 0.8f32 * ctx.bonus_ad + 0.6f32 * ctx.ap,2 => 85f32 + 0.8f32 * ctx.bonus_ad + 0.6f32 * ctx.ap,3 => 110f32 + 0.8f32 * ctx.bonus_ad + 0.6f32 * ctx.ap,4 => 135f32 + 0.8f32 * ctx.bonus_ad + 0.6f32 * ctx.ap,5 => 160f32 + 0.8f32 * ctx.bonus_ad + 0.6f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 0.55f32,2 => 0.65f32,3 => 0.75f32,4 => 0.85f32,5 => 0.95f32,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 80f32 + ctx.bonus_ad + ctx.ap,2 => 115f32 + ctx.bonus_ad + ctx.ap,3 => 150f32 + ctx.bonus_ad + ctx.ap,4 => 185f32 + ctx.bonus_ad + ctx.ap,5 => 220f32 + ctx.bonus_ad + ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 2f32 + 0.025f32 * ctx.bonus_ad + 0.025f32 * ctx.ap,2 => 2.875f32 + 0.025f32 * ctx.bonus_ad + 0.025f32 * ctx.ap,3 => 3.75f32 + 0.025f32 * ctx.bonus_ad + 0.025f32 * ctx.ap,4 => 4.625f32 + 0.025f32 * ctx.bonus_ad + 0.025f32 * ctx.ap,5 => 5.5f32 + 0.025f32 * ctx.bonus_ad + 0.025f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 140f32 + 2.4f32 * ctx.bonus_ad + 2.4f32 * ctx.ap,2 => 215f32 + 2.4f32 * ctx.bonus_ad + 2.4f32 * ctx.ap,3 => 290f32 + 2.4f32 * ctx.bonus_ad + 2.4f32 * ctx.ap,4 => 365f32 + 2.4f32 * ctx.bonus_ad + 2.4f32 * ctx.ap,5 => 440f32 + 2.4f32 * ctx.bonus_ad + 2.4f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 220f32 + 3.4f32 * ctx.bonus_ad + 3.4f32 * ctx.ap,2 => 330f32 + 3.4f32 * ctx.bonus_ad + 3.4f32 * ctx.ap,3 => 440f32 + 3.4f32 * ctx.bonus_ad + 3.4f32 * ctx.ap,4 => 550f32 + 3.4f32 * ctx.bonus_ad + 3.4f32 * ctx.ap,5 => 660f32 + 3.4f32 * ctx.bonus_ad + 3.4f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:625f32,per_level:95f32},mana:CachedChampionStatsMap{flat:0f32,per_level:0f32},armor:CachedChampionStatsMap{flat:30f32,per_level:4.2f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:60f32,per_level:2.5f32},attack_speed:CachedChampionStatsMap{flat:0.644f32,per_level:2f32},movespeed:340f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.669f32,attack_range:125f32,aram_damage_taken:1f32,aram_damage_dealt:1.05f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static CAITLYN: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Bottom],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::Void), 
                damage_type: DamageType::Unknown, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::Void), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 50f32 + 1.25f32 * ctx.ad,2 => 90f32 + 1.45f32 * ctx.ad,3 => 130f32 + 1.65f32 * ctx.ad,4 => 170f32 + 1.85f32 * ctx.ad,5 => 210f32 + 2.05f32 * ctx.ad,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 30f32 + 0.75f32 * ctx.ad,2 => 54f32 + 0.87f32 * ctx.ad,3 => 78f32 + 0.99f32 * ctx.ad,4 => 102f32 + 1.11f32 * ctx.ad,5 => 126f32 + 1.23f32 * ctx.ad,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 35f32 + 0.3f32 * ctx.bonus_ad,2 => 80f32 + 0.3f32 * ctx.bonus_ad,3 => 125f32 + 0.3f32 * ctx.bonus_ad,4 => 170f32 + 0.3f32 * ctx.bonus_ad,5 => 215f32 + 0.3f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 80f32 + 0.8f32 * ctx.ap,2 => 130f32 + 0.8f32 * ctx.ap,3 => 180f32 + 0.8f32 * ctx.ap,4 => 230f32 + 0.8f32 * ctx.ap,5 => 280f32 + 0.8f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 300f32 + ctx.bonus_ad,2 => 475f32 + ctx.bonus_ad,3 => 650f32 + ctx.bonus_ad,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:580f32,per_level:107f32},mana:CachedChampionStatsMap{flat:315f32,per_level:40f32},armor:CachedChampionStatsMap{flat:27f32,per_level:4.7f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:62f32,per_level:3.8f32},attack_speed:CachedChampionStatsMap{flat:0.681f32,per_level:4f32},movespeed:325f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:650f32,aram_damage_taken:1f32,aram_damage_dealt:1f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static CAMILLE: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Support,Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1), 
                damage_type: DamageType::Unknown, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_2), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::Monster1), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::Monster2), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 0.25f32,2 => 0.3f32,3 => 0.35f32,4 => 0.4f32,5 => 0.45f32,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 0.06f32 * ctx.enemy_max_health + 0.025f32 * 0.01f32 * ctx.bonus_ad,2 => 0.065f32 * ctx.enemy_max_health + 0.025f32 * 0.01f32 * ctx.bonus_ad,3 => 0.07f32 * ctx.enemy_max_health + 0.025f32 * 0.01f32 * ctx.bonus_ad,4 => 0.075f32 * ctx.enemy_max_health + 0.025f32 * 0.01f32 * ctx.bonus_ad,5 => 0.08f32 * ctx.enemy_max_health + 0.025f32 * 0.01f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 60f32 + 0.6f32 * ctx.bonus_ad,2 => 85f32 + 0.6f32 * ctx.bonus_ad,3 => 110f32 + 0.6f32 * ctx.bonus_ad,4 => 135f32 + 0.6f32 * ctx.bonus_ad,5 => 160f32 + 0.6f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 30f32 + 0.3f32 * ctx.bonus_ad,2 => 42.5f32 + 0.3f32 * ctx.bonus_ad,3 => 55f32 + 0.3f32 * ctx.bonus_ad,4 => 67.5f32 + 0.3f32 * ctx.bonus_ad,5 => 80f32 + 0.3f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 0.03f32 * ctx.enemy_max_health + 0.0125f32 * 0.01f32 * ctx.bonus_ad,2 => 0.0325f32 * ctx.enemy_max_health + 0.0125f32 * 0.01f32 * ctx.bonus_ad,3 => 0.035f32 * ctx.enemy_max_health + 0.0125f32 * 0.01f32 * ctx.bonus_ad,4 => 0.0375f32 * ctx.enemy_max_health + 0.0125f32 * 0.01f32 * ctx.bonus_ad,5 => 0.04f32 * ctx.enemy_max_health + 0.0125f32 * 0.01f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 0.04f32 * ctx.enemy_current_health,2 => 0.06f32 * ctx.enemy_current_health,3 => 0.08f32 * ctx.enemy_current_health,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:650f32,per_level:99f32},mana:CachedChampionStatsMap{flat:339f32,per_level:52f32},armor:CachedChampionStatsMap{flat:35f32,per_level:4.5f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:68f32,per_level:3.8f32},attack_speed:CachedChampionStatsMap{flat:0.644f32,per_level:2.5f32},movespeed:340f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.643999993801116f32,attack_range:125f32,aram_damage_taken:0.95f32,aram_damage_dealt:1.05f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static CASSIOPEIA: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Middle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_2Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 10.71f32 + 0.0929f32 * ctx.ap,2 => 15.71f32 + 0.0929f32 * ctx.ap,3 => 20.71f32 + 0.0929f32 * ctx.ap,4 => 25.71f32 + 0.0929f32 * ctx.ap,5 => 30.71f32 + 0.0929f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 75f32 + 0.65f32 * ctx.ap,2 => 110f32 + 0.65f32 * ctx.ap,3 => 145f32 + 0.65f32 * ctx.ap,4 => 180f32 + 0.65f32 * ctx.ap,5 => 215f32 + 0.65f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 0.4f32,2 => 0.5f32,3 => 0.6f32,4 => 0.7f32,5 => 0.8f32,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 20f32 + 0.1f32 * ctx.ap,2 => 25f32 + 0.1f32 * ctx.ap,3 => 30f32 + 0.1f32 * ctx.ap,4 => 35f32 + 0.1f32 * ctx.ap,5 => 40f32 + 0.1f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 20f32 + 0.55f32 * ctx.ap,2 => 43f32 + 0.55f32 * ctx.ap,3 => 66f32 + 0.55f32 * ctx.ap,4 => 89f32 + 0.55f32 * ctx.ap,5 => 112f32 + 0.55f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 0.025f32 * ctx.ap,2 => 0.02875f32 * ctx.ap,3 => 0.0325f32 * ctx.ap,4 => 0.03625f32 * ctx.ap,5 => 0.04f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 150f32 + 0.5f32 * ctx.ap,2 => 250f32 + 0.5f32 * ctx.ap,3 => 350f32 + 0.5f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:630f32,per_level:104f32},mana:CachedChampionStatsMap{flat:450f32,per_level:40f32},armor:CachedChampionStatsMap{flat:18f32,per_level:4.7f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:53f32,per_level:3f32},attack_speed:CachedChampionStatsMap{flat:0.647f32,per_level:1.5f32},movespeed:328f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.647000014781951f32,attack_range:550f32,aram_damage_taken:1f32,aram_damage_dealt:1f32,urf_damage_taken:1.05f32,urf_damage_dealt:0.95f32,},
                    merge_data: &[],
                };pub static CHOGATH: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_2Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::True, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_2Min), 
                damage_type: DamageType::True, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 80f32 + ctx.ap,2 => 135f32 + ctx.ap,3 => 190f32 + ctx.ap,4 => 245f32 + ctx.ap,5 => 300f32 + ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 80f32 + 0.7f32 * ctx.ap,2 => 130f32 + 0.7f32 * ctx.ap,3 => 180f32 + 0.7f32 * ctx.ap,4 => 230f32 + 0.7f32 * ctx.ap,5 => 280f32 + 0.7f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 0.3f32,2 => 0.35f32,3 => 0.4f32,4 => 0.45f32,5 => 0.5f32,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 20f32 + 0.3f32 * ctx.ap + 0.025f32 * ctx.enemy_max_health + 0.005f32 * ctx.chogath_stacks,2 => 40f32 + 0.3f32 * ctx.ap + 0.0285f32 * ctx.enemy_max_health + 0.005f32 * ctx.chogath_stacks,3 => 60f32 + 0.3f32 * ctx.ap + 0.032f32 * ctx.enemy_max_health + 0.005f32 * ctx.chogath_stacks,4 => 80f32 + 0.3f32 * ctx.ap + 0.0355f32 * ctx.enemy_max_health + 0.005f32 * ctx.chogath_stacks,5 => 100f32 + 0.3f32 * ctx.ap + 0.039f32 * ctx.enemy_max_health + 0.005f32 * ctx.chogath_stacks,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 300f32 + 0.5f32 * ctx.ap + 0.1f32 * ctx.bonus_health,2 => 475f32 + 0.5f32 * ctx.ap + 0.1f32 * ctx.bonus_health,3 => 650f32 + 0.5f32 * ctx.ap + 0.1f32 * ctx.bonus_health,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 1200f32 + 0.5f32 * ctx.ap + 0.1f32 * ctx.bonus_health,2 => 1200f32 + 0.5f32 * ctx.ap + 0.1f32 * ctx.bonus_health,3 => 1200f32 + 0.5f32 * ctx.ap + 0.1f32 * ctx.bonus_health,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:644f32,per_level:94f32},mana:CachedChampionStatsMap{flat:270f32,per_level:60f32},armor:CachedChampionStatsMap{flat:38f32,per_level:4.5f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:69f32,per_level:4.2f32},attack_speed:CachedChampionStatsMap{flat:0.658f32,per_level:1.44f32},movespeed:345f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:125f32,aram_damage_taken:1.1f32,aram_damage_dealt:1f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static CORKI: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Middle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_2Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_2Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 60f32 + 1.25f32 * ctx.bonus_ad + ctx.ap,2 => 105f32 + 1.25f32 * ctx.bonus_ad + ctx.ap,3 => 150f32 + 1.25f32 * ctx.bonus_ad + ctx.ap,4 => 195f32 + 1.25f32 * ctx.bonus_ad + ctx.ap,5 => 240f32 + 1.25f32 * ctx.bonus_ad + ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 200f32 + 2f32 * ctx.bonus_ad + 1.5f32 * ctx.ap,2 => 275f32 + 2f32 * ctx.bonus_ad + 1.5f32 * ctx.ap,3 => 350f32 + 2f32 * ctx.bonus_ad + 1.5f32 * ctx.ap,4 => 425f32 + 2f32 * ctx.bonus_ad + 1.5f32 * ctx.ap,5 => 500f32 + 2f32 * ctx.bonus_ad + 1.5f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 40f32 + 0.4f32 * ctx.bonus_ad + 0.3f32 * ctx.ap,2 => 55f32 + 0.4f32 * ctx.bonus_ad + 0.3f32 * ctx.ap,3 => 70f32 + 0.4f32 * ctx.bonus_ad + 0.3f32 * ctx.ap,4 => 85f32 + 0.4f32 * ctx.bonus_ad + 0.3f32 * ctx.ap,5 => 100f32 + 0.4f32 * ctx.bonus_ad + 0.3f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 3f32,2 => 3.5f32,3 => 4f32,4 => 4.5f32,5 => 5f32,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 5f32 + 0.15f32 * ctx.bonus_ad,2 => 8.125f32 + 0.15f32 * ctx.bonus_ad,3 => 11.25f32 + 0.15f32 * ctx.bonus_ad,4 => 14.375f32 + 0.15f32 * ctx.bonus_ad,5 => 17.5f32 + 0.15f32 * ctx.bonus_ad,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:610f32,per_level:100f32},mana:CachedChampionStatsMap{flat:350f32,per_level:40f32},armor:CachedChampionStatsMap{flat:27f32,per_level:4.5f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:52f32,per_level:2f32},attack_speed:CachedChampionStatsMap{flat:0.644f32,per_level:2.8f32},movespeed:325f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.644f32,attack_range:550f32,aram_damage_taken:0.9f32,aram_damage_dealt:1f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static DARIUS: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::Void), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1), 
                damage_type: DamageType::True, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_2), 
                damage_type: DamageType::True, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Max), 
                damage_type: DamageType::True, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 17.5f32 + 0.35f32 * ctx.ad,2 => 28f32 + 0.385f32 * ctx.ad,3 => 38.5f32 + 0.42f32 * ctx.ad,4 => 49f32 + 0.455f32 * ctx.ad,5 => 59.5f32 + 0.49f32 * ctx.ad,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 50f32 + ctx.ad,2 => 80f32 + 1.1f32 * ctx.ad,3 => 110f32 + 1.2f32 * ctx.ad,4 => 140f32 + 1.3f32 * ctx.ad,5 => 170f32 + 1.4f32 * ctx.ad,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 0.4f32 * ctx.ad,2 => 0.45f32 * ctx.ad,3 => 0.5f32 * ctx.ad,4 => 0.55f32 * ctx.ad,5 => 0.6f32 * ctx.ad,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 125f32 + 0.75f32 * ctx.bonus_ad,2 => 250f32 + 0.75f32 * ctx.bonus_ad,3 => 375f32 + 0.75f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 250f32 + 1.5f32 * ctx.bonus_ad,2 => 500f32 + 1.5f32 * ctx.bonus_ad,3 => 750f32 + 1.5f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 25f32 + 0.15f32 * ctx.bonus_ad,2 => 50f32 + 0.15f32 * ctx.bonus_ad,3 => 75f32 + 0.15f32 * ctx.bonus_ad,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:652f32,per_level:114f32},mana:CachedChampionStatsMap{flat:263f32,per_level:58f32},armor:CachedChampionStatsMap{flat:37f32,per_level:5.2f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:64f32,per_level:5f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:1f32},movespeed:340f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:175f32,aram_damage_taken:1f32,aram_damage_dealt:1.05f32,urf_damage_taken:0.85f32,urf_damage_dealt:1.05f32,},
                    merge_data: &[],
                };pub static DIANA: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Jungle,Position::Middle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_2Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_3Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_2Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 70f32 + 0.7f32 * ctx.ap,2 => 105f32 + 0.7f32 * ctx.ap,3 => 140f32 + 0.7f32 * ctx.ap,4 => 175f32 + 0.7f32 * ctx.ap,5 => 210f32 + 0.7f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 60f32 + 0.54f32 * ctx.ap,2 => 96f32 + 0.54f32 * ctx.ap,3 => 132f32 + 0.54f32 * ctx.ap,4 => 168f32 + 0.54f32 * ctx.ap,5 => 204f32 + 0.54f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 45f32 + 0.3f32 * ctx.ap + 0.09f32 * ctx.bonus_health,2 => 60f32 + 0.3f32 * ctx.ap + 0.09f32 * ctx.bonus_health,3 => 75f32 + 0.3f32 * ctx.ap + 0.09f32 * ctx.bonus_health,4 => 90f32 + 0.3f32 * ctx.ap + 0.09f32 * ctx.bonus_health,5 => 105f32 + 0.3f32 * ctx.ap + 0.09f32 * ctx.bonus_health,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 50f32 + 0.6f32 * ctx.ap,2 => 70f32 + 0.6f32 * ctx.ap,3 => 90f32 + 0.6f32 * ctx.ap,4 => 110f32 + 0.6f32 * ctx.ap,5 => 130f32 + 0.6f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 200f32 + 0.6f32 * ctx.ap,2 => 300f32 + 0.6f32 * ctx.ap,3 => 400f32 + 0.6f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 340f32 + 1.2f32 * ctx.ap,2 => 540f32 + 1.2f32 * ctx.ap,3 => 740f32 + 1.2f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 35f32 + 0.15f32 * ctx.ap,2 => 60f32 + 0.15f32 * ctx.ap,3 => 85f32 + 0.15f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:640f32,per_level:109f32},mana:CachedChampionStatsMap{flat:375f32,per_level:25f32},armor:CachedChampionStatsMap{flat:31f32,per_level:4.3f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:57f32,per_level:3f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:2f32},movespeed:345f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.694f32,attack_range:150f32,aram_damage_taken:1f32,aram_damage_dealt:1f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static DRMUNDO: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_2Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_3Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 0.2f32 * ctx.enemy_current_health,2 => 0.225f32 * ctx.enemy_current_health,3 => 0.25f32 * ctx.enemy_current_health,4 => 0.275f32 * ctx.enemy_current_health,5 => 0.3f32 * ctx.enemy_current_health,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 80f32,2 => 140f32,3 => 200f32,4 => 260f32,5 => 320f32,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 5f32,2 => 8.75f32,3 => 12.5f32,4 => 16.25f32,5 => 20f32,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 20f32 + 0.07f32 * ctx.bonus_health,2 => 35f32 + 0.07f32 * ctx.bonus_health,3 => 50f32 + 0.07f32 * ctx.bonus_health,4 => 65f32 + 0.07f32 * ctx.bonus_health,5 => 80f32 + 0.07f32 * ctx.bonus_health,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 0.02f32 * ctx.max_health,2 => 0.022000000000000002f32 * ctx.max_health,3 => 0.024f32 * ctx.max_health,4 => 0.026000000000000002f32 * ctx.max_health,5 => 0.027999999999999997f32 * ctx.max_health,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:640f32,per_level:103f32},mana:CachedChampionStatsMap{flat:0f32,per_level:0f32},armor:CachedChampionStatsMap{flat:32f32,per_level:3.7f32},magic_resist:CachedChampionStatsMap{flat:29f32,per_level:2.3f32},attack_damage:CachedChampionStatsMap{flat:61f32,per_level:2.5f32},attack_speed:CachedChampionStatsMap{flat:0.67f32,per_level:3.3f32},movespeed:345f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:125f32,aram_damage_taken:1.05f32,aram_damage_dealt:0.9f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static DRAVEN: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Bottom],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_2Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 40f32 + 0.75f32 * ctx.bonus_ad,2 => 45f32 + 0.85f32 * ctx.bonus_ad,3 => 50f32 + 0.95f32 * ctx.bonus_ad,4 => 55f32 + 1.05f32 * ctx.bonus_ad,5 => 60f32 + 1.15f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 75f32 + 0.5f32 * ctx.bonus_ad,2 => 110f32 + 0.5f32 * ctx.bonus_ad,3 => 145f32 + 0.5f32 * ctx.bonus_ad,4 => 180f32 + 0.5f32 * ctx.bonus_ad,5 => 215f32 + 0.5f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 400f32 + 2.2f32 * ctx.bonus_ad,2 => 600f32 + 2.6f32 * ctx.bonus_ad,3 => 800f32 + 3f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 200f32 + 1.1f32 * ctx.bonus_ad,2 => 300f32 + 1.3f32 * ctx.bonus_ad,3 => 400f32 + 1.5f32 * ctx.bonus_ad,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:675f32,per_level:104f32},mana:CachedChampionStatsMap{flat:361f32,per_level:39f32},armor:CachedChampionStatsMap{flat:29f32,per_level:4.5f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:62f32,per_level:3.6f32},attack_speed:CachedChampionStatsMap{flat:0.679f32,per_level:2.7f32},movespeed:330f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.67900002002716f32,attack_range:550f32,aram_damage_taken:0.95f32,aram_damage_dealt:1.05f32,urf_damage_taken:1.05f32,urf_damage_dealt:0.95f32,},
                    merge_data: &[],
                };pub static EKKO: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Jungle,Position::Middle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_2Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 80f32 + 0.3f32 * ctx.ap,2 => 95f32 + 0.3f32 * ctx.ap,3 => 110f32 + 0.3f32 * ctx.ap,4 => 125f32 + 0.3f32 * ctx.ap,5 => 140f32 + 0.3f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 0.4f32,2 => 0.45f32,3 => 0.5f32,4 => 0.55f32,5 => 0.6f32,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 50f32 + 0.4f32 * ctx.ap,2 => 75f32 + 0.4f32 * ctx.ap,3 => 100f32 + 0.4f32 * ctx.ap,4 => 125f32 + 0.4f32 * ctx.ap,5 => 150f32 + 0.4f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:655f32,per_level:99f32},mana:CachedChampionStatsMap{flat:280f32,per_level:70f32},armor:CachedChampionStatsMap{flat:32f32,per_level:4.2f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:58f32,per_level:3f32},attack_speed:CachedChampionStatsMap{flat:0.688f32,per_level:3.3f32},movespeed:340f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:125f32,aram_damage_taken:1f32,aram_damage_dealt:1.1f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static ELISE: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Jungle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_2Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Monster2), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 75f32,2 => 100f32,3 => 125f32,4 => 150f32,5 => 175f32,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 40f32 + 0.04f32 * ctx.enemy_current_health + 0.03f32 * 0.01f32 * ctx.ap,2 => 70f32 + 0.04f32 * ctx.enemy_current_health + 0.03f32 * 0.01f32 * ctx.ap,3 => 100f32 + 0.04f32 * ctx.enemy_current_health + 0.03f32 * 0.01f32 * ctx.ap,4 => 130f32 + 0.04f32 * ctx.enemy_current_health + 0.03f32 * 0.01f32 * ctx.ap,5 => 160f32 + 0.04f32 * ctx.enemy_current_health + 0.03f32 * 0.01f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 60f32 + 0.75f32 * ctx.ap,2 => 100f32 + 0.75f32 * ctx.ap,3 => 140f32 + 0.75f32 * ctx.ap,4 => 180f32 + 0.75f32 * ctx.ap,5 => 220f32 + 0.75f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:620f32,per_level:109f32},mana:CachedChampionStatsMap{flat:324f32,per_level:50f32},armor:CachedChampionStatsMap{flat:30f32,per_level:4.5f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:55f32,per_level:3f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:1.75f32},movespeed:330f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:550f32,aram_damage_taken:0.9f32,aram_damage_dealt:1.05f32,urf_damage_taken:0.95f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static EVELYNN: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Jungle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_2Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_2Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 15f32 + 0.25f32 * ctx.ap,2 => 25f32 + 0.25f32 * ctx.ap,3 => 35f32 + 0.25f32 * ctx.ap,4 => 45f32 + 0.25f32 * ctx.ap,5 => 55f32 + 0.25f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 0.0f32,2 => 0.0f32,3 => 0.0f32,4 => 0.0f32,5 => 0.0f32,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 60f32 + 0.03f32 * ctx.enemy_max_health + 0.015f32 * 0.01f32 * ctx.ap,2 => 90f32 + 0.03f32 * ctx.enemy_max_health + 0.015f32 * 0.01f32 * ctx.ap,3 => 120f32 + 0.03f32 * ctx.enemy_max_health + 0.015f32 * 0.01f32 * ctx.ap,4 => 150f32 + 0.03f32 * ctx.enemy_max_health + 0.015f32 * 0.01f32 * ctx.ap,5 => 180f32 + 0.03f32 * ctx.enemy_max_health + 0.015f32 * 0.01f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 125f32 + 0.75f32 * ctx.ap,2 => 250f32 + 0.75f32 * ctx.ap,3 => 375f32 + 0.75f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 300f32 + 1.8f32 * ctx.ap,2 => 600f32 + 1.8f32 * ctx.ap,3 => 900f32 + 1.8f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:642f32,per_level:98f32},mana:CachedChampionStatsMap{flat:315f32,per_level:42f32},armor:CachedChampionStatsMap{flat:37f32,per_level:4.7f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:61f32,per_level:3f32},attack_speed:CachedChampionStatsMap{flat:0.667f32,per_level:2.1f32},movespeed:335f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.666999995708465f32,attack_range:125f32,aram_damage_taken:0.9f32,aram_damage_dealt:1.1f32,urf_damage_taken:0.95f32,urf_damage_dealt:1.05f32,},
                    merge_data: &[],
                };pub static EZREAL: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Bottom],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_2Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 20f32 + 1.3f32 * ctx.ad + 0.15f32 * ctx.ap,2 => 45f32 + 1.3f32 * ctx.ad + 0.15f32 * ctx.ap,3 => 70f32 + 1.3f32 * ctx.ad + 0.15f32 * ctx.ap,4 => 95f32 + 1.3f32 * ctx.ad + 0.15f32 * ctx.ap,5 => 120f32 + 1.3f32 * ctx.ad + 0.15f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 80f32 + ctx.bonus_ad + 0.7f32 * ctx.ap,2 => 135f32 + ctx.bonus_ad + 0.75f32 * ctx.ap,3 => 190f32 + ctx.bonus_ad + 0.8f32 * ctx.ap,4 => 245f32 + ctx.bonus_ad + 0.85f32 * ctx.ap,5 => 300f32 + ctx.bonus_ad + 0.9f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 80f32 + 0.5f32 * ctx.bonus_ad + 0.75f32 * ctx.ap,2 => 130f32 + 0.5f32 * ctx.bonus_ad + 0.75f32 * ctx.ap,3 => 180f32 + 0.5f32 * ctx.bonus_ad + 0.75f32 * ctx.ap,4 => 230f32 + 0.5f32 * ctx.bonus_ad + 0.75f32 * ctx.ap,5 => 280f32 + 0.5f32 * ctx.bonus_ad + 0.75f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 350f32 + ctx.bonus_ad + 0.9f32 * ctx.ap,2 => 550f32 + ctx.bonus_ad + 0.9f32 * ctx.ap,3 => 750f32 + ctx.bonus_ad + 0.9f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 175f32 + 0.5f32 * ctx.bonus_ad + 0.45f32 * ctx.ap,2 => 275f32 + 0.5f32 * ctx.bonus_ad + 0.45f32 * ctx.ap,3 => 375f32 + 0.5f32 * ctx.bonus_ad + 0.45f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:600f32,per_level:102f32},mana:CachedChampionStatsMap{flat:375f32,per_level:70f32},armor:CachedChampionStatsMap{flat:24f32,per_level:4.2f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:60f32,per_level:2.75f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:2.5f32},movespeed:325f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:550f32,aram_damage_taken:1f32,aram_damage_dealt:1f32,urf_damage_taken:1f32,urf_damage_dealt:0.95f32,},
                    merge_data: &[],
                };pub static FIDDLESTICKS: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Jungle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_2Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_3Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_2Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 0.08f32 * ctx.enemy_current_health + 0.06f32 * 0.01f32 * ctx.ap,2 => 0.09f32 * ctx.enemy_current_health + 0.06f32 * 0.01f32 * ctx.ap,3 => 0.1f32 * ctx.enemy_current_health + 0.06f32 * 0.01f32 * ctx.ap,4 => 0.11f32 * ctx.enemy_current_health + 0.06f32 * 0.01f32 * ctx.ap,5 => 0.12f32 * ctx.enemy_current_health + 0.06f32 * 0.01f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 80f32,2 => 120f32,3 => 160f32,4 => 200f32,5 => 240f32,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 70f32 + 0.5f32 * ctx.ap,2 => 105f32 + 0.5f32 * ctx.ap,3 => 140f32 + 0.5f32 * ctx.ap,4 => 175f32 + 0.5f32 * ctx.ap,5 => 210f32 + 0.5f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 750f32 + 2.5f32 * ctx.ap,2 => 1250f32 + 2.5f32 * ctx.ap,3 => 1750f32 + 2.5f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 37.5f32 + 0.125f32 * ctx.ap,2 => 62.5f32 + 0.125f32 * ctx.ap,3 => 87.5f32 + 0.125f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:650f32,per_level:106f32},mana:CachedChampionStatsMap{flat:500f32,per_level:28f32},armor:CachedChampionStatsMap{flat:34f32,per_level:4.7f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:55f32,per_level:2.65f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:2.11f32},movespeed:335f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:480f32,aram_damage_taken:1.05f32,aram_damage_dealt:0.95f32,urf_damage_taken:0.85f32,urf_damage_dealt:1.1f32,},
                    merge_data: &[],
                };pub static FIORA: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::W(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.w_level {1 => 110f32 + ctx.ap,2 => 150f32 + ctx.ap,3 => 190f32 + ctx.ap,4 => 230f32 + ctx.ap,5 => 270f32 + ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:620f32,per_level:99f32},mana:CachedChampionStatsMap{flat:300f32,per_level:60f32},armor:CachedChampionStatsMap{flat:33f32,per_level:4.7f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:66f32,per_level:3.3f32},attack_speed:CachedChampionStatsMap{flat:0.69f32,per_level:3.2f32},movespeed:345f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.689999997615814f32,attack_range:150f32,aram_damage_taken:0.95f32,aram_damage_dealt:1f32,urf_damage_taken:1.1f32,urf_damage_dealt:0.9f32,},
                    merge_data: &[],
                };pub static FIZZ: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Middle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Unknown, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_2), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_4Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_3Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_2Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 10f32 + 0.55f32 * ctx.ap,2 => 25f32 + 0.55f32 * ctx.ap,3 => 40f32 + 0.55f32 * ctx.ap,4 => 55f32 + 0.55f32 * ctx.ap,5 => 70f32 + 0.55f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 30f32 + 0.25f32 * ctx.ap,2 => 45f32 + 0.25f32 * ctx.ap,3 => 60f32 + 0.25f32 * ctx.ap,4 => 75f32 + 0.25f32 * ctx.ap,5 => 90f32 + 0.25f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 50f32 + 0.45f32 * ctx.ap,2 => 75f32 + 0.45f32 * ctx.ap,3 => 100f32 + 0.45f32 * ctx.ap,4 => 125f32 + 0.45f32 * ctx.ap,5 => 150f32 + 0.45f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 20f32 + 0.3f32 * ctx.ap,2 => 25f32 + 0.3f32 * ctx.ap,3 => 30f32 + 0.3f32 * ctx.ap,4 => 35f32 + 0.3f32 * ctx.ap,5 => 40f32 + 0.3f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 30f32,2 => 40f32,3 => 50f32,4 => 60f32,5 => 70f32,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 80f32 + 0.95f32 * ctx.ap,2 => 130f32 + 0.95f32 * ctx.ap,3 => 180f32 + 0.95f32 * ctx.ap,4 => 230f32 + 0.95f32 * ctx.ap,5 => 280f32 + 0.95f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 180f32 + 0.6f32 * ctx.ap,2 => 300f32 + 0.6f32 * ctx.ap,3 => 420f32 + 0.6f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 270f32 + 0.9f32 * ctx.ap,2 => 450f32 + 0.9f32 * ctx.ap,3 => 630f32 + 0.9f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:640f32,per_level:106f32},mana:CachedChampionStatsMap{flat:317f32,per_level:52f32},armor:CachedChampionStatsMap{flat:22f32,per_level:4.6f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:58f32,per_level:3f32},attack_speed:CachedChampionStatsMap{flat:0.658f32,per_level:3.1f32},movespeed:335f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.657999992370605f32,attack_range:175f32,aram_damage_taken:0.95f32,aram_damage_dealt:1.05f32,urf_damage_taken:1.1f32,urf_damage_dealt:0.9f32,},
                    merge_data: &[],
                };pub static GALIO: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Middle,Position::Support],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_2Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 70f32 + 0.7f32 * ctx.ap,2 => 105f32 + 0.7f32 * ctx.ap,3 => 140f32 + 0.7f32 * ctx.ap,4 => 175f32 + 0.7f32 * ctx.ap,5 => 210f32 + 0.7f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 0.075f32 * ctx.max_health,2 => 0.09f32 * ctx.max_health,3 => 0.105f32 * ctx.max_health,4 => 0.12f32 * ctx.max_health,5 => 0.135f32 * ctx.max_health,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 90f32 + 0.9f32 * ctx.ap,2 => 130f32 + 0.9f32 * ctx.ap,3 => 170f32 + 0.9f32 * ctx.ap,4 => 210f32 + 0.9f32 * ctx.ap,5 => 250f32 + 0.9f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 72f32 + 0.72f32 * ctx.ap,2 => 104f32 + 0.72f32 * ctx.ap,3 => 136f32 + 0.72f32 * ctx.ap,4 => 168f32 + 0.72f32 * ctx.ap,5 => 200f32 + 0.72f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 150f32 + 0.7f32 * ctx.ap,2 => 250f32 + 0.7f32 * ctx.ap,3 => 350f32 + 0.7f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:600f32,per_level:126f32},mana:CachedChampionStatsMap{flat:410f32,per_level:40f32},armor:CachedChampionStatsMap{flat:24f32,per_level:4.7f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:59f32,per_level:3.5f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:1.5f32},movespeed:340f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:150f32,aram_damage_taken:1.05f32,aram_damage_dealt:0.9f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static GANGPLANK: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_3Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_6Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_7Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_2Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 10f32 + ctx.ad,2 => 40f32 + ctx.ad,3 => 70f32 + ctx.ad,4 => 100f32 + ctx.ad,5 => 130f32 + ctx.ad,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 3f32,2 => 3f32,3 => 4f32,4 => 4f32,5 => 5f32,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 480f32 + 1.2f32 * ctx.ap,2 => 840f32 + 1.2f32 * ctx.ap,3 => 1200f32 + 1.2f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 120f32 + 0.3f32 * ctx.ap,2 => 210f32 + 0.3f32 * ctx.ap,3 => 300f32 + 0.3f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 600f32 + 1.5f32 * ctx.ap,2 => 1050f32 + 1.5f32 * ctx.ap,3 => 1500f32 + 1.5f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 40f32 + 0.1f32 * ctx.ap,2 => 70f32 + 0.1f32 * ctx.ap,3 => 100f32 + 0.1f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 120f32 + 0.3f32 * ctx.ap,2 => 210f32 + 0.3f32 * ctx.ap,3 => 300f32 + 0.3f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:630f32,per_level:114f32},mana:CachedChampionStatsMap{flat:280f32,per_level:60f32},armor:CachedChampionStatsMap{flat:31f32,per_level:4.7f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:64f32,per_level:4.2f32},attack_speed:CachedChampionStatsMap{flat:0.658f32,per_level:3.2f32},movespeed:345f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.69f32,attack_range:125f32,aram_damage_taken:1f32,aram_damage_dealt:1.05f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static GAREN: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Void), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::Void), 
                damage_type: DamageType::True, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 30f32 + 0.5f32 * ctx.ad,2 => 60f32 + 0.5f32 * ctx.ad,3 => 90f32 + 0.5f32 * ctx.ad,4 => 120f32 + 0.5f32 * ctx.ad,5 => 150f32 + 0.5f32 * ctx.ad,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 4f32 + 0.38f32 * ctx.ad,2 => 7f32 + 0.41f32 * ctx.ad,3 => 10f32 + 0.44f32 * ctx.ad,4 => 13f32 + 0.47f32 * ctx.ad,5 => 16f32 + 0.5f32 * ctx.ad,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 150f32 + 0.25f32 * ctx.enemy_missing_health,2 => 250f32 + 0.3f32 * ctx.enemy_missing_health,3 => 350f32 + 0.35f32 * ctx.enemy_missing_health,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:690f32,per_level:98f32},mana:CachedChampionStatsMap{flat:0f32,per_level:0f32},armor:CachedChampionStatsMap{flat:38f32,per_level:4.2f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:1.55f32},attack_damage:CachedChampionStatsMap{flat:69f32,per_level:4.5f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:3.65f32},movespeed:340f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:175f32,aram_damage_taken:0.95f32,aram_damage_dealt:1f32,urf_damage_taken:0.95f32,urf_damage_dealt:1.1f32,},
                    merge_data: &[],
                };pub static GNAR: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Mega), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::Mega), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 5f32 + 1.25f32 * ctx.ad,2 => 45f32 + 1.25f32 * ctx.ad,3 => 85f32 + 1.25f32 * ctx.ad,4 => 125f32 + 1.25f32 * ctx.ad,5 => 165f32 + 1.25f32 * ctx.ad,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 2.5f32 + 0.625f32 * ctx.ad,2 => 22.5f32 + 0.625f32 * ctx.ad,3 => 42.5f32 + 0.625f32 * ctx.ad,4 => 62.5f32 + 0.625f32 * ctx.ad,5 => 82.5f32 + 0.625f32 * ctx.ad,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 0.4f32,2 => 0.45f32,3 => 0.5f32,4 => 0.55f32,5 => 0.6f32,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 0.4f32,2 => 0.6f32,3 => 0.8f32,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 1.25f32,2 => 1.5f32,3 => 1.75f32,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:540f32,per_level:79f32},mana:CachedChampionStatsMap{flat:100f32,per_level:0f32},armor:CachedChampionStatsMap{flat:32f32,per_level:3.7f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:60f32,per_level:3.2f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:6f32},movespeed:335f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:175f32,aram_damage_taken:0.95f32,aram_damage_dealt:1.05f32,urf_damage_taken:0.9f32,urf_damage_dealt:1.1f32,},
                    merge_data: &[],
                };pub static GRAGAS: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Jungle,Position::Middle,Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_2Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Minion1), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Minion2), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_2), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::Monster1), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 80f32 + 0.8f32 * ctx.ap,2 => 120f32 + 0.8f32 * ctx.ap,3 => 160f32 + 0.8f32 * ctx.ap,4 => 200f32 + 0.8f32 * ctx.ap,5 => 240f32 + 0.8f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 120f32 + 1.2f32 * ctx.ap,2 => 180f32 + 1.2f32 * ctx.ap,3 => 240f32 + 1.2f32 * ctx.ap,4 => 300f32 + 1.2f32 * ctx.ap,5 => 360f32 + 1.2f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 84f32 + 0.84f32 * ctx.ap,2 => 126f32 + 0.84f32 * ctx.ap,3 => 168f32 + 0.84f32 * ctx.ap,4 => 210f32 + 0.84f32 * ctx.ap,5 => 252f32 + 0.84f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 0.4f32,2 => 0.45f32,3 => 0.5f32,4 => 0.55f32,5 => 0.6f32,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 20f32 + 0.07f32 * ctx.enemy_max_health + 0.7f32 * ctx.ap,2 => 50f32 + 0.07f32 * ctx.enemy_max_health + 0.7f32 * ctx.ap,3 => 80f32 + 0.07f32 * ctx.enemy_max_health + 0.7f32 * ctx.ap,4 => 110f32 + 0.07f32 * ctx.enemy_max_health + 0.7f32 * ctx.ap,5 => 140f32 + 0.07f32 * ctx.enemy_max_health + 0.7f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 0.1f32 + 0.04f32 * 0.01f32 * ctx.ap,2 => 0.12f32 + 0.04f32 * 0.01f32 * ctx.ap,3 => 0.14f32 + 0.04f32 * 0.01f32 * ctx.ap,4 => 0.16f32 + 0.04f32 * 0.01f32 * ctx.ap,5 => 0.18f32 + 0.04f32 * 0.01f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 320f32 + 0.7f32 * ctx.ap,2 => 350f32 + 0.7f32 * ctx.ap,3 => 380f32 + 0.7f32 * ctx.ap,4 => 410f32 + 0.7f32 * ctx.ap,5 => 440f32 + 0.7f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 80f32 + 0.6f32 * ctx.ap,2 => 125f32 + 0.6f32 * ctx.ap,3 => 170f32 + 0.6f32 * ctx.ap,4 => 215f32 + 0.6f32 * ctx.ap,5 => 260f32 + 0.6f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 200f32 + 0.8f32 * ctx.ap,2 => 300f32 + 0.8f32 * ctx.ap,3 => 400f32 + 0.8f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:640f32,per_level:115f32},mana:CachedChampionStatsMap{flat:400f32,per_level:47f32},armor:CachedChampionStatsMap{flat:38f32,per_level:5f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:64f32,per_level:3.5f32},attack_speed:CachedChampionStatsMap{flat:0.675f32,per_level:2.05f32},movespeed:330f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:125f32,aram_damage_taken:0.95f32,aram_damage_dealt:1f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static GRAVES: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Jungle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_3Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_2Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_2Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 130f32 + 1.2f32 * ctx.bonus_ad,2 => 200f32 + 1.35f32 * ctx.bonus_ad,3 => 270f32 + 1.5f32 * ctx.bonus_ad,4 => 340f32 + 1.65f32 * ctx.bonus_ad,5 => 410f32 + 1.8f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 50f32 + 0.65f32 * ctx.bonus_ad,2 => 75f32 + 0.65f32 * ctx.bonus_ad,3 => 100f32 + 0.65f32 * ctx.bonus_ad,4 => 125f32 + 0.65f32 * ctx.bonus_ad,5 => 150f32 + 0.65f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 80f32 + 0.55f32 * ctx.bonus_ad,2 => 125f32 + 0.7f32 * ctx.bonus_ad,3 => 170f32 + 0.85f32 * ctx.bonus_ad,4 => 215f32 + ctx.bonus_ad,5 => 260f32 + 1.15f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 60f32 + 0.6f32 * ctx.ap,2 => 110f32 + 0.6f32 * ctx.ap,3 => 160f32 + 0.6f32 * ctx.ap,4 => 210f32 + 0.6f32 * ctx.ap,5 => 260f32 + 0.6f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 275f32 + 1.5f32 * ctx.bonus_ad,2 => 425f32 + 1.5f32 * ctx.bonus_ad,3 => 575f32 + 1.5f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 200f32 + 1.2f32 * ctx.bonus_ad,2 => 320f32 + 1.2f32 * ctx.bonus_ad,3 => 440f32 + 1.2f32 * ctx.bonus_ad,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:625f32,per_level:106f32},mana:CachedChampionStatsMap{flat:325f32,per_level:40f32},armor:CachedChampionStatsMap{flat:33f32,per_level:4.6f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:68f32,per_level:4f32},attack_speed:CachedChampionStatsMap{flat:0.475f32,per_level:3f32},movespeed:340f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.490000009536743f32,attack_range:425f32,aram_damage_taken:1f32,aram_damage_dealt:1f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static GWEN: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Jungle,Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_5Max), 
                damage_type: DamageType::Unknown, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_6Max), 
                damage_type: DamageType::Unknown, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Minion1), 
                damage_type: DamageType::Unknown, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Minion2), 
                damage_type: DamageType::Unknown, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_2Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 10f32 + 0.02f32 * ctx.ap,2 => 15f32 + 0.02f32 * ctx.ap,3 => 20f32 + 0.02f32 * ctx.ap,4 => 25f32 + 0.02f32 * ctx.ap,5 => 30f32 + 0.02f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 10f32 + 0.02f32 * ctx.ap + 0.01f32 * ctx.enemy_max_health + 0.0055000000000000005f32 * 0.01f32 * ctx.ap,2 => 15f32 + 0.02f32 * ctx.ap + 0.01f32 * ctx.enemy_max_health + 0.0055000000000000005f32 * 0.01f32 * ctx.ap,3 => 20f32 + 0.02f32 * ctx.ap + 0.01f32 * ctx.enemy_max_health + 0.0055000000000000005f32 * 0.01f32 * ctx.ap,4 => 25f32 + 0.02f32 * ctx.ap + 0.01f32 * ctx.enemy_max_health + 0.0055000000000000005f32 * 0.01f32 * ctx.ap,5 => 30f32 + 0.02f32 * ctx.ap + 0.01f32 * ctx.enemy_max_health + 0.0055000000000000005f32 * 0.01f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 60f32 + 0.35f32 * ctx.ap,2 => 85f32 + 0.35f32 * ctx.ap,3 => 110f32 + 0.35f32 * ctx.ap,4 => 135f32 + 0.35f32 * ctx.ap,5 => 160f32 + 0.35f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 60f32 + 0.35f32 * ctx.ap + 0.01f32 * ctx.enemy_max_health + 0.0055000000000000005f32 * 0.01f32 * ctx.ap,2 => 85f32 + 0.35f32 * ctx.ap + 0.01f32 * ctx.enemy_max_health + 0.0055000000000000005f32 * 0.01f32 * ctx.ap,3 => 110f32 + 0.35f32 * ctx.ap + 0.01f32 * ctx.enemy_max_health + 0.0055000000000000005f32 * 0.01f32 * ctx.ap,4 => 135f32 + 0.35f32 * ctx.ap + 0.01f32 * ctx.enemy_max_health + 0.0055000000000000005f32 * 0.01f32 * ctx.ap,5 => 160f32 + 0.35f32 * ctx.ap + 0.01f32 * ctx.enemy_max_health + 0.0055000000000000005f32 * 0.01f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 0.2f32,2 => 0.35f32,3 => 0.5f32,4 => 0.65f32,5 => 0.8f32,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 30f32 + 0.08f32 * ctx.ap,2 => 60f32 + 0.08f32 * ctx.ap,3 => 90f32 + 0.08f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 30f32 + 0.08f32 * ctx.ap + 0.01f32 * ctx.enemy_max_health + 0.0055000000000000005f32 * 0.01f32 * ctx.ap,2 => 60f32 + 0.08f32 * ctx.ap + 0.01f32 * ctx.enemy_max_health + 0.0055000000000000005f32 * 0.01f32 * ctx.ap,3 => 90f32 + 0.08f32 * ctx.ap + 0.01f32 * ctx.enemy_max_health + 0.0055000000000000005f32 * 0.01f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:600f32,per_level:110f32},mana:CachedChampionStatsMap{flat:330f32,per_level:40f32},armor:CachedChampionStatsMap{flat:33f32,per_level:4.9f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:63f32,per_level:3f32},attack_speed:CachedChampionStatsMap{flat:0.69f32,per_level:2.5f32},movespeed:340f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.69f32,attack_range:150f32,aram_damage_taken:1f32,aram_damage_dealt:1.02f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static HECARIM: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Jungle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_2Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_2Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 60f32 + 0.9f32 * ctx.bonus_ad,2 => 85f32 + 0.9f32 * ctx.bonus_ad,3 => 110f32 + 0.9f32 * ctx.bonus_ad,4 => 135f32 + 0.9f32 * ctx.bonus_ad,5 => 160f32 + 0.9f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 36f32 + 0.54f32 * ctx.bonus_ad,2 => 51f32 + 0.54f32 * ctx.bonus_ad,3 => 66f32 + 0.54f32 * ctx.bonus_ad,4 => 81f32 + 0.54f32 * ctx.bonus_ad,5 => 96f32 + 0.54f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 100f32 + ctx.ap,2 => 150f32 + ctx.ap,3 => 200f32 + ctx.ap,4 => 250f32 + ctx.ap,5 => 300f32 + ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 20f32 + 0.2f32 * ctx.ap,2 => 30f32 + 0.2f32 * ctx.ap,3 => 40f32 + 0.2f32 * ctx.ap,4 => 50f32 + 0.2f32 * ctx.ap,5 => 60f32 + 0.2f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 150f32 + ctx.ap,2 => 250f32 + ctx.ap,3 => 350f32 + ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:625f32,per_level:106f32},mana:CachedChampionStatsMap{flat:280f32,per_level:40f32},armor:CachedChampionStatsMap{flat:32f32,per_level:5.45f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:66f32,per_level:3.7f32},attack_speed:CachedChampionStatsMap{flat:0.67f32,per_level:2.5f32},movespeed:345f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.6700000166893f32,attack_range:175f32,aram_damage_taken:0.9f32,aram_damage_dealt:1.05f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static HEIMERDINGER: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Middle,Position::Support,Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_2), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_3), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_2Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_3Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_4Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_5Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.w_level {1 => 80f32 + 1.03f32 * ctx.ap,2 => 125f32,3 => 170f32,4 => 215f32,5 => 260f32,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 30f32 + 0.36f32 * ctx.ap,2 => 45f32,3 => 60f32,4 => 75f32,5 => 90f32,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 10f32 + 0.12f32 * ctx.ap,2 => 15f32,3 => 20f32,4 => 25f32,5 => 30f32,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 40f32 + 0.48f32 * ctx.ap,2 => 60f32,3 => 80f32,4 => 100f32,5 => 120f32,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 120f32 + 1.44f32 * ctx.ap,2 => 180f32,3 => 240f32,4 => 300f32,5 => 360f32,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 160f32 + 1.99f32 * ctx.ap,2 => 245f32,3 => 330f32,4 => 415f32,5 => 500f32,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 40f32 + 0.55f32 * ctx.ap,2 => 65f32,3 => 90f32,4 => 115f32,5 => 140f32,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 60f32 + 0.6f32 * ctx.ap,2 => 100f32,3 => 140f32,4 => 180f32,5 => 220f32,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:558f32,per_level:101f32},mana:CachedChampionStatsMap{flat:385f32,per_level:20f32},armor:CachedChampionStatsMap{flat:19f32,per_level:4.2f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:56f32,per_level:2.7f32},attack_speed:CachedChampionStatsMap{flat:0.658f32,per_level:1.36f32},movespeed:340f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:550f32,aram_damage_taken:1.1f32,aram_damage_dealt:0.9f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static HWEI: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Middle,Position::Support],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_2Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.r_level {1 => 30f32 + 0.15f32 * ctx.ap,2 => 60f32 + 0.15f32 * ctx.ap,3 => 90f32 + 0.15f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 2.5f32 + 0.0125f32 * ctx.ap,2 => 5f32 + 0.0125f32 * ctx.ap,3 => 7.5f32 + 0.0125f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:580f32,per_level:109f32},mana:CachedChampionStatsMap{flat:480f32,per_level:30f32},armor:CachedChampionStatsMap{flat:21f32,per_level:4.7f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:54f32,per_level:3.3f32},attack_speed:CachedChampionStatsMap{flat:0.69f32,per_level:2.5f32},movespeed:330f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.658f32,attack_range:550f32,aram_damage_taken:1f32,aram_damage_dealt:0.95f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static ILLAOI: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.r_level {1 => 150f32 + 0.5f32 * ctx.bonus_ad,2 => 250f32 + 0.5f32 * ctx.bonus_ad,3 => 350f32 + 0.5f32 * ctx.bonus_ad,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:656f32,per_level:115f32},mana:CachedChampionStatsMap{flat:350f32,per_level:50f32},armor:CachedChampionStatsMap{flat:35f32,per_level:5f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:65f32,per_level:5f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:2.5f32},movespeed:350f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:125f32,aram_damage_taken:1.05f32,aram_damage_dealt:0.95f32,urf_damage_taken:0.9f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static IRELIA: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Middle,Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 0.09f32 * ctx.ad,2 => 0.1f32 * ctx.ad,3 => 0.11f32 * ctx.ad,4 => 0.12f32 * ctx.ad,5 => 0.13f32 * ctx.ad,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 70f32 + ctx.ap,2 => 110f32 + ctx.ap,3 => 150f32 + ctx.ap,4 => 190f32 + ctx.ap,5 => 230f32 + ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 125f32 + 0.7f32 * ctx.ap,2 => 200f32 + 0.7f32 * ctx.ap,3 => 275f32 + 0.7f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:630f32,per_level:115f32},mana:CachedChampionStatsMap{flat:350f32,per_level:50f32},armor:CachedChampionStatsMap{flat:36f32,per_level:4.7f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:65f32,per_level:3.5f32},attack_speed:CachedChampionStatsMap{flat:0.656f32,per_level:2.5f32},movespeed:335f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.656000018119812f32,attack_range:200f32,aram_damage_taken:1f32,aram_damage_dealt:1.05f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static IVERN: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Jungle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 80f32 + 0.7f32 * ctx.ap,2 => 125f32 + 0.7f32 * ctx.ap,3 => 170f32 + 0.7f32 * ctx.ap,4 => 215f32 + 0.7f32 * ctx.ap,5 => 260f32 + 0.7f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 10f32 + 0.1f32 * ctx.ap,2 => 15f32 + 0.1f32 * ctx.ap,3 => 20f32 + 0.1f32 * ctx.ap,4 => 25f32 + 0.1f32 * ctx.ap,5 => 30f32 + 0.1f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 70f32 + 0.8f32 * ctx.ap,2 => 90f32 + 0.8f32 * ctx.ap,3 => 110f32 + 0.8f32 * ctx.ap,4 => 130f32 + 0.8f32 * ctx.ap,5 => 150f32 + 0.8f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:630f32,per_level:99f32},mana:CachedChampionStatsMap{flat:450f32,per_level:60f32},armor:CachedChampionStatsMap{flat:27f32,per_level:4.7f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:50f32,per_level:3f32},attack_speed:CachedChampionStatsMap{flat:0.644f32,per_level:3.4f32},movespeed:330f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.643999993801116f32,attack_range:475f32,aram_damage_taken:1f32,aram_damage_dealt:0.95f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static JANNA: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Support],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::Void), 
                damage_type: DamageType::Unknown, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 10f32 + 0.1f32 * ctx.ap,2 => 15f32 + 0.1f32 * ctx.ap,3 => 20f32 + 0.1f32 * ctx.ap,4 => 25f32 + 0.1f32 * ctx.ap,5 => 30f32 + 0.1f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 55f32 + 0.5f32 * ctx.ap,2 => 90f32 + 0.5f32 * ctx.ap,3 => 125f32 + 0.5f32 * ctx.ap,4 => 160f32 + 0.5f32 * ctx.ap,5 => 195f32 + 0.5f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 0.06f32 + 0.02f32 * 0.01f32 * ctx.ap,2 => 0.07f32 + 0.02f32 * 0.01f32 * ctx.ap,3 => 0.08f32 + 0.02f32 * 0.01f32 * ctx.ap,4 => 0.09f32 + 0.02f32 * 0.01f32 * ctx.ap,5 => 0.1f32 + 0.02f32 * 0.01f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 10f32 + 0.1f32 * ctx.ap,2 => 15f32 + 0.1f32 * ctx.ap,3 => 20f32 + 0.1f32 * ctx.ap,4 => 25f32 + 0.1f32 * ctx.ap,5 => 30f32 + 0.1f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:570f32,per_level:90f32},mana:CachedChampionStatsMap{flat:360f32,per_level:50f32},armor:CachedChampionStatsMap{flat:28f32,per_level:4.5f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:47f32,per_level:2.5f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:3f32},movespeed:325f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:550f32,aram_damage_taken:1.05f32,aram_damage_dealt:0.95f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static JARVANIV: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Jungle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 0.0f32,2 => 0.0f32,3 => 0.0f32,4 => 0.0f32,5 => 0.0f32,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 0.2f32,2 => 0.225f32,3 => 0.25f32,4 => 0.275f32,5 => 0.3f32,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 200f32 + 1.8f32 * ctx.bonus_ad,2 => 325f32 + 1.8f32 * ctx.bonus_ad,3 => 450f32 + 1.8f32 * ctx.bonus_ad,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:640f32,per_level:104f32},mana:CachedChampionStatsMap{flat:300f32,per_level:55f32},armor:CachedChampionStatsMap{flat:36f32,per_level:4.6f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:64f32,per_level:3f32},attack_speed:CachedChampionStatsMap{flat:0.658f32,per_level:2.5f32},movespeed:340f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.657999992370605f32,attack_range:175f32,aram_damage_taken:1f32,aram_damage_dealt:1f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static JAX: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Jungle,Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::Minion1), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 65f32 + ctx.bonus_ad,2 => 105f32 + ctx.bonus_ad,3 => 145f32 + ctx.bonus_ad,4 => 185f32 + ctx.bonus_ad,5 => 225f32 + ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 50f32 + 0.6f32 * ctx.ap,2 => 85f32 + 0.6f32 * ctx.ap,3 => 120f32 + 0.6f32 * ctx.ap,4 => 155f32 + 0.6f32 * ctx.ap,5 => 190f32 + 0.6f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 40f32 + 0.7f32 * ctx.ap + 0.035f32 * ctx.enemy_max_health,2 => 70f32 + 0.7f32 * ctx.ap + 0.035f32 * ctx.enemy_max_health,3 => 100f32 + 0.7f32 * ctx.ap + 0.035f32 * ctx.enemy_max_health,4 => 130f32 + 0.7f32 * ctx.ap + 0.035f32 * ctx.enemy_max_health,5 => 160f32 + 0.7f32 * ctx.ap + 0.035f32 * ctx.enemy_max_health,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 80f32 + 1.4f32 * ctx.ap + 0.07f32 * ctx.enemy_max_health,2 => 140f32 + 1.4f32 * ctx.ap + 0.07f32 * ctx.enemy_max_health,3 => 200f32 + 1.4f32 * ctx.ap + 0.07f32 * ctx.enemy_max_health,4 => 260f32 + 1.4f32 * ctx.ap + 0.07f32 * ctx.enemy_max_health,5 => 320f32 + 1.4f32 * ctx.ap + 0.07f32 * ctx.enemy_max_health,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:650f32,per_level:103f32},mana:CachedChampionStatsMap{flat:339f32,per_level:52f32},armor:CachedChampionStatsMap{flat:36f32,per_level:4.2f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:68f32,per_level:4.25f32},attack_speed:CachedChampionStatsMap{flat:0.638f32,per_level:3.4f32},movespeed:350f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.638000011444091f32,attack_range:125f32,aram_damage_taken:0.97f32,aram_damage_dealt:1.05f32,urf_damage_taken:1.15f32,urf_damage_dealt:0.9f32,},
                    merge_data: &[],
                };pub static JAYCE: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Middle,Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_2Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_3Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::Monster1), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 60f32 + 1.35f32 * ctx.bonus_ad,2 => 105f32 + 1.35f32 * ctx.bonus_ad,3 => 150f32 + 1.35f32 * ctx.bonus_ad,4 => 195f32 + 1.35f32 * ctx.bonus_ad,5 => 240f32 + 1.35f32 * ctx.bonus_ad,6 => 285f32 + 1.35f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 15f32,2 => 17f32,3 => 19f32,4 => 21f32,5 => 23f32,6 => 25f32,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 0.08f32 * ctx.enemy_max_health + ctx.bonus_ad,2 => 0.10800000000000001f32 * ctx.enemy_max_health + ctx.bonus_ad,3 => 0.136f32 * ctx.enemy_max_health + ctx.bonus_ad,4 => 0.16399999999999998f32 * ctx.enemy_max_health + ctx.bonus_ad,5 => 0.192f32 * ctx.enemy_max_health + ctx.bonus_ad,6 => 0.22f32 * ctx.enemy_max_health + ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 200f32,2 => 300f32,3 => 400f32,4 => 500f32,5 => 600f32,6 => 700f32,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:590f32,per_level:109f32},mana:CachedChampionStatsMap{flat:375f32,per_level:45f32},armor:CachedChampionStatsMap{flat:22f32,per_level:5f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:59f32,per_level:4.25f32},attack_speed:CachedChampionStatsMap{flat:0.658f32,per_level:3f32},movespeed:335f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.657999992370605f32,attack_range:125f32,aram_damage_taken:1f32,aram_damage_dealt:1.05f32,urf_damage_taken:1.05f32,urf_damage_dealt:0.92f32,},
                    merge_data: &[],
                };pub static JHIN: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Bottom],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_2), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_3Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_2Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::Minion1), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::Minion2), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 15.4f32 + 0.154f32 * ctx.ad + 0.21f32 * ctx.ap,2 => 24.15f32 + 0.18025f32 * ctx.ad + 0.21f32 * ctx.ap,3 => 32.9f32 + 0.2065f32 * ctx.ad + 0.21f32 * ctx.ap,4 => 41.65f32 + 0.23274999999999998f32 * ctx.ad + 0.21f32 * ctx.ap,5 => 50.4f32 + 0.259f32 * ctx.ad + 0.21f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 90.2f32 + 0.902f32 * ctx.ad + 1.23f32 * ctx.ap,2 => 141.45f32 + 1.05575f32 * ctx.ad + 1.23f32 * ctx.ap,3 => 192.7f32 + 1.2095f32 * ctx.ad + 1.23f32 * ctx.ap,4 => 243.95f32 + 1.3632499999999999f32 * ctx.ad + 1.23f32 * ctx.ap,5 => 295.2f32 + 1.517f32 * ctx.ad + 1.23f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 44f32 + 0.44f32 * ctx.ad + 0.6f32 * ctx.ap,2 => 69f32 + 0.515f32 * ctx.ad + 0.6f32 * ctx.ap,3 => 94f32 + 0.59f32 * ctx.ad + 0.6f32 * ctx.ap,4 => 119f32 + 0.665f32 * ctx.ad + 0.6f32 * ctx.ap,5 => 144f32 + 0.74f32 * ctx.ad + 0.6f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 64f32 + 0.25f32 * ctx.ad,2 => 128f32 + 0.25f32 * ctx.ad,3 => 192f32 + 0.25f32 * ctx.ad,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 128f32 + 0.5f32 * ctx.ad,2 => 256f32 + 0.5f32 * ctx.ad,3 => 384f32 + 0.5f32 * ctx.ad,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 256f32 + ctx.ad,2 => 512f32 + ctx.ad,3 => 768f32 + ctx.ad,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 512f32 + 2f32 * ctx.ad,2 => 1024f32 + 2f32 * ctx.ad,3 => 1536f32 + 2f32 * ctx.ad,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:655f32,per_level:107f32},mana:CachedChampionStatsMap{flat:300f32,per_level:50f32},armor:CachedChampionStatsMap{flat:24f32,per_level:4.7f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:59f32,per_level:4.4f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:3f32},movespeed:330f32,critical_strike_damage:175f32,critical_strike_damage_modifier:0.86f32,attack_speed_ratio:0f32,attack_range:550f32,aram_damage_taken:1.05f32,aram_damage_dealt:0.95f32,urf_damage_taken:1f32,urf_damage_dealt:1.01f32,},
                    merge_data: &[],
                };pub static JINX: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Bottom],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_2Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::Minion1), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::Minion2), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.w_level {1 => 10f32 + 1.4f32 * ctx.ad,2 => 60f32 + 1.4f32 * ctx.ad,3 => 110f32 + 1.4f32 * ctx.ad,4 => 160f32 + 1.4f32 * ctx.ad,5 => 210f32 + 1.4f32 * ctx.ad,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 90f32 + ctx.ap,2 => 140f32 + ctx.ap,3 => 190f32 + ctx.ap,4 => 240f32 + ctx.ap,5 => 290f32 + ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 300f32 + 1.55f32 * ctx.bonus_ad + 0.25f32 * ctx.enemy_missing_health,2 => 450f32 + 1.55f32 * ctx.bonus_ad + 0.3f32 * ctx.enemy_missing_health,3 => 600f32 + 1.55f32 * ctx.bonus_ad + 0.35f32 * ctx.enemy_missing_health,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 240f32 + 1.24f32 * ctx.bonus_ad + 0.2f32 * ctx.enemy_missing_health,2 => 360f32 + 1.24f32 * ctx.bonus_ad + 0.24f32 * ctx.enemy_missing_health,3 => 480f32 + 1.24f32 * ctx.bonus_ad + 0.28f32 * ctx.enemy_missing_health,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 30f32 + 0.155f32 * ctx.bonus_ad + 0.25f32 * ctx.enemy_missing_health,2 => 45f32 + 0.155f32 * ctx.bonus_ad + 0.3f32 * ctx.enemy_missing_health,3 => 60f32 + 0.155f32 * ctx.bonus_ad + 0.35f32 * ctx.enemy_missing_health,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 24f32 + 0.124f32 * ctx.bonus_ad + 0.2f32 * ctx.enemy_missing_health,2 => 36f32 + 0.124f32 * ctx.bonus_ad + 0.24f32 * ctx.enemy_missing_health,3 => 48f32 + 0.124f32 * ctx.bonus_ad + 0.28f32 * ctx.enemy_missing_health,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:630f32,per_level:105f32},mana:CachedChampionStatsMap{flat:260f32,per_level:50f32},armor:CachedChampionStatsMap{flat:26f32,per_level:4.7f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:59f32,per_level:3.25f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:1.4f32},movespeed:325f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:525f32,aram_damage_taken:1.05f32,aram_damage_dealt:0.9f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static KSANTE: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_2Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 70f32 + 0.4f32 * ctx.bonus_armor + 0.4f32 * ctx.bonus_magic_resist,2 => 100f32 + 0.4f32 * ctx.bonus_armor + 0.4f32 * ctx.bonus_magic_resist,3 => 130f32 + 0.4f32 * ctx.bonus_armor + 0.4f32 * ctx.bonus_magic_resist,4 => 160f32 + 0.4f32 * ctx.bonus_armor + 0.4f32 * ctx.bonus_magic_resist,5 => 190f32 + 0.4f32 * ctx.bonus_armor + 0.4f32 * ctx.bonus_magic_resist,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 80f32,2 => 115f32,3 => 150f32,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 0.4f32,2 => 0.6f32,3 => 0.8f32,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:625f32,per_level:120f32},mana:CachedChampionStatsMap{flat:320f32,per_level:60f32},armor:CachedChampionStatsMap{flat:36f32,per_level:5.2f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:2.1f32},attack_damage:CachedChampionStatsMap{flat:64f32,per_level:3.5f32},attack_speed:CachedChampionStatsMap{flat:0.688f32,per_level:2.5f32},movespeed:330f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:150f32,aram_damage_taken:1f32,aram_damage_dealt:1.05f32,urf_damage_taken:1f32,urf_damage_dealt:1.05f32,},
                    merge_data: &[],
                };pub static KAISA: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Bottom],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_2Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 150f32 + 2.0625f32 * ctx.bonus_ad + 0.75f32 * ctx.ap,2 => 206.25f32 + 2.0625f32 * ctx.bonus_ad + 0.75f32 * ctx.ap,3 => 262.5f32 + 2.0625f32 * ctx.bonus_ad + 0.75f32 * ctx.ap,4 => 318.75f32 + 2.0625f32 * ctx.bonus_ad + 0.75f32 * ctx.ap,5 => 375f32 + 2.0625f32 * ctx.bonus_ad + 0.75f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 10f32 + 0.1375f32 * ctx.bonus_ad + 0.05f32 * ctx.ap,2 => 13.75f32 + 0.1375f32 * ctx.bonus_ad + 0.05f32 * ctx.ap,3 => 17.5f32 + 0.1375f32 * ctx.bonus_ad + 0.05f32 * ctx.ap,4 => 21.25f32 + 0.1375f32 * ctx.bonus_ad + 0.05f32 * ctx.ap,5 => 25f32 + 0.1375f32 * ctx.bonus_ad + 0.05f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 30f32 + 1.3f32 * ctx.ad + 0.45f32 * ctx.ap,2 => 55f32 + 1.3f32 * ctx.ad + 0.45f32 * ctx.ap,3 => 80f32 + 1.3f32 * ctx.ad + 0.45f32 * ctx.ap,4 => 105f32 + 1.3f32 * ctx.ad + 0.45f32 * ctx.ap,5 => 130f32 + 1.3f32 * ctx.ad + 0.45f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:640f32,per_level:102f32},mana:CachedChampionStatsMap{flat:345f32,per_level:40f32},armor:CachedChampionStatsMap{flat:27f32,per_level:4.2f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:59f32,per_level:2.6f32},attack_speed:CachedChampionStatsMap{flat:0.644f32,per_level:1.8f32},movespeed:335f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.643999993801116f32,attack_range:525f32,aram_damage_taken:0.9f32,aram_damage_dealt:1f32,urf_damage_taken:1.1f32,urf_damage_dealt:0.9f32,},
                    merge_data: &[],
                };pub static KALISTA: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Bottom],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Void), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::Void), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 10f32 + 1.05f32 * ctx.ad,2 => 75f32 + 1.05f32 * ctx.ad,3 => 140f32 + 1.05f32 * ctx.ad,4 => 205f32 + 1.05f32 * ctx.ad,5 => 270f32 + 1.05f32 * ctx.ad,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 100f32,2 => 125f32,3 => 150f32,4 => 175f32,5 => 200f32,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 0.1f32 * ctx.enemy_max_health,2 => 0.12f32 * ctx.enemy_max_health,3 => 0.14f32 * ctx.enemy_max_health,4 => 0.16f32 * ctx.enemy_max_health,5 => 0.18f32 * ctx.enemy_max_health,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 7f32 + 0.2f32 * ctx.ad + 0.5f32 * ctx.ap,2 => 14f32 + 0.25f32 * ctx.ad + 0.5f32 * ctx.ap,3 => 21f32 + 0.3f32 * ctx.ad + 0.5f32 * ctx.ap,4 => 28f32 + 0.35f32 * ctx.ad + 0.5f32 * ctx.ap,5 => 35f32 + 0.4f32 * ctx.ad + 0.5f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 5f32 + 0.7f32 * ctx.ad + 0.65f32 * ctx.ap,2 => 15f32 + 0.7f32 * ctx.ad + 0.65f32 * ctx.ap,3 => 25f32 + 0.7f32 * ctx.ad + 0.65f32 * ctx.ap,4 => 35f32 + 0.7f32 * ctx.ad + 0.65f32 * ctx.ap,5 => 45f32 + 0.7f32 * ctx.ad + 0.65f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:560f32,per_level:114f32},mana:CachedChampionStatsMap{flat:300f32,per_level:45f32},armor:CachedChampionStatsMap{flat:24f32,per_level:5.2f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:57f32,per_level:4.25f32},attack_speed:CachedChampionStatsMap{flat:0.694f32,per_level:4.5f32},movespeed:330f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.694000005722045f32,attack_range:525f32,aram_damage_taken:0.9f32,aram_damage_dealt:1.1f32,urf_damage_taken:0.9f32,urf_damage_dealt:1.1f32,},
                    merge_data: &[],
                };pub static KARMA: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Middle,Position::Support,Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_2), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_2Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 60f32 + 0.7f32 * ctx.ap,2 => 110f32,3 => 160f32,4 => 210f32,5 => 260f32,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 0.0f32,2 => 0.0f32,3 => 0.0f32,4 => 0.0f32,5 => 0.0f32,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 40f32 + 0.45f32 * ctx.ap,2 => 65f32,3 => 90f32,4 => 115f32,5 => 140f32,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:630f32,per_level:109f32},mana:CachedChampionStatsMap{flat:374f32,per_level:40f32},armor:CachedChampionStatsMap{flat:28f32,per_level:5f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:51f32,per_level:3.3f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:2.3f32},movespeed:335f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:525f32,aram_damage_taken:1f32,aram_damage_dealt:1f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static KARTHUS: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Jungle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 80f32 + 0.7f32 * ctx.ap,2 => 118f32 + 0.7f32 * ctx.ap,3 => 156f32 + 0.7f32 * ctx.ap,4 => 194f32 + 0.7f32 * ctx.ap,5 => 232f32 + 0.7f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 40f32 + 0.35f32 * ctx.ap,2 => 59f32 + 0.35f32 * ctx.ap,3 => 78f32 + 0.35f32 * ctx.ap,4 => 97f32 + 0.35f32 * ctx.ap,5 => 116f32 + 0.35f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 200f32 + 0.7f32 * ctx.ap,2 => 350f32 + 0.7f32 * ctx.ap,3 => 500f32 + 0.7f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:620f32,per_level:110f32},mana:CachedChampionStatsMap{flat:467f32,per_level:31f32},armor:CachedChampionStatsMap{flat:21f32,per_level:4.7f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:46f32,per_level:3.25f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:2.11f32},movespeed:335f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:450f32,aram_damage_taken:1.05f32,aram_damage_dealt:0.93f32,urf_damage_taken:1f32,urf_damage_dealt:0.9f32,},
                    merge_data: &[],
                };pub static KASSADIN: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Middle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_2), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_3), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 65f32 + 0.7f32 * ctx.ap,2 => 95f32 + 0.7f32 * ctx.ap,3 => 125f32 + 0.7f32 * ctx.ap,4 => 155f32 + 0.7f32 * ctx.ap,5 => 185f32 + 0.7f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 70f32 + 0.5f32 * ctx.ap + 0.02f32 * ctx.max_mana,2 => 90f32 + 0.5f32 * ctx.ap + 0.02f32 * ctx.max_mana,3 => 110f32 + 0.5f32 * ctx.ap + 0.02f32 * ctx.max_mana,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 35f32 + 0.07f32 * ctx.ap + 0.01f32 * ctx.max_mana,2 => 45f32 + 0.07f32 * ctx.ap + 0.01f32 * ctx.max_mana,3 => 55f32 + 0.07f32 * ctx.ap + 0.01f32 * ctx.max_mana,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 140f32 + 0.28f32 * ctx.ap + 0.04f32 * ctx.max_mana,2 => 180f32 + 0.28f32 * ctx.ap + 0.04f32 * ctx.max_mana,3 => 220f32 + 0.28f32 * ctx.ap + 0.04f32 * ctx.max_mana,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 210f32 + 0.78f32 * ctx.ap + 0.06f32 * ctx.max_mana,2 => 270f32 + 0.78f32 * ctx.ap + 0.06f32 * ctx.max_mana,3 => 330f32 + 0.78f32 * ctx.ap + 0.06f32 * ctx.max_mana,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:646f32,per_level:119f32},mana:CachedChampionStatsMap{flat:400f32,per_level:87f32},armor:CachedChampionStatsMap{flat:21f32,per_level:4f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:59f32,per_level:3.9f32},attack_speed:CachedChampionStatsMap{flat:0.64f32,per_level:3.7f32},movespeed:335f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.639999985694885f32,attack_range:150f32,aram_damage_taken:0.95f32,aram_damage_dealt:1f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static KATARINA: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Middle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 80f32 + 0.4f32 * ctx.ap,2 => 115f32 + 0.4f32 * ctx.ap,3 => 150f32 + 0.4f32 * ctx.ap,4 => 185f32 + 0.4f32 * ctx.ap,5 => 220f32 + 0.4f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 20f32 + 0.4f32 * ctx.ad + 0.25f32 * ctx.ap,2 => 30f32 + 0.4f32 * ctx.ad + 0.25f32 * ctx.ap,3 => 40f32 + 0.4f32 * ctx.ad + 0.25f32 * ctx.ap,4 => 50f32 + 0.4f32 * ctx.ad + 0.25f32 * ctx.ap,5 => 60f32 + 0.4f32 * ctx.ad + 0.25f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:672f32,per_level:108f32},mana:CachedChampionStatsMap{flat:0f32,per_level:0f32},armor:CachedChampionStatsMap{flat:32f32,per_level:4.7f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:58f32,per_level:3.2f32},attack_speed:CachedChampionStatsMap{flat:0.658f32,per_level:2.74f32},movespeed:335f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.657999992370605f32,attack_range:125f32,aram_damage_taken:1f32,aram_damage_dealt:1f32,urf_damage_taken:0.95f32,urf_damage_dealt:1.1f32,},
                    merge_data: &[],
                };pub static KAYLE: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 60f32 + 0.6f32 * ctx.bonus_ad + 0.5f32 * ctx.ap,2 => 100f32 + 0.6f32 * ctx.bonus_ad + 0.5f32 * ctx.ap,3 => 140f32 + 0.6f32 * ctx.bonus_ad + 0.5f32 * ctx.ap,4 => 180f32 + 0.6f32 * ctx.bonus_ad + 0.5f32 * ctx.ap,5 => 220f32 + 0.6f32 * ctx.bonus_ad + 0.5f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 15f32 + 0.1f32 * ctx.bonus_ad + 0.2f32 * ctx.ap,2 => 20f32 + 0.1f32 * ctx.bonus_ad + 0.2f32 * ctx.ap,3 => 25f32 + 0.1f32 * ctx.bonus_ad + 0.2f32 * ctx.ap,4 => 30f32 + 0.1f32 * ctx.bonus_ad + 0.2f32 * ctx.ap,5 => 35f32 + 0.1f32 * ctx.bonus_ad + 0.2f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 200f32 + ctx.bonus_ad + 0.7f32 * ctx.ap,2 => 300f32 + ctx.bonus_ad + 0.7f32 * ctx.ap,3 => 400f32 + ctx.bonus_ad + 0.7f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:670f32,per_level:92f32},mana:CachedChampionStatsMap{flat:330f32,per_level:50f32},armor:CachedChampionStatsMap{flat:26f32,per_level:4.2f32},magic_resist:CachedChampionStatsMap{flat:22f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:50f32,per_level:2.5f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:1.5f32},movespeed:335f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.666999995708465f32,attack_range:175f32,aram_damage_taken:1.1f32,aram_damage_dealt:0.95f32,urf_damage_taken:1.1f32,urf_damage_dealt:0.85f32,},
                    merge_data: &[],
                };pub static KAYN: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Jungle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_2Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_4Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_3Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Monster1), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Monster2), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 150f32 + 1.7f32 * ctx.bonus_ad,2 => 200f32 + 1.7f32 * ctx.bonus_ad,3 => 250f32 + 1.7f32 * ctx.bonus_ad,4 => 300f32 + 1.7f32 * ctx.bonus_ad,5 => 350f32 + 1.7f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 230f32 + 1.7f32 * ctx.bonus_ad,2 => 280f32 + 1.7f32 * ctx.bonus_ad,3 => 330f32 + 1.7f32 * ctx.bonus_ad,4 => 380f32 + 1.7f32 * ctx.bonus_ad,5 => 430f32 + 1.7f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 75f32 + 0.85f32 * ctx.bonus_ad,2 => 100f32 + 0.85f32 * ctx.bonus_ad,3 => 125f32 + 0.85f32 * ctx.bonus_ad,4 => 150f32 + 0.85f32 * ctx.bonus_ad,5 => 175f32 + 0.85f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 115f32 + 0.85f32 * ctx.bonus_ad,2 => 140f32 + 0.85f32 * ctx.bonus_ad,3 => 165f32 + 0.85f32 * ctx.bonus_ad,4 => 190f32 + 0.85f32 * ctx.bonus_ad,5 => 215f32 + 0.85f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 200f32,2 => 250f32,3 => 300f32,4 => 350f32,5 => 400f32,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 400f32,2 => 500f32,3 => 600f32,4 => 700f32,5 => 800f32,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 85f32 + 1.1f32 * ctx.bonus_ad,2 => 130f32 + 1.1f32 * ctx.bonus_ad,3 => 175f32 + 1.1f32 * ctx.bonus_ad,4 => 220f32 + 1.1f32 * ctx.bonus_ad,5 => 265f32 + 1.1f32 * ctx.bonus_ad,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:655f32,per_level:103f32},mana:CachedChampionStatsMap{flat:410f32,per_level:50f32},armor:CachedChampionStatsMap{flat:38f32,per_level:4.5f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:68f32,per_level:3.3f32},attack_speed:CachedChampionStatsMap{flat:0.669f32,per_level:2.7f32},movespeed:340f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.669000029563903f32,attack_range:175f32,aram_damage_taken:1f32,aram_damage_dealt:1f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static KENNEN: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_2Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 75f32 + 0.75f32 * ctx.ap,2 => 125f32 + 0.75f32 * ctx.ap,3 => 175f32 + 0.75f32 * ctx.ap,4 => 225f32 + 0.75f32 * ctx.ap,5 => 275f32 + 0.75f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 35f32 + 0.8f32 * ctx.bonus_ad + 0.35f32 * ctx.ap,2 => 45f32 + 0.9f32 * ctx.bonus_ad + 0.35f32 * ctx.ap,3 => 55f32 + ctx.bonus_ad + 0.35f32 * ctx.ap,4 => 65f32 + 1.1f32 * ctx.bonus_ad + 0.35f32 * ctx.ap,5 => 75f32 + 1.2f32 * ctx.bonus_ad + 0.35f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 80f32 + 0.8f32 * ctx.ap,2 => 120f32 + 0.8f32 * ctx.ap,3 => 160f32 + 0.8f32 * ctx.ap,4 => 200f32 + 0.8f32 * ctx.ap,5 => 240f32 + 0.8f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 52f32 + 0.52f32 * ctx.ap,2 => 78f32 + 0.52f32 * ctx.ap,3 => 104f32 + 0.52f32 * ctx.ap,4 => 130f32 + 0.52f32 * ctx.ap,5 => 156f32 + 0.52f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 300f32 + 1.6875f32 * ctx.ap,2 => 562.5f32 + 1.6875f32 * ctx.ap,3 => 825f32 + 1.6875f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:580f32,per_level:98f32},mana:CachedChampionStatsMap{flat:200f32,per_level:0f32},armor:CachedChampionStatsMap{flat:29f32,per_level:4.95f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:48f32,per_level:3.75f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:3.4f32},movespeed:335f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.689999997615814f32,attack_range:550f32,aram_damage_taken:1f32,aram_damage_dealt:1f32,urf_damage_taken:0.95f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static KHAZIX: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Jungle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.w_level {1 => 85f32 + ctx.bonus_ad,2 => 115f32 + ctx.bonus_ad,3 => 145f32 + ctx.bonus_ad,4 => 175f32 + ctx.bonus_ad,5 => 205f32 + ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 65f32 + 0.4f32 * ctx.bonus_ad,2 => 100f32 + 0.4f32 * ctx.bonus_ad,3 => 135f32 + 0.4f32 * ctx.bonus_ad,4 => 170f32 + 0.4f32 * ctx.bonus_ad,5 => 205f32 + 0.4f32 * ctx.bonus_ad,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:643f32,per_level:99f32},mana:CachedChampionStatsMap{flat:327f32,per_level:40f32},armor:CachedChampionStatsMap{flat:32f32,per_level:4.2f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:60f32,per_level:3.1f32},attack_speed:CachedChampionStatsMap{flat:0.668f32,per_level:2.7f32},movespeed:350f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.667999982833862f32,attack_range:125f32,aram_damage_taken:0.9f32,aram_damage_dealt:1.1f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static KINDRED: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Jungle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 40f32 + 0.75f32 * ctx.bonus_ad,2 => 65f32 + 0.75f32 * ctx.bonus_ad,3 => 90f32 + 0.75f32 * ctx.bonus_ad,4 => 115f32 + 0.75f32 * ctx.bonus_ad,5 => 140f32 + 0.75f32 * ctx.bonus_ad,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:595f32,per_level:104f32},mana:CachedChampionStatsMap{flat:300f32,per_level:35f32},armor:CachedChampionStatsMap{flat:29f32,per_level:4.7f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:65f32,per_level:3.25f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:3.5f32},movespeed:325f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:500f32,aram_damage_taken:0.9f32,aram_damage_dealt:1.1f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static KLED: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_4Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_2), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_2Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::Minion1), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 45f32 + 0.9f32 * ctx.bonus_ad,2 => 82.5f32 + 0.9f32 * ctx.bonus_ad,3 => 120f32 + 0.9f32 * ctx.bonus_ad,4 => 157.5f32 + 0.9f32 * ctx.bonus_ad,5 => 195f32 + 0.9f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 30f32 + 0.6f32 * ctx.bonus_ad,2 => 55f32 + 0.6f32 * ctx.bonus_ad,3 => 80f32 + 0.6f32 * ctx.bonus_ad,4 => 105f32 + 0.6f32 * ctx.bonus_ad,5 => 130f32 + 0.6f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 62f32 + 0f32,2 => 63.75f32 + 15f32,3 => 65.5f32 + 30f32,4 => 67.25f32 + 45f32,5 => 69f32 + 60f32,6 => 70.75f32,7 => 72.5f32,8 => 74.25f32,9 => 76f32,10 => 77.75f32,11 => 79.5f32,12 => 81.25f32,13 => 83f32,14 => 84.75f32,15 => 86.5f32,16 => 88.25f32,17 => 90f32,18 => 91.75f32,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 70f32 + 1.1f32 * ctx.bonus_ad,2 => 120f32 + 1.1f32 * ctx.bonus_ad,3 => 170f32 + 1.1f32 * ctx.bonus_ad,4 => 220f32 + 1.1f32 * ctx.bonus_ad,5 => 270f32 + 1.1f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 35f32 + 0.55f32 * ctx.bonus_ad,2 => 60f32 + 0.55f32 * ctx.bonus_ad,3 => 85f32 + 0.55f32 * ctx.bonus_ad,4 => 110f32 + 0.55f32 * ctx.bonus_ad,5 => 135f32 + 0.55f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 20f32 + 0.3f32 * ctx.bonus_ad,2 => 30f32 + 0.3f32 * ctx.bonus_ad,3 => 40f32 + 0.3f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 200f32 + 3f32 * ctx.bonus_ad,2 => 300f32 + 3f32 * ctx.bonus_ad,3 => 400f32 + 3f32 * ctx.bonus_ad,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:410f32,per_level:84f32},mana:CachedChampionStatsMap{flat:100f32,per_level:0f32},armor:CachedChampionStatsMap{flat:35f32,per_level:5.2f32},magic_resist:CachedChampionStatsMap{flat:28f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:65f32,per_level:3.5f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:3.5f32},movespeed:305f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:250f32,aram_damage_taken:1f32,aram_damage_dealt:1.05f32,urf_damage_taken:0.9f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static KOGMAW: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Bottom],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::Minion1), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 0.1f32,2 => 0.15f32,3 => 0.2f32,4 => 0.25f32,5 => 0.3f32,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 0.03f32 * ctx.enemy_max_health + 0.015f32 * 0.01f32 * ctx.ap,2 => 0.0375f32 * ctx.enemy_max_health + 0.015f32 * 0.01f32 * ctx.ap,3 => 0.045f32 * ctx.enemy_max_health + 0.015f32 * 0.01f32 * ctx.ap,4 => 0.0525f32 * ctx.enemy_max_health + 0.015f32 * 0.01f32 * ctx.ap,5 => 0.06f32 * ctx.enemy_max_health + 0.015f32 * 0.01f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 0.4f32,2 => 0.45f32,3 => 0.5f32,4 => 0.55f32,5 => 0.6f32,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 100f32 + 0.75f32 * ctx.bonus_ad + 0.35f32 * ctx.ap,2 => 140f32 + 0.75f32 * ctx.bonus_ad + 0.4f32 * ctx.ap,3 => 180f32 + 0.75f32 * ctx.bonus_ad + 0.45f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 200f32 + 1.5f32 * ctx.bonus_ad + 0.7f32 * ctx.ap,2 => 280f32 + 1.5f32 * ctx.bonus_ad + 0.8f32 * ctx.ap,3 => 360f32 + 1.5f32 * ctx.bonus_ad + 0.9f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:635f32,per_level:99f32},mana:CachedChampionStatsMap{flat:325f32,per_level:40f32},armor:CachedChampionStatsMap{flat:24f32,per_level:4.45f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:61f32,per_level:3.1f32},attack_speed:CachedChampionStatsMap{flat:0.665f32,per_level:2.65f32},movespeed:330f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.665000021457672f32,attack_range:500f32,aram_damage_taken:1.1f32,aram_damage_dealt:0.92f32,urf_damage_taken:1.05f32,urf_damage_dealt:0.85f32,},
                    merge_data: &[],
                };pub static LEBLANC: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Middle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_2Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_3Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_2Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_7Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_2Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_5Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_6Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 130f32 + 0.8f32 * ctx.ap,2 => 180f32 + 0.8f32 * ctx.ap,3 => 230f32 + 0.8f32 * ctx.ap,4 => 280f32 + 0.8f32 * ctx.ap,5 => 330f32 + 0.8f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 65f32 + 0.4f32 * ctx.ap,2 => 90f32 + 0.4f32 * ctx.ap,3 => 115f32 + 0.4f32 * ctx.ap,4 => 140f32 + 0.4f32 * ctx.ap,5 => 165f32 + 0.4f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 75f32 + 0.8f32 * ctx.ap,2 => 115f32 + 0.8f32 * ctx.ap,3 => 155f32 + 0.8f32 * ctx.ap,4 => 195f32 + 0.8f32 * ctx.ap,5 => 235f32 + 0.8f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 130f32 + 1.25f32 * ctx.ap,2 => 190f32 + 1.25f32 * ctx.ap,3 => 250f32 + 1.25f32 * ctx.ap,4 => 310f32 + 1.25f32 * ctx.ap,5 => 370f32 + 1.25f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 50f32 + 0.4f32 * ctx.ap,2 => 70f32 + 0.4f32 * ctx.ap,3 => 90f32 + 0.4f32 * ctx.ap,4 => 110f32 + 0.4f32 * ctx.ap,5 => 130f32 + 0.4f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 80f32 + 0.85f32 * ctx.ap,2 => 120f32 + 0.85f32 * ctx.ap,3 => 160f32 + 0.85f32 * ctx.ap,4 => 200f32 + 0.85f32 * ctx.ap,5 => 240f32 + 0.85f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 210f32 + 1.25f32 * ctx.ap,2 => 420f32 + 1.25f32 * ctx.ap,3 => 630f32 + 1.25f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 70f32 + 0.4f32 * ctx.ap,2 => 140f32 + 0.4f32 * ctx.ap,3 => 210f32 + 0.4f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 150f32 + 0.8f32 * ctx.ap,2 => 300f32 + 0.8f32 * ctx.ap,3 => 450f32 + 0.8f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 70f32 + 0.4f32 * ctx.ap,2 => 140f32 + 0.4f32 * ctx.ap,3 => 210f32 + 0.4f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 140f32 + 0.85f32 * ctx.ap,2 => 280f32 + 0.85f32 * ctx.ap,3 => 420f32 + 0.85f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:598f32,per_level:111f32},mana:CachedChampionStatsMap{flat:400f32,per_level:25f32},armor:CachedChampionStatsMap{flat:22f32,per_level:4.7f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:55f32,per_level:2.2f32},attack_speed:CachedChampionStatsMap{flat:0.658f32,per_level:2.35f32},movespeed:340f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.4f32,attack_range:525f32,aram_damage_taken:0.9f32,aram_damage_dealt:1.1f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static LEESIN: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Jungle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_2Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_2Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 60f32 + 0.95f32 * ctx.bonus_ad,2 => 90f32 + 0.95f32 * ctx.bonus_ad,3 => 120f32 + 0.95f32 * ctx.bonus_ad,4 => 150f32 + 0.95f32 * ctx.bonus_ad,5 => 180f32 + 0.95f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 35f32 + ctx.ad,2 => 60f32 + ctx.ad,3 => 85f32 + ctx.ad,4 => 110f32 + ctx.ad,5 => 135f32 + ctx.ad,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 175f32 + 2f32 * ctx.bonus_ad,2 => 400f32 + 2f32 * ctx.bonus_ad,3 => 625f32 + 2f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 175f32 + 2f32 * ctx.bonus_ad + 0.12f32 * ctx.enemy_bonus_health,2 => 400f32 + 2f32 * ctx.bonus_ad + 0.15f32 * ctx.enemy_bonus_health,3 => 625f32 + 2f32 * ctx.bonus_ad + 0.18f32 * ctx.enemy_bonus_health,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:645f32,per_level:108f32},mana:CachedChampionStatsMap{flat:200f32,per_level:0f32},armor:CachedChampionStatsMap{flat:36f32,per_level:4.5f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:66f32,per_level:3.7f32},attack_speed:CachedChampionStatsMap{flat:0.651f32,per_level:3f32},movespeed:345f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.651000022888183f32,attack_range:125f32,aram_damage_taken:0.95f32,aram_damage_dealt:1.05f32,urf_damage_taken:1f32,urf_damage_dealt:1.05f32,},
                    merge_data: &[],
                };pub static LEONA: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Support],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_2Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 10f32 + 0.3f32 * ctx.ap,2 => 35f32 + 0.3f32 * ctx.ap,3 => 60f32 + 0.3f32 * ctx.ap,4 => 85f32 + 0.3f32 * ctx.ap,5 => 110f32 + 0.3f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 20f32 + 0.2f32 * ctx.bonus_magic_resist,2 => 27.5f32 + 0.2f32 * ctx.bonus_magic_resist,3 => 35f32 + 0.2f32 * ctx.bonus_magic_resist,4 => 42.5f32 + 0.2f32 * ctx.bonus_magic_resist,5 => 50f32 + 0.2f32 * ctx.bonus_magic_resist,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 55f32 + 0.4f32 * ctx.ap,2 => 85f32 + 0.4f32 * ctx.ap,3 => 115f32 + 0.4f32 * ctx.ap,4 => 145f32 + 0.4f32 * ctx.ap,5 => 175f32 + 0.4f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 50f32 + 0.4f32 * ctx.ap,2 => 90f32 + 0.4f32 * ctx.ap,3 => 130f32 + 0.4f32 * ctx.ap,4 => 170f32 + 0.4f32 * ctx.ap,5 => 210f32 + 0.4f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 150f32 + 0.8f32 * ctx.ap,2 => 225f32 + 0.8f32 * ctx.ap,3 => 300f32 + 0.8f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:646f32,per_level:101f32},mana:CachedChampionStatsMap{flat:302f32,per_level:40f32},armor:CachedChampionStatsMap{flat:43f32,per_level:4.8f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:60f32,per_level:3f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:2.9f32},movespeed:335f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:125f32,aram_damage_taken:1.05f32,aram_damage_dealt:0.95f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static LILLIA: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Jungle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_3Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_2Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_4Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.w_level {1 => 80f32 + 0.35f32 * ctx.ap,2 => 100f32 + 0.35f32 * ctx.ap,3 => 120f32 + 0.35f32 * ctx.ap,4 => 140f32 + 0.35f32 * ctx.ap,5 => 160f32 + 0.35f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 40f32 + 0.175f32 * ctx.ap,2 => 50f32 + 0.175f32 * ctx.ap,3 => 60f32 + 0.175f32 * ctx.ap,4 => 70f32 + 0.175f32 * ctx.ap,5 => 80f32 + 0.175f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 240f32 + 1.05f32 * ctx.ap,2 => 300f32 + 1.05f32 * ctx.ap,3 => 360f32 + 1.05f32 * ctx.ap,4 => 420f32 + 1.05f32 * ctx.ap,5 => 480f32 + 1.05f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 120f32 + 0.525f32 * ctx.ap,2 => 150f32 + 0.525f32 * ctx.ap,3 => 180f32 + 0.525f32 * ctx.ap,4 => 210f32 + 0.525f32 * ctx.ap,5 => 240f32 + 0.525f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 60f32 + 0.5f32 * ctx.ap,2 => 85f32 + 0.5f32 * ctx.ap,3 => 110f32 + 0.5f32 * ctx.ap,4 => 135f32 + 0.5f32 * ctx.ap,5 => 160f32 + 0.5f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:605f32,per_level:105f32},mana:CachedChampionStatsMap{flat:410f32,per_level:50f32},armor:CachedChampionStatsMap{flat:22f32,per_level:4.5f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:1.55f32},attack_damage:CachedChampionStatsMap{flat:61f32,per_level:3.1f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:2.7f32},movespeed:330f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:325f32,aram_damage_taken:1.05f32,aram_damage_dealt:0.95f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static LISSANDRA: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Middle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 80f32 + 0.75f32 * ctx.ap,2 => 115f32 + 0.75f32 * ctx.ap,3 => 150f32 + 0.75f32 * ctx.ap,4 => 185f32 + 0.75f32 * ctx.ap,5 => 220f32 + 0.75f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 70f32 + 0.7f32 * ctx.ap,2 => 105f32 + 0.7f32 * ctx.ap,3 => 140f32 + 0.7f32 * ctx.ap,4 => 175f32 + 0.7f32 * ctx.ap,5 => 210f32 + 0.7f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 70f32 + 0.6f32 * ctx.ap,2 => 105f32 + 0.6f32 * ctx.ap,3 => 140f32 + 0.6f32 * ctx.ap,4 => 175f32 + 0.6f32 * ctx.ap,5 => 210f32 + 0.6f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 10f32 + 0.055f32 * ctx.ap,2 => 15f32 + 0.055f32 * ctx.ap,3 => 20f32 + 0.055f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:620f32,per_level:110f32},mana:CachedChampionStatsMap{flat:475f32,per_level:30f32},armor:CachedChampionStatsMap{flat:22f32,per_level:4.9f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:55f32,per_level:2.7f32},attack_speed:CachedChampionStatsMap{flat:0.656f32,per_level:1.5f32},movespeed:325f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:550f32,aram_damage_taken:1.05f32,aram_damage_dealt:0.95f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static LUCIAN: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Bottom],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_5Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_6Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_2Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_3Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_4Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 85f32 + 0.6f32 * ctx.bonus_ad,2 => 115f32 + 0.75f32 * ctx.bonus_ad,3 => 145f32 + 0.9f32 * ctx.bonus_ad,4 => 175f32 + 1.05f32 * ctx.bonus_ad,5 => 205f32 + 1.2f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 75f32 + 0.9f32 * ctx.ap,2 => 110f32 + 0.9f32 * ctx.ap,3 => 145f32 + 0.9f32 * ctx.ap,4 => 180f32 + 0.9f32 * ctx.ap,5 => 215f32 + 0.9f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 150f32 + 2.5f32 * ctx.ad + 1.5f32 * ctx.ap,2 => 300f32 + 2.5f32 * ctx.ad + 1.5f32 * ctx.ap,3 => 450f32 + 2.5f32 * ctx.ad + 1.5f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 660f32 + 0.3f32 * ctx.crit_chance + 11f32 * ctx.ad + 6.6f32 * ctx.ap + 0.5f32 * ctx.ad + 0.3f32 * ctx.ap,2 => 1320f32 + 0.6f32 * ctx.crit_chance + 11f32 * ctx.ad + 6.6f32 * ctx.ap + 0.5f32 * ctx.ad + 0.3f32 * ctx.ap,3 => 1980f32 + 0.9f32 * ctx.crit_chance + 11f32 * ctx.ad + 6.6f32 * ctx.ap + 0.5f32 * ctx.ad + 0.3f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 15f32 + 0.25f32 * ctx.ad + 0.15f32 * ctx.ap,2 => 30f32 + 0.25f32 * ctx.ad + 0.15f32 * ctx.ap,3 => 45f32 + 0.25f32 * ctx.ad + 0.15f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 75f32 + 1.25f32 * ctx.ad + 0.75f32 * ctx.ap,2 => 150f32 + 1.25f32 * ctx.ad + 0.75f32 * ctx.ap,3 => 225f32 + 1.25f32 * ctx.ad + 0.75f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 330f32 + 0.15f32 * ctx.crit_chance + 5.5f32 * ctx.ad + 3.3f32 * ctx.ap + 0.25f32 * ctx.ad + 0.15f32 * ctx.ap,2 => 660f32 + 0.3f32 * ctx.crit_chance + 5.5f32 * ctx.ad + 3.3f32 * ctx.ap + 0.25f32 * ctx.ad + 0.15f32 * ctx.ap,3 => 990f32 + 0.45f32 * ctx.crit_chance + 5.5f32 * ctx.ad + 3.3f32 * ctx.ap + 0.25f32 * ctx.ad + 0.15f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 30f32 + 0.5f32 * ctx.ad + 0.3f32 * ctx.ap,2 => 60f32 + 0.5f32 * ctx.ad + 0.3f32 * ctx.ap,3 => 90f32 + 0.5f32 * ctx.ad + 0.3f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:641f32,per_level:100f32},mana:CachedChampionStatsMap{flat:320f32,per_level:43f32},armor:CachedChampionStatsMap{flat:28f32,per_level:4.2f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:60f32,per_level:2.9f32},attack_speed:CachedChampionStatsMap{flat:0.638f32,per_level:3.3f32},movespeed:335f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.638000011444091f32,attack_range:500f32,aram_damage_taken:0.95f32,aram_damage_dealt:1.05f32,urf_damage_taken:1.05f32,urf_damage_dealt:0.95f32,},
                    merge_data: &[],
                };pub static LULU: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Support],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_5Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_6Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_2Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_3Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_4Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 21f32 + 0.175f32 * ctx.ap,2 => 33.25f32 + 0.175f32 * ctx.ap,3 => 45.5f32 + 0.175f32 * ctx.ap,4 => 57.75f32 + 0.175f32 * ctx.ap,5 => 70f32 + 0.175f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 63f32 + 0.525f32 * ctx.ap,2 => 99.75f32 + 0.525f32 * ctx.ap,3 => 136.5f32 + 0.525f32 * ctx.ap,4 => 173.25f32 + 0.525f32 * ctx.ap,5 => 210f32 + 0.525f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 60f32 + 0.5f32 * ctx.ap,2 => 95f32 + 0.5f32 * ctx.ap,3 => 130f32 + 0.5f32 * ctx.ap,4 => 165f32 + 0.5f32 * ctx.ap,5 => 200f32 + 0.5f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 42f32 + 0.35f32 * ctx.ap,2 => 66.5f32 + 0.35f32 * ctx.ap,3 => 91f32 + 0.35f32 * ctx.ap,4 => 115.5f32 + 0.35f32 * ctx.ap,5 => 140f32 + 0.35f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 30f32 + 0.25f32 * ctx.ap,2 => 47.5f32 + 0.25f32 * ctx.ap,3 => 65f32 + 0.25f32 * ctx.ap,4 => 82.5f32 + 0.25f32 * ctx.ap,5 => 100f32 + 0.25f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 90f32 + 0.75f32 * ctx.ap,2 => 142.5f32 + 0.75f32 * ctx.ap,3 => 195f32 + 0.75f32 * ctx.ap,4 => 247.5f32 + 0.75f32 * ctx.ap,5 => 300f32 + 0.75f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 70f32 + 0.5f32 * ctx.ap,2 => 110f32 + 0.5f32 * ctx.ap,3 => 150f32 + 0.5f32 * ctx.ap,4 => 190f32 + 0.5f32 * ctx.ap,5 => 230f32 + 0.5f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:565f32,per_level:92f32},mana:CachedChampionStatsMap{flat:350f32,per_level:55f32},armor:CachedChampionStatsMap{flat:26f32,per_level:4.6f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:47f32,per_level:2.6f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:2.25f32},movespeed:330f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:550f32,aram_damage_taken:1f32,aram_damage_dealt:1f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static LUX: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Middle,Position::Support],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 80f32 + 0.65f32 * ctx.ap,2 => 120f32 + 0.65f32 * ctx.ap,3 => 160f32 + 0.65f32 * ctx.ap,4 => 200f32 + 0.65f32 * ctx.ap,5 => 240f32 + 0.65f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 65f32 + 0.8f32 * ctx.ap,2 => 115f32 + 0.8f32 * ctx.ap,3 => 165f32 + 0.8f32 * ctx.ap,4 => 215f32 + 0.8f32 * ctx.ap,5 => 265f32 + 0.8f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 300f32 + 1.2f32 * ctx.ap,2 => 400f32 + 1.2f32 * ctx.ap,3 => 500f32 + 1.2f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:580f32,per_level:99f32},mana:CachedChampionStatsMap{flat:480f32,per_level:23.5f32},armor:CachedChampionStatsMap{flat:21f32,per_level:5.2f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:54f32,per_level:3.3f32},attack_speed:CachedChampionStatsMap{flat:0.669f32,per_level:3f32},movespeed:330f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:550f32,aram_damage_taken:1.1f32,aram_damage_dealt:0.9f32,urf_damage_taken:1f32,urf_damage_dealt:0.95f32,},
                    merge_data: &[],
                };pub static MALPHITE: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Middle,Position::Support,Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_2), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 70f32 + 0.6f32 * ctx.ap,2 => 120f32 + 0.6f32 * ctx.ap,3 => 170f32 + 0.6f32 * ctx.ap,4 => 220f32 + 0.6f32 * ctx.ap,5 => 270f32 + 0.6f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 0.1f32 * ctx.armor,2 => 0.15f32 * ctx.armor,3 => 0.2f32 * ctx.armor,4 => 0.25f32 * ctx.armor,5 => 0.3f32 * ctx.armor,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 30f32 + 0.2f32 * ctx.ap + 0.15f32 * ctx.armor,2 => 40f32 + 0.2f32 * ctx.ap + 0.15f32 * ctx.armor,3 => 50f32 + 0.2f32 * ctx.ap + 0.15f32 * ctx.armor,4 => 60f32 + 0.2f32 * ctx.ap + 0.15f32 * ctx.armor,5 => 70f32 + 0.2f32 * ctx.ap + 0.15f32 * ctx.armor,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 0.3f32,2 => 0.35f32,3 => 0.4f32,4 => 0.45f32,5 => 0.5f32,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 200f32 + 0.9f32 * ctx.ap,2 => 300f32 + 0.9f32 * ctx.ap,3 => 400f32 + 0.9f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:665f32,per_level:104f32},mana:CachedChampionStatsMap{flat:280f32,per_level:60f32},armor:CachedChampionStatsMap{flat:40f32,per_level:4.95f32},magic_resist:CachedChampionStatsMap{flat:28f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:62f32,per_level:4f32},attack_speed:CachedChampionStatsMap{flat:0.736f32,per_level:3.4f32},movespeed:335f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.638000011444091f32,attack_range:125f32,aram_damage_taken:1f32,aram_damage_dealt:1f32,urf_damage_taken:1f32,urf_damage_dealt:0.95f32,},
                    merge_data: &[],
                };pub static MALZAHAR: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Middle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_2Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_2Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 70f32 + 0.55f32 * ctx.ap,2 => 105f32 + 0.55f32 * ctx.ap,3 => 140f32 + 0.55f32 * ctx.ap,4 => 175f32 + 0.55f32 * ctx.ap,5 => 210f32 + 0.55f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 5f32 + 0.05f32 * ctx.ap,2 => 7.1875f32 + 0.05f32 * ctx.ap,3 => 9.375f32 + 0.05f32 * ctx.ap,4 => 11.5625f32 + 0.05f32 * ctx.ap,5 => 13.75f32 + 0.05f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 80f32 + 0.8f32 * ctx.ap,2 => 115f32 + 0.8f32 * ctx.ap,3 => 150f32 + 0.8f32 * ctx.ap,4 => 185f32 + 0.8f32 * ctx.ap,5 => 220f32 + 0.8f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 12.5f32 + 0.08f32 * ctx.ap,2 => 20f32 + 0.08f32 * ctx.ap,3 => 27.5f32 + 0.08f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 125f32 + 0.8f32 * ctx.ap,2 => 200f32 + 0.8f32 * ctx.ap,3 => 275f32 + 0.8f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:580f32,per_level:101f32},mana:CachedChampionStatsMap{flat:375f32,per_level:28f32},armor:CachedChampionStatsMap{flat:18f32,per_level:4.7f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:55f32,per_level:3f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:1.5f32},movespeed:335f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:500f32,aram_damage_taken:1.1f32,aram_damage_dealt:0.9f32,urf_damage_taken:1.08f32,urf_damage_dealt:0.92f32,},
                    merge_data: &[],
                };pub static MAOKAI: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Jungle,Position::Support],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Monster1), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Monster2), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_3Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 120f32,2 => 130f32,3 => 140f32,4 => 150f32,5 => 160f32,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 65f32 + 0.02f32 * ctx.enemy_max_health + 0.4f32 * ctx.ap,2 => 110f32 + 0.025f32 * ctx.enemy_max_health + 0.4f32 * ctx.ap,3 => 155f32 + 0.03f32 * ctx.enemy_max_health + 0.4f32 * ctx.ap,4 => 200f32 + 0.035f32 * ctx.enemy_max_health + 0.4f32 * ctx.ap,5 => 245f32 + 0.04f32 * ctx.enemy_max_health + 0.4f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 185f32 + 0.02f32 * ctx.enemy_max_health + 0.4f32 * ctx.ap,2 => 240f32 + 0.025f32 * ctx.enemy_max_health + 0.4f32 * ctx.ap,3 => 295f32 + 0.03f32 * ctx.enemy_max_health + 0.4f32 * ctx.ap,4 => 350f32 + 0.035f32 * ctx.enemy_max_health + 0.4f32 * ctx.ap,5 => 405f32 + 0.04f32 * ctx.enemy_max_health + 0.4f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 60f32 + 0.4f32 * ctx.ap,2 => 85f32 + 0.4f32 * ctx.ap,3 => 110f32 + 0.4f32 * ctx.ap,4 => 135f32 + 0.4f32 * ctx.ap,5 => 160f32 + 0.4f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 33.33f32 + 0.0333f32 * ctx.bonus_health + 0.16670000000000001f32 * ctx.ap,2 => 50f32 + 0.0333f32 * ctx.bonus_health + 0.16670000000000001f32 * ctx.ap,3 => 66.67f32 + 0.0333f32 * ctx.bonus_health + 0.16670000000000001f32 * ctx.ap,4 => 83.33f32 + 0.0333f32 * ctx.bonus_health + 0.16670000000000001f32 * ctx.ap,5 => 100f32 + 0.0333f32 * ctx.bonus_health + 0.16670000000000001f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 150f32 + 0.75f32 * ctx.ap,2 => 225f32 + 0.75f32 * ctx.ap,3 => 300f32 + 0.75f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:665f32,per_level:109f32},mana:CachedChampionStatsMap{flat:375f32,per_level:43f32},armor:CachedChampionStatsMap{flat:35f32,per_level:5.2f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:64f32,per_level:3.3f32},attack_speed:CachedChampionStatsMap{flat:0.8f32,per_level:2.125f32},movespeed:335f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.694999992847442f32,attack_range:125f32,aram_damage_taken:1.1f32,aram_damage_dealt:0.85f32,urf_damage_taken:1.05f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static MASTERYI: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Jungle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1), 
                damage_type: DamageType::True, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.e_level {1 => 20f32 + 0.35f32 * ctx.bonus_ad,2 => 25f32 + 0.35f32 * ctx.bonus_ad,3 => 30f32 + 0.35f32 * ctx.bonus_ad,4 => 35f32 + 0.35f32 * ctx.bonus_ad,5 => 40f32 + 0.35f32 * ctx.bonus_ad,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:655f32,per_level:105f32},mana:CachedChampionStatsMap{flat:251f32,per_level:42f32},armor:CachedChampionStatsMap{flat:33f32,per_level:4.2f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:65f32,per_level:2.8f32},attack_speed:CachedChampionStatsMap{flat:0.679f32,per_level:2.5f32},movespeed:355f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.67900002002716f32,attack_range:175f32,aram_damage_taken:1f32,aram_damage_dealt:1f32,urf_damage_taken:1f32,urf_damage_dealt:0.9f32,},
                    merge_data: &[],
                };pub static MEL: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Middle,Position::Support],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_3Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_4Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_2Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_2Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_3Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_5Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_6Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 78f32 + 0.51f32 * ctx.ap,2 => 108.5f32 + 0.595f32 * ctx.ap,3 => 144f32 + 0.68f32 * ctx.ap,4 => 184.5f32 + 0.765f32 * ctx.ap,5 => 230f32 + 0.85f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 58.5f32 + 0.3825f32 * ctx.ap,2 => 81.375f32 + 0.44625f32 * ctx.ap,3 => 108f32 + 0.51f32 * ctx.ap,4 => 138.375f32 + 0.57375f32 * ctx.ap,5 => 172.5f32 + 0.6375f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 13f32 + 0.085f32 * ctx.ap,2 => 15.5f32 + 0.085f32 * ctx.ap,3 => 18f32 + 0.085f32 * ctx.ap,4 => 20.5f32 + 0.085f32 * ctx.ap,5 => 23f32 + 0.085f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 9.75f32 + 0.06375f32 * ctx.ap,2 => 11.625f32 + 0.06375f32 * ctx.ap,3 => 13.5f32 + 0.06375f32 * ctx.ap,4 => 15.375f32 + 0.06375f32 * ctx.ap,5 => 17.25f32 + 0.06375f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 0.4f32 * 100.0f32 + 0.05f32 * 0.01f32 * ctx.ap,2 => 0.45f32 * 100.0f32 + 0.05f32 * 0.01f32 * ctx.ap,3 => 0.5f32 * 100.0f32 + 0.05f32 * 0.01f32 * ctx.ap,4 => 0.55f32 * 100.0f32 + 0.05f32 * 0.01f32 * ctx.ap,5 => 0.6f32 * 100.0f32 + 0.05f32 * 0.01f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 60f32 + 0.6f32 * ctx.ap,2 => 105f32 + 0.6f32 * ctx.ap,3 => 150f32 + 0.6f32 * ctx.ap,4 => 195f32 + 0.6f32 * ctx.ap,5 => 240f32 + 0.6f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 2f32 + 0.01f32 * ctx.ap,2 => 4f32 + 0.01f32 * ctx.ap,3 => 6f32 + 0.01f32 * ctx.ap,4 => 8f32 + 0.01f32 * ctx.ap,5 => 10f32 + 0.01f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 16f32 + 0.08f32 * ctx.ap,2 => 32f32 + 0.08f32 * ctx.ap,3 => 48f32 + 0.08f32 * ctx.ap,4 => 64f32 + 0.08f32 * ctx.ap,5 => 80f32 + 0.08f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 30f32 + 0.3f32 * ctx.ap,2 => 52.5f32 + 0.3f32 * ctx.ap,3 => 75f32 + 0.3f32 * ctx.ap,4 => 97.5f32 + 0.3f32 * ctx.ap,5 => 120f32 + 0.3f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 1f32 + 0.005f32 * ctx.ap,2 => 2f32 + 0.005f32 * ctx.ap,3 => 3f32 + 0.005f32 * ctx.ap,4 => 4f32 + 0.005f32 * ctx.ap,5 => 5f32 + 0.005f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 10f32,2 => 20f32,3 => 30f32,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:630f32,per_level:93f32},mana:CachedChampionStatsMap{flat:480f32,per_level:28f32},armor:CachedChampionStatsMap{flat:21f32,per_level:5.2f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:54f32,per_level:3.3f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:2.5f32},movespeed:330f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.4f32,attack_range:550f32,aram_damage_taken:1f32,aram_damage_dealt:1f32,urf_damage_taken:1f32,urf_damage_dealt:0.9f32,},
                    merge_data: &[],
                };pub static MILIO: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Support],
                    metadata: &[],
                    closures: &[],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:560f32,per_level:88f32},mana:CachedChampionStatsMap{flat:365f32,per_level:43f32},armor:CachedChampionStatsMap{flat:26f32,per_level:4.6f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:48f32,per_level:3.2f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:3f32},movespeed:330f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:525f32,aram_damage_taken:1f32,aram_damage_dealt:1f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static MISSFORTUNE: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Bottom],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Void), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 20f32 + ctx.ad + 0.35f32 * ctx.ap,2 => 45f32 + ctx.ad + 0.35f32 * ctx.ap,3 => 70f32 + ctx.ad + 0.35f32 * ctx.ap,4 => 95f32 + ctx.ad + 0.35f32 * ctx.ap,5 => 120f32 + ctx.ad + 0.35f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 70f32 + 1.2f32 * ctx.ap,2 => 100f32 + 1.2f32 * ctx.ap,3 => 130f32 + 1.2f32 * ctx.ap,4 => 160f32 + 1.2f32 * ctx.ap,5 => 190f32 + 1.2f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 8.75f32 + 0.15f32 * ctx.ap,2 => 12.5f32 + 0.15f32 * ctx.ap,3 => 16.25f32 + 0.15f32 * ctx.ap,4 => 20f32 + 0.15f32 * ctx.ap,5 => 23.75f32 + 0.15f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 14f32,2 => 16f32,3 => 18f32,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:640f32,per_level:103f32},mana:CachedChampionStatsMap{flat:300f32,per_level:40f32},armor:CachedChampionStatsMap{flat:25f32,per_level:4f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:55f32,per_level:2.4f32},attack_speed:CachedChampionStatsMap{flat:0.656f32,per_level:3f32},movespeed:325f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.656000018119812f32,attack_range:550f32,aram_damage_taken:1.05f32,aram_damage_dealt:0.95f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static MONKEYKING: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Jungle,Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Void), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::Void), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_2Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 20f32 + 0.5f32 * ctx.bonus_ad,2 => 45f32 + 0.5f32 * ctx.bonus_ad,3 => 70f32 + 0.5f32 * ctx.bonus_ad,4 => 95f32 + 0.5f32 * ctx.bonus_ad,5 => 120f32 + 0.5f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 0.3f32,2 => 0.35f32,3 => 0.4f32,4 => 0.45f32,5 => 0.5f32,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 80f32 + ctx.ap,2 => 120f32 + ctx.ap,3 => 160f32 + ctx.ap,4 => 200f32 + ctx.ap,5 => 240f32 + ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 0.08f32 * ctx.enemy_max_health + 2.75f32 * ctx.ad,2 => 0.12f32 * ctx.enemy_max_health + 2.75f32 * ctx.ad,3 => 0.16f32 * ctx.enemy_max_health + 2.75f32 * ctx.ad,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 0.01f32 * ctx.enemy_max_health + 0.34375f32 * ctx.ad,2 => 0.015f32 * ctx.enemy_max_health + 0.34375f32 * ctx.ad,3 => 0.02f32 * ctx.enemy_max_health + 0.34375f32 * ctx.ad,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:610f32,per_level:99f32},mana:CachedChampionStatsMap{flat:330f32,per_level:65f32},armor:CachedChampionStatsMap{flat:31f32,per_level:4.7f32},magic_resist:CachedChampionStatsMap{flat:28f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:66f32,per_level:3.5f32},attack_speed:CachedChampionStatsMap{flat:0.69f32,per_level:3f32},movespeed:340f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.657999992370605f32,attack_range:175f32,aram_damage_taken:1f32,aram_damage_dealt:1f32,urf_damage_taken:1.1f32,urf_damage_dealt:0.9f32,},
                    merge_data: &[],
                };pub static MORDEKAISER: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_2Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 0f32 + 80f32 + 1.2f32 * ctx.bonus_ad + 0.7f32 * ctx.ap,2 => 2.6470588235294117f32 + 115f32 + 1.2f32 * ctx.bonus_ad + 0.7f32 * ctx.ap,3 => 5.294117647058823f32 + 150f32 + 1.2f32 * ctx.bonus_ad + 0.7f32 * ctx.ap,4 => 7.9411764705882355f32 + 185f32 + 1.2f32 * ctx.bonus_ad + 0.7f32 * ctx.ap,5 => 10.588235294117649f32 + 220f32 + 1.2f32 * ctx.bonus_ad + 0.7f32 * ctx.ap,6 => 13.235294117647058f32,7 => 15.882352941176473f32,8 => 18.52941176470588f32,9 => 21.176470588235293f32,10 => 23.823529411764707f32,11 => 26.470588235294116f32,12 => 29.11764705882353f32,13 => 31.764705882352946f32,14 => 34.411764705882355f32,15 => 37.05882352941176f32,16 => 39.705882352941174f32,17 => 42.35294117647059f32,18 => 45f32,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 0.3f32,2 => 0.35f32,3 => 0.4f32,4 => 0.45f32,5 => 0.5f32,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 0.05f32,2 => 0.075f32,3 => 0.1f32,4 => 0.125f32,5 => 0.15f32,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:645f32,per_level:104f32},mana:CachedChampionStatsMap{flat:0f32,per_level:0f32},armor:CachedChampionStatsMap{flat:37f32,per_level:4.2f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:61f32,per_level:4f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:1f32},movespeed:335f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:175f32,aram_damage_taken:1f32,aram_damage_dealt:1f32,urf_damage_taken:0.85f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static MORGANA: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Middle,Position::Support],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_2Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 80f32 + 0.9f32 * ctx.ap,2 => 135f32 + 0.9f32 * ctx.ap,3 => 190f32 + 0.9f32 * ctx.ap,4 => 245f32 + 0.9f32 * ctx.ap,5 => 300f32 + 0.9f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 0.2f32,2 => 0.4f32,3 => 0.6f32,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 400f32 + 1.6f32 * ctx.ap,2 => 550f32 + 1.6f32 * ctx.ap,3 => 700f32 + 1.6f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:630f32,per_level:104f32},mana:CachedChampionStatsMap{flat:340f32,per_level:60f32},armor:CachedChampionStatsMap{flat:25f32,per_level:5f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:56f32,per_level:3.5f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:1.53f32},movespeed:335f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:450f32,aram_damage_taken:1.1f32,aram_damage_dealt:0.95f32,urf_damage_taken:1.05f32,urf_damage_dealt:0.9f32,},
                    merge_data: &[],
                };pub static NAAFIRI: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Middle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_5), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_7), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_3Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_4Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_6Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_8Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_2Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_3Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_2Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_2Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 60f32 + 1.4f32 * ctx.bonus_ad,2 => 90f32 + 1.4f32 * ctx.bonus_ad,3 => 120f32 + 1.4f32 * ctx.bonus_ad,4 => 150f32 + 1.4f32 * ctx.bonus_ad,5 => 180f32 + 1.4f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 130f32 + 2.4f32 * ctx.bonus_ad,2 => 190f32 + 2.4f32 * ctx.bonus_ad,3 => 250f32 + 2.4f32 * ctx.bonus_ad,4 => 310f32 + 2.4f32 * ctx.bonus_ad,5 => 370f32 + 2.4f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 35f32 + 0.8f32 * ctx.bonus_ad,2 => 60f32 + 0.8f32 * ctx.bonus_ad,3 => 85f32 + 0.8f32 * ctx.bonus_ad,4 => 110f32 + 0.8f32 * ctx.bonus_ad,5 => 135f32 + 0.8f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 70f32 + ctx.bonus_ad,2 => 100f32 + ctx.bonus_ad,3 => 130f32 + ctx.bonus_ad,4 => 160f32 + ctx.bonus_ad,5 => 190f32 + ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 100f32 + 1.4f32 * ctx.bonus_ad,2 => 145f32 + 1.4f32 * ctx.bonus_ad,3 => 190f32 + 1.4f32 * ctx.bonus_ad,4 => 235f32 + 1.4f32 * ctx.bonus_ad,5 => 280f32 + 1.4f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 45f32 + 0.4f32 * ctx.bonus_ad,2 => 60f32 + 0.4f32 * ctx.bonus_ad,3 => 75f32 + 0.4f32 * ctx.bonus_ad,4 => 90f32 + 0.4f32 * ctx.bonus_ad,5 => 105f32 + 0.4f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 35f32 + 0.2f32 * ctx.bonus_ad,2 => 40f32 + 0.2f32 * ctx.bonus_ad,3 => 45f32 + 0.2f32 * ctx.bonus_ad,4 => 50f32 + 0.2f32 * ctx.bonus_ad,5 => 55f32 + 0.2f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 3.5f32 + 0.08f32 * ctx.bonus_ad,2 => 6f32 + 0.08f32 * ctx.bonus_ad,3 => 8.5f32 + 0.08f32 * ctx.bonus_ad,4 => 11f32 + 0.08f32 * ctx.bonus_ad,5 => 13.5f32 + 0.08f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 75f32 + 1.2f32 * ctx.bonus_ad,2 => 110f32 + 1.2f32 * ctx.bonus_ad,3 => 145f32 + 1.2f32 * ctx.bonus_ad,4 => 180f32 + 1.2f32 * ctx.bonus_ad,5 => 215f32 + 1.2f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 15f32 + 0.4f32 * ctx.bonus_ad,2 => 25f32 + 0.4f32 * ctx.bonus_ad,3 => 35f32 + 0.4f32 * ctx.bonus_ad,4 => 45f32 + 0.4f32 * ctx.bonus_ad,5 => 55f32 + 0.4f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 60f32 + 0.8f32 * ctx.bonus_ad,2 => 85f32 + 0.8f32 * ctx.bonus_ad,3 => 110f32 + 0.8f32 * ctx.bonus_ad,4 => 135f32 + 0.8f32 * ctx.bonus_ad,5 => 160f32 + 0.8f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 150f32 + 1.2f32 * ctx.bonus_ad,2 => 250f32 + 1.2f32 * ctx.bonus_ad,3 => 350f32 + 1.2f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 15f32 + 0.12f32 * ctx.bonus_ad,2 => 25f32 + 0.12f32 * ctx.bonus_ad,3 => 35f32 + 0.12f32 * ctx.bonus_ad,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:610f32,per_level:105f32},mana:CachedChampionStatsMap{flat:400f32,per_level:55f32},armor:CachedChampionStatsMap{flat:28f32,per_level:4.2f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:55f32,per_level:2f32},attack_speed:CachedChampionStatsMap{flat:0.663f32,per_level:2.1f32},movespeed:340f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:125f32,aram_damage_taken:1f32,aram_damage_dealt:1.1f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static NAMI: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Support],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::Minion1), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_2), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 90f32 + 0.5f32 * ctx.ap,2 => 145f32 + 0.5f32 * ctx.ap,3 => 200f32 + 0.5f32 * ctx.ap,4 => 255f32 + 0.5f32 * ctx.ap,5 => 310f32 + 0.5f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 44f32 + 0.32f32 * ctx.ap,2 => 64f32 + 0.32f32 * ctx.ap,3 => 84f32 + 0.32f32 * ctx.ap,4 => 104f32 + 0.32f32 * ctx.ap,5 => 124f32 + 0.32f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 60f32 + 0.5f32 * ctx.ap,2 => 95f32 + 0.5f32 * ctx.ap,3 => 130f32 + 0.5f32 * ctx.ap,4 => 165f32 + 0.5f32 * ctx.ap,5 => 200f32 + 0.5f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 20f32 + 0.2f32 * ctx.ap,2 => 30f32 + 0.2f32 * ctx.ap,3 => 40f32 + 0.2f32 * ctx.ap,4 => 50f32 + 0.2f32 * ctx.ap,5 => 60f32 + 0.2f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 0.15f32 + 0.05f32 * 0.01f32 * ctx.ap,2 => 0.2f32 + 0.05f32 * 0.01f32 * ctx.ap,3 => 0.25f32 + 0.05f32 * 0.01f32 * ctx.ap,4 => 0.3f32 + 0.05f32 * 0.01f32 * ctx.ap,5 => 0.35f32 + 0.05f32 * 0.01f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 150f32 + 0.6f32 * ctx.ap,2 => 250f32 + 0.6f32 * ctx.ap,3 => 350f32 + 0.6f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:560f32,per_level:88f32},mana:CachedChampionStatsMap{flat:365f32,per_level:43f32},armor:CachedChampionStatsMap{flat:29f32,per_level:5.2f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:54f32,per_level:3.1f32},attack_speed:CachedChampionStatsMap{flat:0.644f32,per_level:2.61f32},movespeed:335f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.643999993801116f32,attack_range:550f32,aram_damage_taken:1.05f32,aram_damage_dealt:0.95f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static NASUS: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_3Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_2Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_2Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 35f32 + ctx.nasus_stacks,2 => 55f32 + ctx.nasus_stacks,3 => 75f32 + ctx.nasus_stacks,4 => 95f32 + ctx.nasus_stacks,5 => 115f32 + ctx.nasus_stacks,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 0.0f32,2 => 0.0f32,3 => 0.0f32,4 => 0.0f32,5 => 0.0f32,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 50f32 + 0.6f32 * ctx.ap,2 => 80f32 + 0.6f32 * ctx.ap,3 => 110f32 + 0.6f32 * ctx.ap,4 => 140f32 + 0.6f32 * ctx.ap,5 => 170f32 + 0.6f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 100f32 + 1.2f32 * ctx.ap,2 => 160f32 + 1.2f32 * ctx.ap,3 => 220f32 + 1.2f32 * ctx.ap,4 => 280f32 + 1.2f32 * ctx.ap,5 => 340f32 + 1.2f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 0.45f32 * ctx.enemy_max_health + 0.15f32 * 0.01f32 * ctx.ap,2 => 0.6f32 * ctx.enemy_max_health + 0.15f32 * 0.01f32 * ctx.ap,3 => 0.75f32 * ctx.enemy_max_health + 0.15f32 * 0.01f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 0.015f32 * ctx.enemy_max_health + 0.005f32 * 0.01f32 * ctx.ap,2 => 0.02f32 * ctx.enemy_max_health + 0.005f32 * 0.01f32 * ctx.ap,3 => 0.025f32 * ctx.enemy_max_health + 0.005f32 * 0.01f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:631f32,per_level:104f32},mana:CachedChampionStatsMap{flat:326f32,per_level:62f32},armor:CachedChampionStatsMap{flat:34f32,per_level:4.7f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:67f32,per_level:4f32},attack_speed:CachedChampionStatsMap{flat:0.638f32,per_level:3.48f32},movespeed:350f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.638000011444091f32,attack_range:125f32,aram_damage_taken:1.05f32,aram_damage_dealt:0.9f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static NAUTILUS: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Support],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_2Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_3Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::Monster1), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_2Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 85f32 + 0.9f32 * ctx.ap,2 => 130f32 + 0.9f32 * ctx.ap,3 => 175f32 + 0.9f32 * ctx.ap,4 => 220f32 + 0.9f32 * ctx.ap,5 => 265f32 + 0.9f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 30f32 + 0.4f32 * ctx.ap,2 => 40f32 + 0.4f32 * ctx.ap,3 => 50f32 + 0.4f32 * ctx.ap,4 => 60f32 + 0.4f32 * ctx.ap,5 => 70f32 + 0.4f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 27.5f32 + 0.25f32 * ctx.ap,2 => 45f32 + 0.25f32 * ctx.ap,3 => 62.5f32 + 0.25f32 * ctx.ap,4 => 80f32 + 0.25f32 * ctx.ap,5 => 97.5f32 + 0.25f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 55f32 + 0.5f32 * ctx.ap,2 => 90f32 + 0.5f32 * ctx.ap,3 => 125f32 + 0.5f32 * ctx.ap,4 => 160f32 + 0.5f32 * ctx.ap,5 => 195f32 + 0.5f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 110f32 + ctx.ap,2 => 180f32 + ctx.ap,3 => 250f32 + ctx.ap,4 => 320f32 + ctx.ap,5 => 390f32 + ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 125f32 + 0.5f32 * ctx.ap,2 => 165f32 + 0.5f32 * ctx.ap,3 => 205f32 + 0.5f32 * ctx.ap,4 => 245f32 + 0.5f32 * ctx.ap,5 => 285f32 + 0.5f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 150f32 + 0.8f32 * ctx.ap,2 => 275f32 + 0.8f32 * ctx.ap,3 => 400f32 + 0.8f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 125f32 + 0.4f32 * ctx.ap,2 => 175f32 + 0.4f32 * ctx.ap,3 => 225f32 + 0.4f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:646f32,per_level:100f32},mana:CachedChampionStatsMap{flat:400f32,per_level:47f32},armor:CachedChampionStatsMap{flat:39f32,per_level:4.95f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:61f32,per_level:3.3f32},attack_speed:CachedChampionStatsMap{flat:0.706f32,per_level:1f32},movespeed:325f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.611999988555908f32,attack_range:175f32,aram_damage_taken:1.1f32,aram_damage_dealt:0.95f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static NEEKO: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Middle,Position::Support],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Monster), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 35f32,2 => 50f32,3 => 65f32,4 => 80f32,5 => 95f32,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 60f32 + 0.6f32 * ctx.ap,2 => 110f32 + 0.6f32 * ctx.ap,3 => 160f32 + 0.6f32 * ctx.ap,4 => 210f32 + 0.6f32 * ctx.ap,5 => 260f32 + 0.6f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 35f32 + 0.25f32 * ctx.ap,2 => 60f32 + 0.25f32 * ctx.ap,3 => 85f32 + 0.25f32 * ctx.ap,4 => 110f32 + 0.25f32 * ctx.ap,5 => 135f32 + 0.25f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 0.2f32,2 => 0.25f32,3 => 0.3f32,4 => 0.35f32,5 => 0.4f32,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 70f32 + 0.65f32 * ctx.ap,2 => 105f32 + 0.65f32 * ctx.ap,3 => 140f32 + 0.65f32 * ctx.ap,4 => 175f32 + 0.65f32 * ctx.ap,5 => 210f32 + 0.65f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 150f32 + 1.2f32 * ctx.ap,2 => 350f32 + 1.2f32 * ctx.ap,3 => 550f32 + 1.2f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:610f32,per_level:104f32},mana:CachedChampionStatsMap{flat:450f32,per_level:30f32},armor:CachedChampionStatsMap{flat:21f32,per_level:5.2f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:48f32,per_level:2.5f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:3.5f32},movespeed:340f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.67f32,attack_range:550f32,aram_damage_taken:1f32,aram_damage_dealt:1f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static NIDALEE: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Jungle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_2Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_3Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_2Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_3Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Unknown, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 70f32 + 0.5f32 * ctx.ap,2 => 90f32,3 => 110f32,4 => 130f32,5 => 150f32,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 227.5f32 + 1.625f32 * ctx.ap,2 => 292.5f32,3 => 357.5f32,4 => 422.5f32,5 => 487.5f32,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 40f32 + 0.2f32 * ctx.ap,2 => 80f32,3 => 120f32,4 => 160f32,5 => 200f32,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 10f32 + 0.05f32 * ctx.ap,2 => 20f32,3 => 30f32,4 => 40f32,5 => 50f32,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 0.3f32,2 => 0.4f32,3 => 0.5f32,4 => 0.6f32,5 => 0.7f32,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:610f32,per_level:109f32},mana:CachedChampionStatsMap{flat:295f32,per_level:45f32},armor:CachedChampionStatsMap{flat:32f32,per_level:5f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.45f32},attack_damage:CachedChampionStatsMap{flat:58f32,per_level:3.5f32},attack_speed:CachedChampionStatsMap{flat:0.638f32,per_level:3.22f32},movespeed:335f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.638000011444091f32,attack_range:525f32,aram_damage_taken:1f32,aram_damage_dealt:1.1f32,urf_damage_taken:0.95f32,urf_damage_dealt:1.1f32,},
                    merge_data: &[],
                };pub static NILAH: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Bottom],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_2Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_4Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_3Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.e_level {1 => 60f32 + 0.2f32 * ctx.bonus_ad,2 => 70f32 + 0.2f32 * ctx.bonus_ad,3 => 80f32 + 0.2f32 * ctx.bonus_ad,4 => 90f32 + 0.2f32 * ctx.bonus_ad,5 => 100f32 + 0.2f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 60f32 + 0.4f32 * ctx.bonus_ad,2 => 100f32 + 0.4f32 * ctx.bonus_ad,3 => 140f32 + 0.4f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 185f32 + 1.4f32 * ctx.bonus_ad,2 => 325f32 + 1.4f32 * ctx.bonus_ad,3 => 465f32 + 1.4f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 15f32 + 0.1f32 * ctx.bonus_ad,2 => 25f32 + 0.1f32 * ctx.bonus_ad,3 => 35f32 + 0.1f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 125f32 + ctx.bonus_ad,2 => 225f32 + ctx.bonus_ad,3 => 325f32 + ctx.bonus_ad,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:570f32,per_level:101f32},mana:CachedChampionStatsMap{flat:350f32,per_level:35f32},armor:CachedChampionStatsMap{flat:27f32,per_level:4.5f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:60f32,per_level:2f32},attack_speed:CachedChampionStatsMap{flat:0.697f32,per_level:2.25f32},movespeed:340f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.67f32,attack_range:225f32,aram_damage_taken:1f32,aram_damage_dealt:1f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static NOCTURNE: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Jungle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_2), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 15f32,2 => 25f32,3 => 35f32,4 => 45f32,5 => 55f32,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 65f32 + 0.85f32 * ctx.bonus_ad,2 => 110f32 + 0.85f32 * ctx.bonus_ad,3 => 155f32 + 0.85f32 * ctx.bonus_ad,4 => 200f32 + 0.85f32 * ctx.bonus_ad,5 => 245f32 + 0.85f32 * ctx.bonus_ad,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:655f32,per_level:109f32},mana:CachedChampionStatsMap{flat:275f32,per_level:35f32},armor:CachedChampionStatsMap{flat:38f32,per_level:4.2f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:1.55f32},attack_damage:CachedChampionStatsMap{flat:62f32,per_level:2.6f32},attack_speed:CachedChampionStatsMap{flat:0.721f32,per_level:2.7f32},movespeed:345f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.721f32,attack_range:125f32,aram_damage_taken:0.9f32,aram_damage_dealt:1.1f32,urf_damage_taken:1f32,urf_damage_dealt:0.95f32,},
                    merge_data: &[],
                };pub static NUNU: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Jungle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Unknown, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_2Min), 
                damage_type: DamageType::Unknown, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::Minion1), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_2Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_4Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_3Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 65f32 + 0.9f32 * ctx.ap + 0.1f32 * ctx.bonus_health,2 => 95f32 + 0.9f32 * ctx.ap + 0.1f32 * ctx.bonus_health,3 => 125f32 + 0.9f32 * ctx.ap + 0.1f32 * ctx.bonus_health,4 => 155f32 + 0.9f32 * ctx.ap + 0.1f32 * ctx.bonus_health,5 => 185f32 + 0.9f32 * ctx.ap + 0.1f32 * ctx.bonus_health,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 58.5f32 + 0.81f32 * ctx.ap + 0.09f32 * ctx.bonus_health,2 => 85.5f32 + 0.81f32 * ctx.ap + 0.09f32 * ctx.bonus_health,3 => 112.5f32 + 0.81f32 * ctx.ap + 0.09f32 * ctx.bonus_health,4 => 139.5f32 + 0.81f32 * ctx.ap + 0.09f32 * ctx.bonus_health,5 => 166.5f32 + 0.81f32 * ctx.ap + 0.09f32 * ctx.bonus_health,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 36f32 + 0.3f32 * ctx.ap,2 => 45f32 + 0.3f32 * ctx.ap,3 => 54f32 + 0.3f32 * ctx.ap,4 => 63f32 + 0.3f32 * ctx.ap,5 => 72f32 + 0.3f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 180f32 + 1.5f32 * ctx.ap,2 => 225f32 + 1.5f32 * ctx.ap,3 => 270f32 + 1.5f32 * ctx.ap,4 => 315f32 + 1.5f32 * ctx.ap,5 => 360f32 + 1.5f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 48f32 + 0.45f32 * ctx.ap,2 => 72f32 + 0.45f32 * ctx.ap,3 => 96f32 + 0.45f32 * ctx.ap,4 => 120f32 + 0.45f32 * ctx.ap,5 => 144f32 + 0.45f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 20f32 + 0.8f32 * ctx.ap,2 => 30f32 + 0.8f32 * ctx.ap,3 => 40f32 + 0.8f32 * ctx.ap,4 => 50f32 + 0.8f32 * ctx.ap,5 => 60f32 + 0.8f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 16f32 + 0.15f32 * ctx.ap,2 => 24f32 + 0.15f32 * ctx.ap,3 => 32f32 + 0.15f32 * ctx.ap,4 => 40f32 + 0.15f32 * ctx.ap,5 => 48f32 + 0.15f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 0.3f32,2 => 0.35f32,3 => 0.4f32,4 => 0.45f32,5 => 0.5f32,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 625f32 + 3f32 * ctx.ap,2 => 950f32 + 3f32 * ctx.ap,3 => 1275f32 + 3f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:610f32,per_level:90f32},mana:CachedChampionStatsMap{flat:280f32,per_level:42f32},armor:CachedChampionStatsMap{flat:29f32,per_level:4.2f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:61f32,per_level:3f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:2.25f32},movespeed:345f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:125f32,aram_damage_taken:0.9f32,aram_damage_dealt:1.1f32,urf_damage_taken:0.95f32,urf_damage_dealt:1.08f32,},
                    merge_data: &[],
                };pub static OLAF: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::True, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1), 
                damage_type: DamageType::Unknown, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 60f32 + ctx.bonus_ad,2 => 110f32 + ctx.bonus_ad,3 => 160f32 + ctx.bonus_ad,4 => 210f32 + ctx.bonus_ad,5 => 260f32 + ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 70f32 + 0.5f32 * ctx.ad,2 => 115f32 + 0.5f32 * ctx.ad,3 => 160f32 + 0.5f32 * ctx.ad,4 => 205f32 + 0.5f32 * ctx.ad,5 => 250f32 + 0.5f32 * ctx.ad,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 10f32,2 => 15f32,3 => 20f32,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:645f32,per_level:119f32},mana:CachedChampionStatsMap{flat:316f32,per_level:50f32},armor:CachedChampionStatsMap{flat:35f32,per_level:4.2f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:68f32,per_level:4.7f32},attack_speed:CachedChampionStatsMap{flat:0.694f32,per_level:2.7f32},movespeed:350f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.694000005722045f32,attack_range:125f32,aram_damage_taken:1f32,aram_damage_dealt:1.05f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static ORIANNA: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Middle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 42f32 + 0.385f32 * ctx.ap,2 => 63f32 + 0.385f32 * ctx.ap,3 => 84f32 + 0.385f32 * ctx.ap,4 => 105f32 + 0.385f32 * ctx.ap,5 => 126f32 + 0.385f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 60f32 + 0.55f32 * ctx.ap,2 => 90f32 + 0.55f32 * ctx.ap,3 => 120f32 + 0.55f32 * ctx.ap,4 => 150f32 + 0.55f32 * ctx.ap,5 => 180f32 + 0.55f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 70f32 + 0.8f32 * ctx.ap,2 => 110f32 + 0.8f32 * ctx.ap,3 => 150f32 + 0.8f32 * ctx.ap,4 => 190f32 + 0.8f32 * ctx.ap,5 => 230f32 + 0.8f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 6f32,2 => 12f32,3 => 18f32,4 => 24f32,5 => 30f32,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 250f32 + 0.95f32 * ctx.ap,2 => 400f32 + 0.95f32 * ctx.ap,3 => 550f32 + 0.95f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:585f32,per_level:110f32},mana:CachedChampionStatsMap{flat:418f32,per_level:25f32},armor:CachedChampionStatsMap{flat:20f32,per_level:4.2f32},magic_resist:CachedChampionStatsMap{flat:26f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:44f32,per_level:2.6f32},attack_speed:CachedChampionStatsMap{flat:0.658f32,per_level:3.5f32},movespeed:325f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.657999992370605f32,attack_range:525f32,aram_damage_taken:1f32,aram_damage_dealt:1f32,urf_damage_taken:1.1f32,urf_damage_dealt:0.9f32,},
                    merge_data: &[],
                };pub static ORNN: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_3Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_2Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::Minion1), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::Monster1), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 20f32 + 1.1f32 * ctx.ad,2 => 45f32 + 1.1f32 * ctx.ad,3 => 70f32 + 1.1f32 * ctx.ad,4 => 95f32 + 1.1f32 * ctx.ad,5 => 120f32 + 1.1f32 * ctx.ad,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 16f32,2 => 26f32,3 => 36f32,4 => 46f32,5 => 56f32,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 80f32,2 => 130f32,3 => 180f32,4 => 230f32,5 => 280f32,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 0.12f32 * ctx.enemy_max_health,2 => 0.13f32 * ctx.enemy_max_health,3 => 0.14f32 * ctx.enemy_max_health,4 => 0.15f32 * ctx.enemy_max_health,5 => 0.16f32 * ctx.enemy_max_health,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 0.024f32 * ctx.enemy_max_health,2 => 0.026000000000000002f32 * ctx.enemy_max_health,3 => 0.027999999999999997f32 * ctx.enemy_max_health,4 => 0.03f32 * ctx.enemy_max_health,5 => 0.032f32 * ctx.enemy_max_health,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 80f32 + 0.4f32 * ctx.bonus_armor + 0.4f32 * ctx.bonus_magic_resist,2 => 125f32 + 0.4f32 * ctx.bonus_armor + 0.4f32 * ctx.bonus_magic_resist,3 => 170f32 + 0.4f32 * ctx.bonus_armor + 0.4f32 * ctx.bonus_magic_resist,4 => 215f32 + 0.4f32 * ctx.bonus_armor + 0.4f32 * ctx.bonus_magic_resist,5 => 260f32 + 0.4f32 * ctx.bonus_armor + 0.4f32 * ctx.bonus_magic_resist,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 125f32 + 0.2f32 * ctx.ap,2 => 175f32 + 0.2f32 * ctx.ap,3 => 225f32 + 0.2f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:660f32,per_level:109f32},mana:CachedChampionStatsMap{flat:341f32,per_level:65f32},armor:CachedChampionStatsMap{flat:33f32,per_level:5.2f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:69f32,per_level:3.5f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:2f32},movespeed:335f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:175f32,aram_damage_taken:1.05f32,aram_damage_dealt:0.9f32,urf_damage_taken:0.95f32,urf_damage_dealt:1.05f32,},
                    merge_data: &[],
                };pub static PANTHEON: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Jungle,Position::Middle,Position::Support,Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Unknown, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_2Min), 
                damage_type: DamageType::Unknown, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.w_level {1 => 0.06f32 * ctx.enemy_max_health + 0.015f32 * 0.01f32 * ctx.ap + 0.004f32 * 0.01f32 * ctx.bonus_health,2 => 0.065f32 * ctx.enemy_max_health + 0.015f32 * 0.01f32 * ctx.ap + 0.004f32 * 0.01f32 * ctx.bonus_health,3 => 0.07f32 * ctx.enemy_max_health + 0.015f32 * 0.01f32 * ctx.ap + 0.004f32 * 0.01f32 * ctx.bonus_health,4 => 0.075f32 * ctx.enemy_max_health + 0.015f32 * 0.01f32 * ctx.ap + 0.004f32 * 0.01f32 * ctx.bonus_health,5 => 0.08f32 * ctx.enemy_max_health + 0.015f32 * 0.01f32 * ctx.ap + 0.004f32 * 0.01f32 * ctx.bonus_health,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 300f32 + ctx.ap,2 => 500f32 + ctx.ap,3 => 700f32 + ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 150f32 + 0.5f32 * ctx.ap,2 => 250f32 + 0.5f32 * ctx.ap,3 => 350f32 + 0.5f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:650f32,per_level:109f32},mana:CachedChampionStatsMap{flat:317f32,per_level:31f32},armor:CachedChampionStatsMap{flat:40f32,per_level:4.95f32},magic_resist:CachedChampionStatsMap{flat:28f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:64f32,per_level:3.3f32},attack_speed:CachedChampionStatsMap{flat:0.658f32,per_level:2.95f32},movespeed:345f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.658f32,attack_range:175f32,aram_damage_taken:1f32,aram_damage_dealt:1f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static POPPY: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Jungle,Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_2Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Monster1), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Monster2), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Monster3), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_2Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 160f32 + 2f32 * ctx.bonus_ad,2 => 270f32 + 2f32 * ctx.bonus_ad,3 => 380f32 + 2f32 * ctx.bonus_ad,4 => 490f32 + 2f32 * ctx.bonus_ad,5 => 600f32 + 2f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 80f32 + ctx.bonus_ad,2 => 135f32 + ctx.bonus_ad,3 => 190f32 + ctx.bonus_ad,4 => 245f32 + ctx.bonus_ad,5 => 300f32 + ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 30f32 + ctx.bonus_ad + 0.09f32 * ctx.enemy_max_health,2 => 55f32 + ctx.bonus_ad + 0.09f32 * ctx.enemy_max_health,3 => 80f32 + ctx.bonus_ad + 0.09f32 * ctx.enemy_max_health,4 => 105f32 + ctx.bonus_ad + 0.09f32 * ctx.enemy_max_health,5 => 130f32 + ctx.bonus_ad + 0.09f32 * ctx.enemy_max_health,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 50f32,2 => 80f32,3 => 110f32,4 => 140f32,5 => 170f32,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 60f32 + 2f32 * ctx.bonus_ad + 0.18f32 * ctx.enemy_max_health,2 => 110f32 + 2f32 * ctx.bonus_ad + 0.18f32 * ctx.enemy_max_health,3 => 160f32 + 2f32 * ctx.bonus_ad + 0.18f32 * ctx.enemy_max_health,4 => 210f32 + 2f32 * ctx.bonus_ad + 0.18f32 * ctx.enemy_max_health,5 => 260f32 + 2f32 * ctx.bonus_ad + 0.18f32 * ctx.enemy_max_health,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 80f32 + 1.2f32 * ctx.bonus_ad,2 => 120f32 + 1.2f32 * ctx.bonus_ad,3 => 160f32 + 1.2f32 * ctx.bonus_ad,4 => 200f32 + 1.2f32 * ctx.bonus_ad,5 => 240f32 + 1.2f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 40f32 + 0.6f32 * ctx.bonus_ad,2 => 60f32 + 0.6f32 * ctx.bonus_ad,3 => 80f32 + 0.6f32 * ctx.bonus_ad,4 => 100f32 + 0.6f32 * ctx.bonus_ad,5 => 120f32 + 0.6f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 100f32 + 0.45f32 * ctx.bonus_ad,2 => 150f32 + 0.45f32 * ctx.bonus_ad,3 => 200f32 + 0.45f32 * ctx.bonus_ad,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:610f32,per_level:110f32},mana:CachedChampionStatsMap{flat:280f32,per_level:40f32},armor:CachedChampionStatsMap{flat:35f32,per_level:5f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:60f32,per_level:4f32},attack_speed:CachedChampionStatsMap{flat:0.658f32,per_level:2.5f32},movespeed:345f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:125f32,aram_damage_taken:1f32,aram_damage_dealt:1f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static PYKE: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Support],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::E(AbilityName::Void), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.e_level {1 => 100f32 + ctx.bonus_ad,2 => 150f32 + ctx.bonus_ad,3 => 200f32 + ctx.bonus_ad,4 => 250f32 + ctx.bonus_ad,5 => 300f32 + ctx.bonus_ad,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:670f32,per_level:110f32},mana:CachedChampionStatsMap{flat:415f32,per_level:50f32},armor:CachedChampionStatsMap{flat:43f32,per_level:4.7f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:62f32,per_level:2f32},attack_speed:CachedChampionStatsMap{flat:0.667f32,per_level:2.5f32},movespeed:330f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.666999995708465f32,attack_range:150f32,aram_damage_taken:0.9f32,aram_damage_dealt:1.1f32,urf_damage_taken:0.95f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static QIYANA: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Middle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_2Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::Monster1), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 70f32 + 0.9f32 * ctx.bonus_ad,2 => 100f32 + 0.9f32 * ctx.bonus_ad,3 => 130f32 + 0.9f32 * ctx.bonus_ad,4 => 160f32 + 0.9f32 * ctx.bonus_ad,5 => 190f32 + 0.9f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 52.5f32 + 0.675f32 * ctx.bonus_ad,2 => 75f32 + 0.675f32 * ctx.bonus_ad,3 => 97.5f32 + 0.675f32 * ctx.bonus_ad,4 => 120f32 + 0.675f32 * ctx.bonus_ad,5 => 142.5f32 + 0.675f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 50f32 + 0.5f32 * ctx.bonus_ad,2 => 90f32 + 0.5f32 * ctx.bonus_ad,3 => 130f32 + 0.5f32 * ctx.bonus_ad,4 => 170f32 + 0.5f32 * ctx.bonus_ad,5 => 210f32 + 0.5f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 500f32 + 1.25f32 * ctx.bonus_ad,2 => 750f32 + 1.25f32 * ctx.bonus_ad,3 => 1000f32 + 1.25f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 100f32 + 1.25f32 * ctx.bonus_ad + 0.1f32 * ctx.enemy_max_health,2 => 200f32 + 1.25f32 * ctx.bonus_ad + 0.1f32 * ctx.enemy_max_health,3 => 300f32 + 1.25f32 * ctx.bonus_ad + 0.1f32 * ctx.enemy_max_health,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:590f32,per_level:124f32},mana:CachedChampionStatsMap{flat:375f32,per_level:60f32},armor:CachedChampionStatsMap{flat:31f32,per_level:4.5f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:64f32,per_level:3.1f32},attack_speed:CachedChampionStatsMap{flat:0.688f32,per_level:2.1f32},movespeed:335f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:150f32,aram_damage_taken:0.9f32,aram_damage_dealt:1.15f32,urf_damage_taken:0.9f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static QUINN: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 65f32 + 0.8f32 * ctx.bonus_ad + 0.5f32 * ctx.ap,2 => 100f32 + 0.9f32 * ctx.bonus_ad + 0.5f32 * ctx.ap,3 => 135f32 + ctx.bonus_ad + 0.5f32 * ctx.ap,4 => 170f32 + 1.1f32 * ctx.bonus_ad + 0.5f32 * ctx.ap,5 => 205f32 + 1.2f32 * ctx.bonus_ad + 0.5f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 40f32 + 0.2f32 * ctx.bonus_ad,2 => 65f32 + 0.2f32 * ctx.bonus_ad,3 => 90f32 + 0.2f32 * ctx.bonus_ad,4 => 115f32 + 0.2f32 * ctx.bonus_ad,5 => 140f32 + 0.2f32 * ctx.bonus_ad,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:565f32,per_level:107f32},mana:CachedChampionStatsMap{flat:269f32,per_level:35f32},armor:CachedChampionStatsMap{flat:28f32,per_level:4.7f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:59f32,per_level:2.7f32},attack_speed:CachedChampionStatsMap{flat:0.668f32,per_level:3.1f32},movespeed:330f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.667999982833862f32,attack_range:525f32,aram_damage_taken:0.9f32,aram_damage_dealt:1.1f32,urf_damage_taken:0.95f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static RAKAN: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Support],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 70f32 + 0.7f32 * ctx.ap,2 => 115f32 + 0.7f32 * ctx.ap,3 => 160f32 + 0.7f32 * ctx.ap,4 => 205f32 + 0.7f32 * ctx.ap,5 => 250f32 + 0.7f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 70f32 + 0.8f32 * ctx.ap,2 => 120f32 + 0.8f32 * ctx.ap,3 => 170f32 + 0.8f32 * ctx.ap,4 => 220f32 + 0.8f32 * ctx.ap,5 => 270f32 + 0.8f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 0.0f32,2 => 0.0f32,3 => 0.0f32,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:610f32,per_level:99f32},mana:CachedChampionStatsMap{flat:315f32,per_level:50f32},armor:CachedChampionStatsMap{flat:30f32,per_level:4.9f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:62f32,per_level:3.5f32},attack_speed:CachedChampionStatsMap{flat:0.635f32,per_level:3f32},movespeed:335f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.634999990463256f32,attack_range:300f32,aram_damage_taken:0.95f32,aram_damage_dealt:1f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static RAMMUS: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Jungle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::Monster1), 
                damage_type: DamageType::Unknown, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 80f32 + ctx.ap,2 => 120f32 + ctx.ap,3 => 160f32 + ctx.ap,4 => 200f32 + ctx.ap,5 => 240f32 + ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 0.0f32,2 => 0.0f32,3 => 0.0f32,4 => 0.0f32,5 => 0.0f32,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 150f32 + 0.6f32 * ctx.ap,2 => 250f32 + 0.6f32 * ctx.ap,3 => 350f32 + 0.6f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:675f32,per_level:100f32},mana:CachedChampionStatsMap{flat:310f32,per_level:33f32},armor:CachedChampionStatsMap{flat:35f32,per_level:4.5f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:65f32,per_level:2.75f32},attack_speed:CachedChampionStatsMap{flat:0.7f32,per_level:2.215f32},movespeed:335f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:125f32,aram_damage_taken:1f32,aram_damage_dealt:1f32,urf_damage_taken:0.85f32,urf_damage_dealt:1.1f32,},
                    merge_data: &[],
                };pub static REKSAI: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Jungle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_2Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Unknown, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_2Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 0.3f32 * ctx.ad,2 => 0.35f32 * ctx.ad,3 => 0.4f32 * ctx.ad,4 => 0.45f32 * ctx.ad,5 => 0.5f32 * ctx.ad,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 5f32,2 => 10f32,3 => 15f32,4 => 20f32,5 => 25f32,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 80f32 + 0.64f32 * ctx.bonus_ad,2 => 108f32 + 0.64f32 * ctx.bonus_ad,3 => 136f32 + 0.64f32 * ctx.bonus_ad,4 => 164f32 + 0.64f32 * ctx.bonus_ad,5 => 192f32 + 0.64f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 100f32 + 0.8f32 * ctx.bonus_ad,2 => 135f32 + 0.8f32 * ctx.bonus_ad,3 => 170f32 + 0.8f32 * ctx.bonus_ad,4 => 205f32 + 0.8f32 * ctx.bonus_ad,5 => 240f32 + 0.8f32 * ctx.bonus_ad,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:600f32,per_level:99f32},mana:CachedChampionStatsMap{flat:100f32,per_level:0f32},armor:CachedChampionStatsMap{flat:35f32,per_level:4.5f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:62f32,per_level:3f32},attack_speed:CachedChampionStatsMap{flat:0.667f32,per_level:2f32},movespeed:340f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.666999995708465f32,attack_range:175f32,aram_damage_taken:0.9f32,aram_damage_dealt:1.05f32,urf_damage_taken:0.93f32,urf_damage_dealt:1.07f32,},
                    merge_data: &[],
                };pub static RELL: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Support],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_2), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_2Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 60f32 + 0.6f32 * ctx.ap,2 => 100f32 + 0.6f32 * ctx.ap,3 => 140f32 + 0.6f32 * ctx.ap,4 => 180f32 + 0.6f32 * ctx.ap,5 => 220f32 + 0.6f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 20f32,2 => 25f32,3 => 30f32,4 => 35f32,5 => 40f32,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 0.05f32 * ctx.enemy_max_health + 0.03f32 * 0.01f32 * ctx.ap,2 => 0.055f32 * ctx.enemy_max_health + 0.03f32 * 0.01f32 * ctx.ap,3 => 0.06f32 * ctx.enemy_max_health + 0.03f32 * 0.01f32 * ctx.ap,4 => 0.065f32 * ctx.enemy_max_health + 0.03f32 * 0.01f32 * ctx.ap,5 => 0.07f32 * ctx.enemy_max_health + 0.03f32 * 0.01f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 120f32 + 1.1f32 * ctx.ap,2 => 200f32 + 1.1f32 * ctx.ap,3 => 280f32 + 1.1f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 15f32 + 0.1375f32 * ctx.ap,2 => 25f32 + 0.1375f32 * ctx.ap,3 => 35f32 + 0.1375f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:620f32,per_level:104f32},mana:CachedChampionStatsMap{flat:320f32,per_level:40f32},armor:CachedChampionStatsMap{flat:30f32,per_level:4f32},magic_resist:CachedChampionStatsMap{flat:28f32,per_level:1.8f32},attack_damage:CachedChampionStatsMap{flat:55f32,per_level:3f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:2f32},movespeed:315f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:175f32,aram_damage_taken:1f32,aram_damage_dealt:0.95f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static RENATA: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Support],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 80f32 + 0.8f32 * ctx.ap,2 => 125f32 + 0.8f32 * ctx.ap,3 => 170f32 + 0.8f32 * ctx.ap,4 => 215f32 + 0.8f32 * ctx.ap,5 => 260f32 + 0.8f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 65f32 + 0.55f32 * ctx.ap,2 => 95f32 + 0.55f32 * ctx.ap,3 => 125f32 + 0.55f32 * ctx.ap,4 => 155f32 + 0.55f32 * ctx.ap,5 => 185f32 + 0.55f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:545f32,per_level:94f32},mana:CachedChampionStatsMap{flat:350f32,per_level:50f32},armor:CachedChampionStatsMap{flat:27f32,per_level:4.7f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:49f32,per_level:3f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:2.11f32},movespeed:330f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:550f32,aram_damage_taken:1.05f32,aram_damage_dealt:0.95f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static RENEKTON: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_2Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_2Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_2Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_2Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 60f32 + ctx.bonus_ad,2 => 90f32 + ctx.bonus_ad,3 => 120f32 + ctx.bonus_ad,4 => 150f32 + ctx.bonus_ad,5 => 180f32 + ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 6f32 + 0.06f32 * ctx.bonus_ad,2 => 9f32 + 0.06f32 * ctx.bonus_ad,3 => 12f32 + 0.06f32 * ctx.bonus_ad,4 => 15f32 + 0.06f32 * ctx.bonus_ad,5 => 18f32 + 0.06f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 5f32 + 0.75f32 * ctx.ad,2 => 20f32 + 0.75f32 * ctx.ad,3 => 35f32 + 0.75f32 * ctx.ad,4 => 50f32 + 0.75f32 * ctx.ad,5 => 65f32 + 0.75f32 * ctx.ad,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 10f32 + 1.5f32 * ctx.ad,2 => 40f32 + 1.5f32 * ctx.ad,3 => 70f32 + 1.5f32 * ctx.ad,4 => 100f32 + 1.5f32 * ctx.ad,5 => 130f32 + 1.5f32 * ctx.ad,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 40f32 + 0.9f32 * ctx.bonus_ad,2 => 70f32 + 0.9f32 * ctx.bonus_ad,3 => 100f32 + 0.9f32 * ctx.bonus_ad,4 => 130f32 + 0.9f32 * ctx.bonus_ad,5 => 160f32 + 0.9f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 900f32 + 1.5f32 * ctx.bonus_ad + 1.5f32 * ctx.ap,2 => 2250f32 + 1.5f32 * ctx.bonus_ad + 1.5f32 * ctx.ap,3 => 3600f32 + 1.5f32 * ctx.bonus_ad + 1.5f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 30f32 + 0.05f32 * ctx.bonus_ad + 0.05f32 * ctx.ap,2 => 75f32 + 0.05f32 * ctx.bonus_ad + 0.05f32 * ctx.ap,3 => 120f32 + 0.05f32 * ctx.bonus_ad + 0.05f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:660f32,per_level:111f32},mana:CachedChampionStatsMap{flat:100f32,per_level:0f32},armor:CachedChampionStatsMap{flat:35f32,per_level:5.2f32},magic_resist:CachedChampionStatsMap{flat:28f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:69f32,per_level:4.15f32},attack_speed:CachedChampionStatsMap{flat:0.665f32,per_level:2.75f32},movespeed:345f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.665000021457672f32,attack_range:125f32,aram_damage_taken:1f32,aram_damage_dealt:1f32,urf_damage_taken:0.95f32,urf_damage_dealt:0.95f32,},
                    merge_data: &[],
                };pub static RENGAR: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Jungle,Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 30f32 + 0f32 * ctx.ad,2 => 60f32 + 0.0375f32 * ctx.ad,3 => 90f32 + 0.075f32 * ctx.ad,4 => 120f32 + 0.1125f32 * ctx.ad,5 => 150f32 + 0.15f32 * ctx.ad,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 55f32 + 0.8f32 * ctx.bonus_ad,2 => 100f32 + 0.8f32 * ctx.bonus_ad,3 => 145f32 + 0.8f32 * ctx.bonus_ad,4 => 190f32 + 0.8f32 * ctx.bonus_ad,5 => 235f32 + 0.8f32 * ctx.bonus_ad,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:590f32,per_level:104f32},mana:CachedChampionStatsMap{flat:4f32,per_level:0f32},armor:CachedChampionStatsMap{flat:34f32,per_level:4.2f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:68f32,per_level:3f32},attack_speed:CachedChampionStatsMap{flat:0.667f32,per_level:3f32},movespeed:345f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.666999995708465f32,attack_range:125f32,aram_damage_taken:0.92f32,aram_damage_dealt:1.05f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static RIVEN: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.w_level {1 => 65f32 + ctx.bonus_ad,2 => 95f32 + ctx.bonus_ad,3 => 125f32 + ctx.bonus_ad,4 => 155f32 + ctx.bonus_ad,5 => 185f32 + ctx.bonus_ad,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:630f32,per_level:100f32},mana:CachedChampionStatsMap{flat:0f32,per_level:0f32},armor:CachedChampionStatsMap{flat:33f32,per_level:4.4f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:64f32,per_level:3f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:3.5f32},movespeed:340f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:125f32,aram_damage_taken:0.92f32,aram_damage_dealt:1.05f32,urf_damage_taken:0.95f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static RUMBLE: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Middle,Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_3Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_4Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_7Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_8Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_2Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_5Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_6Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_2Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_2Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::Minion1), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 5f32 + 0.08333f32 * ctx.ap + 0.005f32 * ctx.enemy_max_health,2 => 7.083f32 + 0.08333f32 * ctx.ap + 0.00542f32 * ctx.enemy_max_health,3 => 9.167f32 + 0.08333f32 * ctx.ap + 0.005829999999999999f32 * ctx.enemy_max_health,4 => 11.25f32 + 0.08333f32 * ctx.ap + 0.00625f32 * ctx.enemy_max_health,5 => 13.333f32 + 0.08333f32 * ctx.ap + 0.006670000000000001f32 * ctx.enemy_max_health,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 75f32 + 1.25f32 * ctx.ap + 0.075f32 * ctx.enemy_max_health,2 => 106.25f32 + 1.25f32 * ctx.ap + 0.08130000000000001f32 * ctx.enemy_max_health,3 => 137.5f32 + 1.25f32 * ctx.ap + 0.0875f32 * ctx.enemy_max_health,4 => 168.75f32 + 1.25f32 * ctx.ap + 0.09380000000000001f32 * ctx.enemy_max_health,5 => 200f32 + 1.25f32 * ctx.ap + 0.1f32 * ctx.enemy_max_health,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 7.5f32 + 0.125f32 * ctx.ap + 0.0075f32 * ctx.enemy_max_health,2 => 10.625f32 + 0.125f32 * ctx.ap + 0.00813f32 * ctx.enemy_max_health,3 => 13.75f32 + 0.125f32 * ctx.ap + 0.00875f32 * ctx.enemy_max_health,4 => 16.875f32 + 0.125f32 * ctx.ap + 0.00938f32 * ctx.enemy_max_health,5 => 20f32 + 0.125f32 * ctx.ap + 0.01f32 * ctx.enemy_max_health,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 112.5f32 + 1.875f32 * ctx.ap + 0.1125f32 * ctx.enemy_max_health,2 => 159.38f32 + 1.875f32 * ctx.ap + 0.1219f32 * ctx.enemy_max_health,3 => 206.25f32 + 1.875f32 * ctx.ap + 0.1313f32 * ctx.enemy_max_health,4 => 253.13f32 + 1.875f32 * ctx.ap + 0.1406f32 * ctx.enemy_max_health,5 => 300f32 + 1.875f32 * ctx.ap + 0.15f32 * ctx.enemy_max_health,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 15f32 + 0.25f32 * ctx.ap + 0.015f32 * ctx.enemy_max_health,2 => 21.25f32 + 0.25f32 * ctx.ap + 0.01625f32 * ctx.enemy_max_health,3 => 27.5f32 + 0.25f32 * ctx.ap + 0.0175f32 * ctx.enemy_max_health,4 => 33.75f32 + 0.25f32 * ctx.ap + 0.01875f32 * ctx.enemy_max_health,5 => 40f32 + 0.25f32 * ctx.ap + 0.02f32 * ctx.enemy_max_health,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 20f32 + 0.3333f32 * ctx.ap + 0.02f32 * ctx.enemy_max_health,2 => 28.33f32 + 0.3333f32 * ctx.ap + 0.02167f32 * ctx.enemy_max_health,3 => 36.67f32 + 0.3333f32 * ctx.ap + 0.023330000000000004f32 * ctx.enemy_max_health,4 => 45f32 + 0.3333f32 * ctx.ap + 0.025f32 * ctx.enemy_max_health,5 => 53.33f32 + 0.3333f32 * ctx.ap + 0.02667f32 * ctx.enemy_max_health,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 22.5f32 + 0.375f32 * ctx.ap + 0.0225f32 * ctx.enemy_max_health,2 => 31.875f32 + 0.375f32 * ctx.ap + 0.024380000000000002f32 * ctx.enemy_max_health,3 => 41.25f32 + 0.375f32 * ctx.ap + 0.02625f32 * ctx.enemy_max_health,4 => 50.625f32 + 0.375f32 * ctx.ap + 0.028130000000000002f32 * ctx.enemy_max_health,5 => 60f32 + 0.375f32 * ctx.ap + 0.03f32 * ctx.enemy_max_health,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 30f32 + 0.5f32 * ctx.ap + 0.03f32 * ctx.enemy_max_health,2 => 42.5f32 + 0.5f32 * ctx.ap + 0.0325f32 * ctx.enemy_max_health,3 => 55f32 + 0.5f32 * ctx.ap + 0.035f32 * ctx.enemy_max_health,4 => 67.5f32 + 0.5f32 * ctx.ap + 0.0375f32 * ctx.enemy_max_health,5 => 80f32 + 0.5f32 * ctx.ap + 0.04f32 * ctx.enemy_max_health,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 0.15f32,2 => 0.2f32,3 => 0.25f32,4 => 0.3f32,5 => 0.35f32,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 55f32 + 0.5f32 * ctx.ap,2 => 80f32 + 0.5f32 * ctx.ap,3 => 105f32 + 0.5f32 * ctx.ap,4 => 130f32 + 0.5f32 * ctx.ap,5 => 155f32 + 0.5f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 140f32 + 0.35f32 * ctx.ap,2 => 210f32 + 0.35f32 * ctx.ap,3 => 280f32 + 0.35f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 35f32 + 0.0875f32 * ctx.ap,2 => 52.5f32 + 0.0875f32 * ctx.ap,3 => 70f32 + 0.0875f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 700f32 + 1.75f32 * ctx.ap,2 => 1050f32 + 1.75f32 * ctx.ap,3 => 1400f32 + 1.75f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:655f32,per_level:105f32},mana:CachedChampionStatsMap{flat:150f32,per_level:0f32},armor:CachedChampionStatsMap{flat:36f32,per_level:4.7f32},magic_resist:CachedChampionStatsMap{flat:28f32,per_level:1.55f32},attack_damage:CachedChampionStatsMap{flat:64f32,per_level:3.2f32},attack_speed:CachedChampionStatsMap{flat:0.644f32,per_level:1.85f32},movespeed:345f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.643999993801116f32,attack_range:125f32,aram_damage_taken:1f32,aram_damage_dealt:1f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static RYZE: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Middle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.w_level {1 => 60f32 + 0.7f32 * ctx.ap + 0.04f32 * ctx.bonus_mana,2 => 90f32 + 0.7f32 * ctx.ap + 0.04f32 * ctx.bonus_mana,3 => 120f32 + 0.7f32 * ctx.ap + 0.04f32 * ctx.bonus_mana,4 => 150f32 + 0.7f32 * ctx.ap + 0.04f32 * ctx.bonus_mana,5 => 180f32 + 0.7f32 * ctx.ap + 0.04f32 * ctx.bonus_mana,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 60f32 + 0.5f32 * ctx.ap + 0.02f32 * ctx.bonus_mana,2 => 90f32 + 0.5f32 * ctx.ap + 0.02f32 * ctx.bonus_mana,3 => 120f32 + 0.5f32 * ctx.ap + 0.02f32 * ctx.bonus_mana,4 => 150f32 + 0.5f32 * ctx.ap + 0.02f32 * ctx.bonus_mana,5 => 180f32 + 0.5f32 * ctx.ap + 0.02f32 * ctx.bonus_mana,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:645f32,per_level:124f32},mana:CachedChampionStatsMap{flat:300f32,per_level:70f32},armor:CachedChampionStatsMap{flat:22f32,per_level:4.2f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:58f32,per_level:3f32},attack_speed:CachedChampionStatsMap{flat:0.658f32,per_level:2.11f32},movespeed:340f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:550f32,aram_damage_taken:0.95f32,aram_damage_dealt:1.05f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static SAMIRA: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Bottom],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 5f32 + 0.95f32 * ctx.ad,2 => 10f32 + 1.025f32 * ctx.ad,3 => 15f32 + 1.1f32 * ctx.ad,4 => 20f32 + 1.175f32 * ctx.ad,5 => 25f32 + 1.25f32 * ctx.ad,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 0.2f32,2 => 0.25f32,3 => 0.3f32,4 => 0.35f32,5 => 0.4f32,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:630f32,per_level:108f32},mana:CachedChampionStatsMap{flat:349f32,per_level:38f32},armor:CachedChampionStatsMap{flat:26f32,per_level:4.7f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:57f32,per_level:3f32},attack_speed:CachedChampionStatsMap{flat:0.658f32,per_level:3.3f32},movespeed:335f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.658f32,attack_range:500f32,aram_damage_taken:1f32,aram_damage_dealt:1f32,urf_damage_taken:1f32,urf_damage_dealt:0.95f32,},
                    merge_data: &[],
                };pub static SEJUANI: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Jungle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_3Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_2Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 90f32 + 0.6f32 * ctx.ap,2 => 140f32 + 0.6f32 * ctx.ap,3 => 190f32 + 0.6f32 * ctx.ap,4 => 240f32 + 0.6f32 * ctx.ap,5 => 290f32 + 0.6f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 10f32 + 0.8f32 * ctx.ap + 0.12f32 * ctx.max_health,2 => 40f32 + 0.8f32 * ctx.ap + 0.12f32 * ctx.max_health,3 => 70f32 + 0.8f32 * ctx.ap + 0.12f32 * ctx.max_health,4 => 100f32 + 0.8f32 * ctx.ap + 0.12f32 * ctx.max_health,5 => 130f32 + 0.8f32 * ctx.ap + 0.12f32 * ctx.max_health,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 5f32 + 0.2f32 * ctx.ap + 0.04f32 * ctx.max_health,2 => 15f32 + 0.2f32 * ctx.ap + 0.04f32 * ctx.max_health,3 => 25f32 + 0.2f32 * ctx.ap + 0.04f32 * ctx.max_health,4 => 35f32 + 0.2f32 * ctx.ap + 0.04f32 * ctx.max_health,5 => 45f32 + 0.2f32 * ctx.ap + 0.04f32 * ctx.max_health,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 5f32 + 0.6f32 * ctx.ap + 0.08f32 * ctx.max_health,2 => 25f32 + 0.6f32 * ctx.ap + 0.08f32 * ctx.max_health,3 => 45f32 + 0.6f32 * ctx.ap + 0.08f32 * ctx.max_health,4 => 65f32 + 0.6f32 * ctx.ap + 0.08f32 * ctx.max_health,5 => 85f32 + 0.6f32 * ctx.ap + 0.08f32 * ctx.max_health,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 125f32 + 0.4f32 * ctx.ap,2 => 150f32 + 0.4f32 * ctx.ap,3 => 175f32 + 0.4f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:630f32,per_level:114f32},mana:CachedChampionStatsMap{flat:400f32,per_level:40f32},armor:CachedChampionStatsMap{flat:34f32,per_level:5.45f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:66f32,per_level:4f32},attack_speed:CachedChampionStatsMap{flat:0.688f32,per_level:3.5f32},movespeed:340f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:150f32,aram_damage_taken:1f32,aram_damage_dealt:1f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static SENNA: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Bottom,Position::Support],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 30f32 + 0.4f32 * ctx.bonus_ad,2 => 60f32 + 0.4f32 * ctx.bonus_ad,3 => 90f32 + 0.4f32 * ctx.bonus_ad,4 => 120f32 + 0.4f32 * ctx.bonus_ad,5 => 150f32 + 0.4f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 70f32 + 0.7f32 * ctx.bonus_ad,2 => 115f32 + 0.7f32 * ctx.bonus_ad,3 => 160f32 + 0.7f32 * ctx.bonus_ad,4 => 205f32 + 0.7f32 * ctx.bonus_ad,5 => 250f32 + 0.7f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 250f32 + 1.15f32 * ctx.bonus_ad + 0.7f32 * ctx.ap,2 => 400f32 + 1.15f32 * ctx.bonus_ad + 0.7f32 * ctx.ap,3 => 550f32 + 1.15f32 * ctx.bonus_ad + 0.7f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:530f32,per_level:89f32},mana:CachedChampionStatsMap{flat:350f32,per_level:45f32},armor:CachedChampionStatsMap{flat:25f32,per_level:4f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:50f32,per_level:0f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:2.6f32},movespeed:330f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.4f32,attack_range:600f32,aram_damage_taken:1.05f32,aram_damage_dealt:0.92f32,urf_damage_taken:1f32,urf_damage_dealt:1.1f32,},
                    merge_data: &[],
                };pub static SERAPHINE: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Bottom,Position::Support],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_2Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_2Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 105f32 + 0.875f32 * ctx.ap,2 => 148.75f32 + 0.875f32 * ctx.ap,3 => 192.5f32 + 0.875f32 * ctx.ap,4 => 236.25f32 + 0.875f32 * ctx.ap,5 => 280f32 + 0.875f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 60f32 + 0.5f32 * ctx.ap,2 => 85f32 + 0.5f32 * ctx.ap,3 => 110f32 + 0.5f32 * ctx.ap,4 => 135f32 + 0.5f32 * ctx.ap,5 => 160f32 + 0.5f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 49f32 + 0.35f32 * ctx.ap,2 => 70f32 + 0.35f32 * ctx.ap,3 => 91f32 + 0.35f32 * ctx.ap,4 => 112f32 + 0.35f32 * ctx.ap,5 => 133f32 + 0.35f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 0.0f32,2 => 0.0f32,3 => 0.0f32,4 => 0.0f32,5 => 0.0f32,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 0.0f32,2 => 0.0f32,3 => 0.0f32,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:570f32,per_level:95f32},mana:CachedChampionStatsMap{flat:360f32,per_level:40f32},armor:CachedChampionStatsMap{flat:26f32,per_level:4.2f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:50f32,per_level:3f32},attack_speed:CachedChampionStatsMap{flat:0.669f32,per_level:2f32},movespeed:330f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:525f32,aram_damage_taken:1.2f32,aram_damage_dealt:1f32,urf_damage_taken:1f32,urf_damage_dealt:0.92f32,},
                    merge_data: &[],
                };pub static SETT: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_2), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::Monster1), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_2Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 0.0f32,2 => 0.0f32,3 => 0.0f32,4 => 0.0f32,5 => 0.0f32,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 0.0f32,2 => 0.0f32,3 => 0.0f32,4 => 0.0f32,5 => 0.0f32,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 50f32 + 0.6f32 * ctx.ad,2 => 70f32 + 0.6f32 * ctx.ad,3 => 90f32 + 0.6f32 * ctx.ad,4 => 110f32 + 0.6f32 * ctx.ad,5 => 130f32 + 0.6f32 * ctx.ad,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 200f32 + 1.2f32 * ctx.bonus_ad + 0.4f32 * ctx.enemy_bonus_health,2 => 300f32 + 1.2f32 * ctx.bonus_ad + 0.5f32 * ctx.enemy_bonus_health,3 => 400f32 + 1.2f32 * ctx.bonus_ad + 0.6f32 * ctx.enemy_bonus_health,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 50f32 + 0.3f32 * ctx.bonus_ad + 0.1f32 * ctx.enemy_bonus_health,2 => 75f32 + 0.3f32 * ctx.bonus_ad + 0.125f32 * ctx.enemy_bonus_health,3 => 100f32 + 0.3f32 * ctx.bonus_ad + 0.15f32 * ctx.enemy_bonus_health,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:670f32,per_level:114f32},mana:CachedChampionStatsMap{flat:0f32,per_level:0f32},armor:CachedChampionStatsMap{flat:33f32,per_level:4.7f32},magic_resist:CachedChampionStatsMap{flat:28f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:60f32,per_level:4f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:1.75f32},movespeed:340f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:125f32,aram_damage_taken:1.1f32,aram_damage_dealt:1f32,urf_damage_taken:0.92f32,urf_damage_dealt:1.05f32,},
                    merge_data: &[],
                };pub static SHACO: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Jungle,Position::Support],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::Monster1), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::Monster2), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_2Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.w_level {1 => 0.0f32,2 => 0.0f32,3 => 0.0f32,4 => 0.0f32,5 => 0.0f32,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 0.0f32,2 => 0.0f32,3 => 0.0f32,4 => 0.0f32,5 => 0.0f32,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 0.0f32,2 => 0.0f32,3 => 0.0f32,4 => 0.0f32,5 => 0.0f32,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 0.2f32,2 => 0.225f32,3 => 0.25f32,4 => 0.275f32,5 => 0.3f32,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 10f32 + 0.1f32 * ctx.ap,2 => 20f32 + 0.1f32 * ctx.ap,3 => 30f32 + 0.1f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 25f32 + 0.15f32 * ctx.ap,2 => 50f32 + 0.15f32 * ctx.ap,3 => 75f32 + 0.15f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:630f32,per_level:99f32},mana:CachedChampionStatsMap{flat:297f32,per_level:40f32},armor:CachedChampionStatsMap{flat:30f32,per_level:4f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:63f32,per_level:3f32},attack_speed:CachedChampionStatsMap{flat:0.694f32,per_level:3f32},movespeed:345f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.694000005722045f32,attack_range:125f32,aram_damage_taken:1f32,aram_damage_dealt:1.05f32,urf_damage_taken:1.1f32,urf_damage_dealt:0.9f32,},
                    merge_data: &[],
                };pub static SHEN: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Support,Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_3), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_2Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Monster1), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 10f32 + 0.05f32 * ctx.enemy_max_health + 0.02f32 * 0.01f32 * ctx.ap,2 => 11.764705882352942f32 + 0.055f32 * ctx.enemy_max_health + 0.02f32 * 0.01f32 * ctx.ap,3 => 13.529411764705882f32 + 0.06f32 * ctx.enemy_max_health + 0.02f32 * 0.01f32 * ctx.ap,4 => 15.294117647058822f32 + 0.065f32 * ctx.enemy_max_health + 0.02f32 * 0.01f32 * ctx.ap,5 => 17.058823529411764f32 + 0.07f32 * ctx.enemy_max_health + 0.02f32 * 0.01f32 * ctx.ap,6 => 18.823529411764707f32,7 => 20.588235294117645f32,8 => 22.352941176470587f32,9 => 24.11764705882353f32,10 => 25.88235294117647f32,11 => 27.647058823529413f32,12 => 29.41176470588235f32,13 => 31.176470588235293f32,14 => 32.94117647058823f32,15 => 34.705882352941174f32,16 => 36.47058823529411f32,17 => 38.23529411764706f32,18 => 40f32,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 120f32,2 => 140f32,3 => 160f32,4 => 180f32,5 => 200f32,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 30f32 + 0.15f32 * ctx.enemy_max_health + 0.06f32 * 0.01f32 * ctx.ap,2 => 35.294117647058826f32 + 0.165f32 * ctx.enemy_max_health + 0.06f32 * 0.01f32 * ctx.ap,3 => 40.588235294117645f32 + 0.18f32 * ctx.enemy_max_health + 0.06f32 * 0.01f32 * ctx.ap,4 => 45.88235294117647f32 + 0.195f32 * ctx.enemy_max_health + 0.06f32 * 0.01f32 * ctx.ap,5 => 51.17647058823529f32 + 0.21f32 * ctx.enemy_max_health + 0.06f32 * 0.01f32 * ctx.ap,6 => 56.47058823529411f32,7 => 61.76470588235294f32,8 => 67.05882352941177f32,9 => 72.35294117647058f32,10 => 77.64705882352942f32,11 => 82.94117647058823f32,12 => 88.23529411764706f32,13 => 93.52941176470588f32,14 => 98.82352941176472f32,15 => 104.11764705882352f32,16 => 109.41176470588236f32,17 => 114.70588235294116f32,18 => 120f32,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 10f32 + 0.02f32 * ctx.enemy_max_health + 0.015f32 * 0.01f32 * ctx.ap,2 => 11.764705882352942f32 + 0.025f32 * ctx.enemy_max_health + 0.015f32 * 0.01f32 * ctx.ap,3 => 13.529411764705882f32 + 0.03f32 * ctx.enemy_max_health + 0.015f32 * 0.01f32 * ctx.ap,4 => 15.294117647058822f32 + 0.035f32 * ctx.enemy_max_health + 0.015f32 * 0.01f32 * ctx.ap,5 => 17.058823529411764f32 + 0.04f32 * ctx.enemy_max_health + 0.015f32 * 0.01f32 * ctx.ap,6 => 18.823529411764707f32,7 => 20.588235294117645f32,8 => 22.352941176470587f32,9 => 24.11764705882353f32,10 => 25.88235294117647f32,11 => 27.647058823529413f32,12 => 29.41176470588235f32,13 => 31.176470588235293f32,14 => 32.94117647058823f32,15 => 34.705882352941174f32,16 => 36.47058823529411f32,17 => 38.23529411764706f32,18 => 40f32,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:610f32,per_level:99f32},mana:CachedChampionStatsMap{flat:400f32,per_level:0f32},armor:CachedChampionStatsMap{flat:34f32,per_level:4.2f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:64f32,per_level:3f32},attack_speed:CachedChampionStatsMap{flat:0.751f32,per_level:3f32},movespeed:340f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.651000022888183f32,attack_range:125f32,aram_damage_taken:1.05f32,aram_damage_dealt:0.95f32,urf_damage_taken:1.05f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static SHYVANA: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Jungle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 0.2f32 * ctx.ad + 0.25f32 * ctx.ap,2 => 0.4f32 * ctx.ad + 0.25f32 * ctx.ap,3 => 0.6f32 * ctx.ad + 0.25f32 * ctx.ap,4 => 0.8f32 * ctx.ad + 0.25f32 * ctx.ap,5 => ctx.ad + 0.25f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 10f32 + 0.1f32 * ctx.bonus_ad,2 => 15f32 + 0.1f32 * ctx.bonus_ad,3 => 20f32 + 0.1f32 * ctx.bonus_ad,4 => 25f32 + 0.1f32 * ctx.bonus_ad,5 => 30f32 + 0.1f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 85f32 + 0.5f32 * ctx.bonus_ad + 0.7f32 * ctx.ap,2 => 125f32 + 0.5f32 * ctx.bonus_ad + 0.7f32 * ctx.ap,3 => 165f32 + 0.5f32 * ctx.bonus_ad + 0.7f32 * ctx.ap,4 => 205f32 + 0.5f32 * ctx.bonus_ad + 0.7f32 * ctx.ap,5 => 245f32 + 0.5f32 * ctx.bonus_ad + 0.7f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 1f32,2 => 1.5f32,3 => 2f32,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:665f32,per_level:104f32},mana:CachedChampionStatsMap{flat:100f32,per_level:0f32},armor:CachedChampionStatsMap{flat:38f32,per_level:4.55f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:1.5f32},attack_damage:CachedChampionStatsMap{flat:66f32,per_level:3f32},attack_speed:CachedChampionStatsMap{flat:0.658f32,per_level:2.5f32},movespeed:350f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.657999992370605f32,attack_range:125f32,aram_damage_taken:0.95f32,aram_damage_dealt:1f32,urf_damage_taken:1f32,urf_damage_dealt:0.85f32,},
                    merge_data: &[],
                };pub static SINGED: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.e_level {1 => 50f32 + 0.06f32 * ctx.enemy_max_health + 0.55f32 * ctx.ap,2 => 60f32 + 0.065f32 * ctx.enemy_max_health + 0.55f32 * ctx.ap,3 => 70f32 + 0.07f32 * ctx.enemy_max_health + 0.55f32 * ctx.ap,4 => 80f32 + 0.075f32 * ctx.enemy_max_health + 0.55f32 * ctx.ap,5 => 90f32 + 0.08f32 * ctx.enemy_max_health + 0.55f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:650f32,per_level:96f32},mana:CachedChampionStatsMap{flat:330f32,per_level:45f32},armor:CachedChampionStatsMap{flat:34f32,per_level:4.2f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:63f32,per_level:3.4f32},attack_speed:CachedChampionStatsMap{flat:0.7f32,per_level:1.9f32},movespeed:345f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:125f32,aram_damage_taken:1.08f32,aram_damage_dealt:1f32,urf_damage_taken:1f32,urf_damage_dealt:0.9f32,},
                    merge_data: &[],
                };pub static SION: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_2Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Minion1), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 1.25f32,2 => 1.5833000000000002f32,3 => 1.75f32,4 => 1.85f32,5 => 1.9166999999999998f32,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 40f32 + 0.4f32 * ctx.ad,2 => 60f32 + 0.5f32 * ctx.ad,3 => 80f32 + 0.6f32 * ctx.ad,4 => 100f32 + 0.7f32 * ctx.ad,5 => 120f32 + 0.8f32 * ctx.ad,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 90f32 + 1.2f32 * ctx.ad,2 => 155f32 + 1.5f32 * ctx.ad,3 => 220f32 + 1.8f32 * ctx.ad,4 => 285f32 + 2.1f32 * ctx.ad,5 => 350f32 + 2.4f32 * ctx.ad,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 65f32 + 0.55f32 * ctx.ap,2 => 100f32 + 0.55f32 * ctx.ap,3 => 135f32 + 0.55f32 * ctx.ap,4 => 170f32 + 0.55f32 * ctx.ap,5 => 205f32 + 0.55f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:655f32,per_level:87f32},mana:CachedChampionStatsMap{flat:400f32,per_level:52f32},armor:CachedChampionStatsMap{flat:36f32,per_level:4.2f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:68f32,per_level:4f32},attack_speed:CachedChampionStatsMap{flat:0.679f32,per_level:1.3f32},movespeed:345f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.67900002002716f32,attack_range:175f32,aram_damage_taken:1.1f32,aram_damage_dealt:0.9f32,urf_damage_taken:0.92f32,urf_damage_dealt:1.1f32,},
                    merge_data: &[],
                };pub static SIVIR: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Bottom],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_2Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Minion1), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_2Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_3Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_4Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 120f32 + 1.7f32 * ctx.bonus_ad + 1.2f32 * ctx.ap,2 => 170f32 + 1.7f32 * ctx.bonus_ad + 1.2f32 * ctx.ap,3 => 220f32 + 1.7f32 * ctx.bonus_ad + 1.2f32 * ctx.ap,4 => 270f32 + 1.7f32 * ctx.bonus_ad + 1.2f32 * ctx.ap,5 => 320f32 + 1.7f32 * ctx.bonus_ad + 1.2f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 60f32 + 0.85f32 * ctx.bonus_ad + 0.6f32 * ctx.ap,2 => 85f32 + 0.85f32 * ctx.bonus_ad + 0.6f32 * ctx.ap,3 => 110f32 + 0.85f32 * ctx.bonus_ad + 0.6f32 * ctx.ap,4 => 135f32 + 0.85f32 * ctx.bonus_ad + 0.6f32 * ctx.ap,5 => 160f32 + 0.85f32 * ctx.bonus_ad + 0.6f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 24f32 + 0.34f32 * ctx.bonus_ad + 0.24f32 * ctx.ap,2 => 34f32 + 0.34f32 * ctx.bonus_ad + 0.24f32 * ctx.ap,3 => 44f32 + 0.34f32 * ctx.bonus_ad + 0.24f32 * ctx.ap,4 => 54f32 + 0.34f32 * ctx.bonus_ad + 0.24f32 * ctx.ap,5 => 64f32 + 0.34f32 * ctx.bonus_ad + 0.24f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 0.4f32 * ctx.ad,2 => 0.425f32 * ctx.ad,3 => 0.45f32 * ctx.ad,4 => 0.475f32 * ctx.ad,5 => 0.5f32 * ctx.ad,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 0.7f32 * ctx.ad,2 => 0.74375f32 * ctx.ad,3 => 0.7875f32 * ctx.ad,4 => 0.83125f32 * ctx.ad,5 => 0.875f32 * ctx.ad,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 0.26f32 * ctx.ad,2 => 0.27625f32 * ctx.ad,3 => 0.2925f32 * ctx.ad,4 => 0.30875f32 * ctx.ad,5 => 0.325f32 * ctx.ad,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 0.455f32 * ctx.ad,2 => 0.4834375f32 * ctx.ad,3 => 0.511875f32 * ctx.ad,4 => 0.5403125f32 * ctx.ad,5 => 0.56875f32 * ctx.ad,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:600f32,per_level:104f32},mana:CachedChampionStatsMap{flat:340f32,per_level:45f32},armor:CachedChampionStatsMap{flat:30f32,per_level:4.45f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:60f32,per_level:2.5f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:2f32},movespeed:335f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:500f32,aram_damage_taken:1.05f32,aram_damage_dealt:0.93f32,urf_damage_taken:1.05f32,urf_damage_dealt:0.85f32,},
                    merge_data: &[],
                };pub static SKARNER: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Jungle,Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_2), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_3Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Monster2), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Unknown, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 0.2f32,2 => 0.25f32,3 => 0.3f32,4 => 0.35f32,5 => 0.4f32,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 30f32 + 2.4f32 * ctx.bonus_ad + 0.09f32 * ctx.bonus_health,2 => 60f32 + 2.4f32 * ctx.bonus_ad + 0.09f32 * ctx.bonus_health,3 => 90f32 + 2.4f32 * ctx.bonus_ad + 0.09f32 * ctx.bonus_health,4 => 120f32 + 2.4f32 * ctx.bonus_ad + 0.09f32 * ctx.bonus_health,5 => 150f32 + 2.4f32 * ctx.bonus_ad + 0.09f32 * ctx.bonus_health,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 10f32 + 0.8f32 * ctx.bonus_ad + 0.03f32 * ctx.bonus_health,2 => 20f32 + 0.8f32 * ctx.bonus_ad + 0.03f32 * ctx.bonus_health,3 => 30f32 + 0.8f32 * ctx.bonus_ad + 0.03f32 * ctx.bonus_health,4 => 40f32 + 0.8f32 * ctx.bonus_ad + 0.03f32 * ctx.bonus_health,5 => 50f32 + 0.8f32 * ctx.bonus_ad + 0.03f32 * ctx.bonus_health,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 50f32 + 0.8f32 * ctx.ap,2 => 70f32 + 0.8f32 * ctx.ap,3 => 90f32 + 0.8f32 * ctx.ap,4 => 110f32 + 0.8f32 * ctx.ap,5 => 130f32 + 0.8f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 150f32 + ctx.ap,2 => 250f32 + ctx.ap,3 => 350f32 + ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:630f32,per_level:110f32},mana:CachedChampionStatsMap{flat:320f32,per_level:40f32},armor:CachedChampionStatsMap{flat:33f32,per_level:4.5f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:63f32,per_level:3f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:2f32},movespeed:335f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:150f32,aram_damage_taken:1f32,aram_damage_dealt:1f32,urf_damage_taken:1f32,urf_damage_dealt:1.1f32,},
                    merge_data: &[],
                };pub static SMOLDER: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Bottom,Position::Middle,Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Max), 
                damage_type: DamageType::Unknown, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_2Max), 
                damage_type: DamageType::Unknown, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_4Max), 
                damage_type: DamageType::Unknown, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_5Max), 
                damage_type: DamageType::Unknown, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_3Min), 
                damage_type: DamageType::Unknown, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_6Min), 
                damage_type: DamageType::Unknown, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_3Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_2Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_2Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_2Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 65f32 + 1.3f32 * ctx.bonus_ad,2 => 80f32 + 1.3f32 * ctx.bonus_ad,3 => 95f32 + 1.3f32 * ctx.bonus_ad,4 => 110f32 + 1.3f32 * ctx.bonus_ad,5 => 125f32 + 1.3f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 113.75f32 + 2.275f32 * ctx.bonus_ad,2 => 140f32 + 2.275f32 * ctx.bonus_ad,3 => 166.25f32 + 2.275f32 * ctx.bonus_ad,4 => 192.5f32 + 2.275f32 * ctx.bonus_ad,5 => 218.75f32 + 2.275f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 32.5f32 + 0.65f32 * ctx.bonus_ad,2 => 40f32 + 0.65f32 * ctx.bonus_ad,3 => 47.5f32 + 0.65f32 * ctx.bonus_ad,4 => 55f32 + 0.65f32 * ctx.bonus_ad,5 => 62.5f32 + 0.65f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 56.875f32 + 1.1375f32 * ctx.bonus_ad,2 => 70f32 + 1.1375f32 * ctx.bonus_ad,3 => 83.125f32 + 1.1375f32 * ctx.bonus_ad,4 => 96.25f32 + 1.1375f32 * ctx.bonus_ad,5 => 109.375f32 + 1.1375f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 139.75f32 + 2.795f32 * ctx.bonus_ad,2 => 172f32 + 2.795f32 * ctx.bonus_ad,3 => 204.25f32 + 2.795f32 * ctx.bonus_ad,4 => 236.5f32 + 2.795f32 * ctx.bonus_ad,5 => 268.75f32 + 2.795f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 69.875f32 + 1.3975f32 * ctx.bonus_ad,2 => 86f32 + 1.3975f32 * ctx.bonus_ad,3 => 102.125f32 + 1.3975f32 * ctx.bonus_ad,4 => 118.25f32 + 1.3975f32 * ctx.bonus_ad,5 => 134.375f32 + 1.3975f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 70f32 + 1.25f32 * ctx.bonus_ad + 0.8f32 * ctx.ap,2 => 105f32 + 1.25f32 * ctx.bonus_ad + 0.8f32 * ctx.ap,3 => 140f32 + 1.25f32 * ctx.bonus_ad + 0.8f32 * ctx.ap,4 => 175f32 + 1.25f32 * ctx.bonus_ad + 0.8f32 * ctx.ap,5 => 210f32 + 1.25f32 * ctx.bonus_ad + 0.8f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 60f32 + 0.6f32 * ctx.bonus_ad,2 => 70f32 + 0.6f32 * ctx.bonus_ad,3 => 80f32 + 0.6f32 * ctx.bonus_ad,4 => 90f32 + 0.6f32 * ctx.bonus_ad,5 => 100f32 + 0.6f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 10f32 + 0.65f32 * ctx.bonus_ad + 0.8f32 * ctx.ap,2 => 35f32 + 0.65f32 * ctx.bonus_ad + 0.8f32 * ctx.ap,3 => 60f32 + 0.65f32 * ctx.bonus_ad + 0.8f32 * ctx.ap,4 => 85f32 + 0.65f32 * ctx.bonus_ad + 0.8f32 * ctx.ap,5 => 110f32 + 0.65f32 * ctx.bonus_ad + 0.8f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 10f32 + 0.3f32 * ctx.ad,2 => 15f32 + 0.3f32 * ctx.ad,3 => 20f32 + 0.3f32 * ctx.ad,4 => 25f32 + 0.3f32 * ctx.ad,5 => 30f32 + 0.3f32 * ctx.ad,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 50f32 + 1.5f32 * ctx.ad,2 => 75f32 + 1.5f32 * ctx.ad,3 => 100f32 + 1.5f32 * ctx.ad,4 => 125f32 + 1.5f32 * ctx.ad,5 => 150f32 + 1.5f32 * ctx.ad,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 100f32 + 0.5f32 * ctx.bonus_ad + 0.75f32 * ctx.ap,2 => 135f32 + 0.5f32 * ctx.bonus_ad + 0.75f32 * ctx.ap,3 => 170f32 + 0.5f32 * ctx.bonus_ad + 0.75f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 200f32 + 1.1f32 * ctx.bonus_ad + ctx.ap,2 => 300f32 + 1.1f32 * ctx.bonus_ad + ctx.ap,3 => 400f32 + 1.1f32 * ctx.bonus_ad + ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:575f32,per_level:100f32},mana:CachedChampionStatsMap{flat:300f32,per_level:40f32},armor:CachedChampionStatsMap{flat:24f32,per_level:4.7f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:60f32,per_level:2.3f32},attack_speed:CachedChampionStatsMap{flat:0.638f32,per_level:4f32},movespeed:330f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.638f32,attack_range:550f32,aram_damage_taken:1.02f32,aram_damage_dealt:1f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static SONA: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Support],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_2), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::Minion1), 
                damage_type: DamageType::Unknown, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 10f32 + 0.1f32 * ctx.ap,2 => 15f32,3 => 20f32,4 => 25f32,5 => 30f32,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 50f32 + 0.4f32 * ctx.ap,2 => 85f32,3 => 120f32,4 => 155f32,5 => 190f32,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 25f32,2 => 45f32,3 => 65f32,4 => 85f32,5 => 105f32,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 150f32 + 0.5f32 * ctx.ap,2 => 250f32 + 0.5f32 * ctx.ap,3 => 350f32 + 0.5f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:550f32,per_level:91f32},mana:CachedChampionStatsMap{flat:340f32,per_level:45f32},armor:CachedChampionStatsMap{flat:26f32,per_level:4.2f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:49f32,per_level:3f32},attack_speed:CachedChampionStatsMap{flat:0.644f32,per_level:2.3f32},movespeed:325f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.643999993801116f32,attack_range:550f32,aram_damage_taken:1f32,aram_damage_dealt:1f32,urf_damage_taken:1f32,urf_damage_dealt:0.9f32,},
                    merge_data: &[],
                };pub static SORAKA: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Support],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_2Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 85f32 + 0.35f32 * ctx.ap,2 => 120f32 + 0.35f32 * ctx.ap,3 => 155f32 + 0.35f32 * ctx.ap,4 => 190f32 + 0.35f32 * ctx.ap,5 => 225f32 + 0.35f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 0.0f32,2 => 0.0f32,3 => 0.0f32,4 => 0.0f32,5 => 0.0f32,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 70f32 + 0.4f32 * ctx.ap,2 => 95f32 + 0.4f32 * ctx.ap,3 => 120f32 + 0.4f32 * ctx.ap,4 => 145f32 + 0.4f32 * ctx.ap,5 => 170f32 + 0.4f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:605f32,per_level:88f32},mana:CachedChampionStatsMap{flat:425f32,per_level:40f32},armor:CachedChampionStatsMap{flat:32f32,per_level:5f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:50f32,per_level:3f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:2.14f32},movespeed:325f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:550f32,aram_damage_taken:1f32,aram_damage_dealt:1f32,urf_damage_taken:0.92f32,urf_damage_dealt:1.05f32,},
                    merge_data: &[],
                };pub static SWAIN: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Bottom,Position::Middle,Position::Support],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_3Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_2Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_2Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 60f32 + 0.45f32 * ctx.ap,2 => 90f32 + 0.45f32 * ctx.ap,3 => 120f32 + 0.45f32 * ctx.ap,4 => 150f32 + 0.45f32 * ctx.ap,5 => 180f32 + 0.45f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 120f32 + 0.9f32 * ctx.ap,2 => 180f32 + 0.9f32 * ctx.ap,3 => 240f32 + 0.9f32 * ctx.ap,4 => 300f32 + 0.9f32 * ctx.ap,5 => 360f32 + 0.9f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 15f32 + 0.1125f32 * ctx.ap,2 => 22.5f32 + 0.1125f32 * ctx.ap,3 => 30f32 + 0.1125f32 * ctx.ap,4 => 37.5f32 + 0.1125f32 * ctx.ap,5 => 45f32 + 0.1125f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 70f32 + 0.6f32 * ctx.ap,2 => 105f32 + 0.6f32 * ctx.ap,3 => 140f32 + 0.6f32 * ctx.ap,4 => 175f32 + 0.6f32 * ctx.ap,5 => 210f32 + 0.6f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 35f32 + 0.3f32 * ctx.ap,2 => 52.5f32 + 0.3f32 * ctx.ap,3 => 70f32 + 0.3f32 * ctx.ap,4 => 87.5f32 + 0.3f32 * ctx.ap,5 => 105f32 + 0.3f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 80f32 + 0.7f32 * ctx.ap,2 => 120f32 + 0.7f32 * ctx.ap,3 => 160f32 + 0.7f32 * ctx.ap,4 => 200f32 + 0.7f32 * ctx.ap,5 => 240f32 + 0.7f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:595f32,per_level:99f32},mana:CachedChampionStatsMap{flat:400f32,per_level:29f32},armor:CachedChampionStatsMap{flat:25f32,per_level:4.7f32},magic_resist:CachedChampionStatsMap{flat:31f32,per_level:1.55f32},attack_damage:CachedChampionStatsMap{flat:58f32,per_level:2.7f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:2.11f32},movespeed:330f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:525f32,aram_damage_taken:1.15f32,aram_damage_dealt:0.95f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static SYLAS: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Middle,Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_4Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_5Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_2Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_3Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 24f32 + 0.32f32 * ctx.ap,2 => 46f32 + 0.32f32 * ctx.ap,3 => 68f32 + 0.32f32 * ctx.ap,4 => 90f32 + 0.32f32 * ctx.ap,5 => 112f32 + 0.32f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 64f32 + 0.72f32 * ctx.ap,2 => 106f32 + 0.72f32 * ctx.ap,3 => 148f32 + 0.72f32 * ctx.ap,4 => 190f32 + 0.72f32 * ctx.ap,5 => 232f32 + 0.72f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 40f32 + 0.4f32 * ctx.ap,2 => 60f32 + 0.4f32 * ctx.ap,3 => 80f32 + 0.4f32 * ctx.ap,4 => 100f32 + 0.4f32 * ctx.ap,5 => 120f32 + 0.4f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 60f32 + 0.8f32 * ctx.ap,2 => 115f32 + 0.8f32 * ctx.ap,3 => 170f32 + 0.8f32 * ctx.ap,4 => 225f32 + 0.8f32 * ctx.ap,5 => 280f32 + 0.8f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 100f32 + 1.2f32 * ctx.ap,2 => 175f32 + 1.2f32 * ctx.ap,3 => 250f32 + 1.2f32 * ctx.ap,4 => 325f32 + 1.2f32 * ctx.ap,5 => 400f32 + 1.2f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 75f32 + 0.6f32 * ctx.ap,2 => 110f32 + 0.6f32 * ctx.ap,3 => 145f32 + 0.6f32 * ctx.ap,4 => 180f32 + 0.6f32 * ctx.ap,5 => 215f32 + 0.6f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:600f32,per_level:122f32},mana:CachedChampionStatsMap{flat:400f32,per_level:70f32},armor:CachedChampionStatsMap{flat:29f32,per_level:5.2f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.55f32},attack_damage:CachedChampionStatsMap{flat:61f32,per_level:3f32},attack_speed:CachedChampionStatsMap{flat:0.645f32,per_level:3.5f32},movespeed:340f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.644999980926513f32,attack_range:175f32,aram_damage_taken:1f32,aram_damage_dealt:1.05f32,urf_damage_taken:1f32,urf_damage_dealt:0.95f32,},
                    merge_data: &[],
                };pub static SYNDRA: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Middle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_2), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_3Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 80f32 + 0.65f32 * ctx.ap,2 => 115f32 + 0.65f32 * ctx.ap,3 => 150f32 + 0.65f32 * ctx.ap,4 => 185f32 + 0.65f32 * ctx.ap,5 => 220f32 + 0.65f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 8.4f32 + 0.092f32 * ctx.ap,2 => 12.6f32 + 0.099f32 * ctx.ap,3 => 16.8f32 + 0.106f32 * ctx.ap,4 => 21f32 + 0.113f32 * ctx.ap,5 => 25.2f32 + 0.12f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 78.4f32 + 0.742f32 * ctx.ap,2 => 117.6f32 + 0.7490000000000001f32 * ctx.ap,3 => 156.8f32 + 0.7559999999999999f32 * ctx.ap,4 => 196f32 + 0.763f32 * ctx.ap,5 => 235.2f32 + 0.77f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 70f32 + 0.65f32 * ctx.ap,2 => 105f32 + 0.65f32 * ctx.ap,3 => 140f32 + 0.65f32 * ctx.ap,4 => 175f32 + 0.65f32 * ctx.ap,5 => 210f32 + 0.65f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 60f32 + 0.6f32 * ctx.ap,2 => 95f32 + 0.6f32 * ctx.ap,3 => 130f32 + 0.6f32 * ctx.ap,4 => 165f32 + 0.6f32 * ctx.ap,5 => 200f32 + 0.6f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 10f32,2 => 20f32,3 => 30f32,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:563f32,per_level:100f32},mana:CachedChampionStatsMap{flat:480f32,per_level:40f32},armor:CachedChampionStatsMap{flat:25f32,per_level:4f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:54f32,per_level:2.9f32},attack_speed:CachedChampionStatsMap{flat:0.658f32,per_level:2f32},movespeed:330f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:550f32,aram_damage_taken:1f32,aram_damage_dealt:1.05f32,urf_damage_taken:1.1f32,urf_damage_dealt:0.9f32,},
                    merge_data: &[],
                };pub static TAHMKENCH: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Support,Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 0.0f32,2 => 0.0f32,3 => 0.0f32,4 => 0.0f32,5 => 0.0f32,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:640f32,per_level:103f32},mana:CachedChampionStatsMap{flat:325f32,per_level:50f32},armor:CachedChampionStatsMap{flat:39f32,per_level:4.7f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:56f32,per_level:3.2f32},attack_speed:CachedChampionStatsMap{flat:0.658f32,per_level:2.5f32},movespeed:335f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.657999992370605f32,attack_range:175f32,aram_damage_taken:1.05f32,aram_damage_dealt:1f32,urf_damage_taken:0.85f32,urf_damage_dealt:1.2f32,},
                    merge_data: &[],
                };pub static TALIYAH: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Jungle,Position::Middle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_3Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_2Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_4Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_2Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_3Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 143f32 + 1.3f32 * ctx.ap,2 => 188.5f32 + 1.3f32 * ctx.ap,3 => 234f32 + 1.3f32 * ctx.ap,4 => 279.5f32 + 1.3f32 * ctx.ap,5 => 325f32 + 1.3f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 55f32 + 0.5f32 * ctx.ap,2 => 72.5f32 + 0.5f32 * ctx.ap,3 => 90f32 + 0.5f32 * ctx.ap,4 => 107.5f32 + 0.5f32 * ctx.ap,5 => 125f32 + 0.5f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 22f32 + 0.2f32 * ctx.ap,2 => 29f32 + 0.2f32 * ctx.ap,3 => 36f32 + 0.2f32 * ctx.ap,4 => 43f32 + 0.2f32 * ctx.ap,5 => 50f32 + 0.2f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 99f32 + 0.9f32 * ctx.ap,2 => 130.5f32 + 0.9f32 * ctx.ap,3 => 162f32 + 0.9f32 * ctx.ap,4 => 193.5f32 + 0.9f32 * ctx.ap,5 => 225f32 + 0.9f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 25f32 + 0.3f32 * ctx.ap,2 => 40f32 + 0.3f32 * ctx.ap,3 => 55f32 + 0.3f32 * ctx.ap,4 => 70f32 + 0.3f32 * ctx.ap,5 => 85f32 + 0.3f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 60f32 + 0.6f32 * ctx.ap,2 => 105f32 + 0.6f32 * ctx.ap,3 => 150f32 + 0.6f32 * ctx.ap,4 => 195f32 + 0.6f32 * ctx.ap,5 => 240f32 + 0.6f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 62.5f32 + 0.75f32 * ctx.ap,2 => 100f32 + 0.75f32 * ctx.ap,3 => 137.5f32 + 0.75f32 * ctx.ap,4 => 175f32 + 0.75f32 * ctx.ap,5 => 212.5f32 + 0.75f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:550f32,per_level:104f32},mana:CachedChampionStatsMap{flat:470f32,per_level:30f32},armor:CachedChampionStatsMap{flat:18f32,per_level:4.7f32},magic_resist:CachedChampionStatsMap{flat:28f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:58f32,per_level:3.3f32},attack_speed:CachedChampionStatsMap{flat:0.658f32,per_level:1.36f32},movespeed:330f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:525f32,aram_damage_taken:1f32,aram_damage_dealt:1f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static TALON: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Jungle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_2Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_3Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_2Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 65f32 + ctx.bonus_ad,2 => 85f32 + ctx.bonus_ad,3 => 105f32 + ctx.bonus_ad,4 => 125f32 + ctx.bonus_ad,5 => 145f32 + ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 97.5f32 + 1.5f32 * ctx.bonus_ad,2 => 127.5f32 + 1.5f32 * ctx.bonus_ad,3 => 157.5f32 + 1.5f32 * ctx.bonus_ad,4 => 187.5f32 + 1.5f32 * ctx.bonus_ad,5 => 217.5f32 + 1.5f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 0.4f32,2 => 0.45f32,3 => 0.5f32,4 => 0.55f32,5 => 0.6f32,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 50f32 + 0.4f32 * ctx.bonus_ad,2 => 60f32 + 0.4f32 * ctx.bonus_ad,3 => 70f32 + 0.4f32 * ctx.bonus_ad,4 => 80f32 + 0.4f32 * ctx.bonus_ad,5 => 90f32 + 0.4f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 60f32 + 0.9f32 * ctx.bonus_ad,2 => 90f32 + 0.9f32 * ctx.bonus_ad,3 => 120f32 + 0.9f32 * ctx.bonus_ad,4 => 150f32 + 0.9f32 * ctx.bonus_ad,5 => 180f32 + 0.9f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 0.4f32,2 => 0.55f32,3 => 0.7f32,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:658f32,per_level:109f32},mana:CachedChampionStatsMap{flat:400f32,per_level:37f32},armor:CachedChampionStatsMap{flat:30f32,per_level:4.7f32},magic_resist:CachedChampionStatsMap{flat:36f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:68f32,per_level:3.1f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:2.9f32},movespeed:335f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:125f32,aram_damage_taken:0.95f32,aram_damage_dealt:1.05f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static TARIC: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Middle,Position::Support],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.e_level {1 => 90f32 + 0.5f32 * ctx.ap + 0.5f32 * ctx.bonus_armor,2 => 130f32 + 0.5f32 * ctx.ap + 0.5f32 * ctx.bonus_armor,3 => 170f32 + 0.5f32 * ctx.ap + 0.5f32 * ctx.bonus_armor,4 => 210f32 + 0.5f32 * ctx.ap + 0.5f32 * ctx.bonus_armor,5 => 250f32 + 0.5f32 * ctx.ap + 0.5f32 * ctx.bonus_armor,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:645f32,per_level:99f32},mana:CachedChampionStatsMap{flat:300f32,per_level:60f32},armor:CachedChampionStatsMap{flat:40f32,per_level:4.3f32},magic_resist:CachedChampionStatsMap{flat:28f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:55f32,per_level:3.5f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:2f32},movespeed:340f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:150f32,aram_damage_taken:1.1f32,aram_damage_dealt:1f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static TEEMO: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Jungle,Position::Support,Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_3Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_2Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::Monster1), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::Monster2), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::Monster3), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 0.0f32,2 => 0.0f32,3 => 0.0f32,4 => 0.0f32,5 => 0.0f32,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 24f32 + 0.4f32 * ctx.ap,2 => 48f32 + 0.4f32 * ctx.ap,3 => 72f32 + 0.4f32 * ctx.ap,4 => 96f32 + 0.4f32 * ctx.ap,5 => 120f32 + 0.4f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 9f32 + 0.3f32 * ctx.ap,2 => 23f32 + 0.3f32 * ctx.ap,3 => 37f32 + 0.3f32 * ctx.ap,4 => 51f32 + 0.3f32 * ctx.ap,5 => 65f32 + 0.3f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 6f32 + 0.1f32 * ctx.ap,2 => 12f32 + 0.1f32 * ctx.ap,3 => 18f32 + 0.1f32 * ctx.ap,4 => 24f32 + 0.1f32 * ctx.ap,5 => 30f32 + 0.1f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 13.05f32 + 0.435f32 * ctx.ap,2 => 33.35f32 + 0.435f32 * ctx.ap,3 => 53.65f32 + 0.435f32 * ctx.ap,4 => 73.95f32 + 0.435f32 * ctx.ap,5 => 94.25f32 + 0.435f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 8.7f32 + 0.145f32 * ctx.ap,2 => 17.4f32 + 0.145f32 * ctx.ap,3 => 26.1f32 + 0.145f32 * ctx.ap,4 => 34.8f32 + 0.145f32 * ctx.ap,5 => 43.5f32 + 0.145f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 34.8f32 + 0.58f32 * ctx.ap,2 => 69.6f32 + 0.58f32 * ctx.ap,3 => 104.4f32 + 0.58f32 * ctx.ap,4 => 139.2f32 + 0.58f32 * ctx.ap,5 => 174f32 + 0.58f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:615f32,per_level:104f32},mana:CachedChampionStatsMap{flat:334f32,per_level:25f32},armor:CachedChampionStatsMap{flat:24f32,per_level:4.95f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:54f32,per_level:3f32},attack_speed:CachedChampionStatsMap{flat:0.69f32,per_level:3.38f32},movespeed:330f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.689999997615814f32,attack_range:500f32,aram_damage_taken:1.1f32,aram_damage_dealt:0.95f32,urf_damage_taken:1f32,urf_damage_dealt:0.9f32,},
                    merge_data: &[],
                };pub static THRESH: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Support],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_2), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_3), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.e_level {1 => 75f32 + 0.7f32 * ctx.ap,2 => 120f32 + 0.7f32 * ctx.ap,3 => 165f32 + 0.7f32 * ctx.ap,4 => 210f32 + 0.7f32 * ctx.ap,5 => 255f32 + 0.7f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 0.2f32,2 => 0.25f32,3 => 0.3f32,4 => 0.35f32,5 => 0.4f32,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 1.7f32 * ctx.thresh_stacks + 0f32 * ctx.ad,2 => 1.7f32 * ctx.thresh_stacks + 0f32 * ctx.ad,3 => 1.7f32 * ctx.thresh_stacks + 0f32 * ctx.ad,4 => 1.7f32 * ctx.thresh_stacks + 0f32 * ctx.ad,5 => 1.7f32 * ctx.thresh_stacks + 0f32 * ctx.ad,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 250f32 + ctx.ap,2 => 400f32 + ctx.ap,3 => 550f32 + ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:620f32,per_level:120f32},mana:CachedChampionStatsMap{flat:274f32,per_level:44f32},armor:CachedChampionStatsMap{flat:33f32,per_level:0f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.55f32},attack_damage:CachedChampionStatsMap{flat:56f32,per_level:2.2f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:3.5f32},movespeed:330f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:450f32,aram_damage_taken:1.05f32,aram_damage_dealt:1f32,urf_damage_taken:0.87f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static TRISTANA: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Bottom,Position::Middle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::Minion1), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.w_level {1 => 70f32 + ctx.bonus_ad + 0.5f32 * ctx.ap,2 => 105f32 + ctx.bonus_ad + 0.5f32 * ctx.ap,3 => 140f32 + ctx.bonus_ad + 0.5f32 * ctx.ap,4 => 175f32 + ctx.bonus_ad + 0.5f32 * ctx.ap,5 => 210f32 + ctx.bonus_ad + 0.5f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 15f32 + 0.25f32 * ctx.bonus_ad + 0.125f32 * ctx.ap,2 => 17.5f32 + 0.275f32 * ctx.bonus_ad + 0.125f32 * ctx.ap,3 => 20f32 + 0.3f32 * ctx.bonus_ad + 0.125f32 * ctx.ap,4 => 22.5f32 + 0.325f32 * ctx.bonus_ad + 0.125f32 * ctx.ap,5 => 25f32 + 0.35f32 * ctx.bonus_ad + 0.125f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 45f32 + 0.25f32 * ctx.ap,2 => 60f32 + 0.25f32 * ctx.ap,3 => 75f32 + 0.25f32 * ctx.ap,4 => 90f32 + 0.25f32 * ctx.ap,5 => 105f32 + 0.25f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 600f32,2 => 800f32,3 => 1000f32,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:640f32,per_level:102f32},mana:CachedChampionStatsMap{flat:300f32,per_level:32f32},armor:CachedChampionStatsMap{flat:30f32,per_level:4f32},magic_resist:CachedChampionStatsMap{flat:28f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:60f32,per_level:2.5f32},attack_speed:CachedChampionStatsMap{flat:0.656f32,per_level:1.5f32},movespeed:325f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.694f32,attack_range:550f32,aram_damage_taken:0.95f32,aram_damage_dealt:1.05f32,urf_damage_taken:1.05f32,urf_damage_dealt:0.9f32,},
                    merge_data: &[],
                };pub static TRUNDLE: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Jungle,Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_3), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_2Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_2Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_3Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 10f32 + 0.15f32 * ctx.ad,2 => 30f32 + 0.25f32 * ctx.ad,3 => 50f32 + 0.35f32 * ctx.ad,4 => 70f32 + 0.45f32 * ctx.ad,5 => 90f32 + 0.55f32 * ctx.ad,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 10f32,2 => 12.5f32,3 => 15f32,4 => 17.5f32,5 => 20f32,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 20f32,2 => 25f32,3 => 30f32,4 => 35f32,5 => 40f32,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 0.2f32 * ctx.enemy_max_health + 0.02f32 * 0.01f32 * ctx.ap,2 => 0.25f32 * ctx.enemy_max_health + 0.02f32 * 0.01f32 * ctx.ap,3 => 0.3f32 * ctx.enemy_max_health + 0.02f32 * 0.01f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 0.1f32 * ctx.enemy_max_health + 0.01f32 * 0.01f32 * ctx.ap,2 => 0.125f32 * ctx.enemy_max_health + 0.01f32 * 0.01f32 * ctx.ap,3 => 0.15f32 * ctx.enemy_max_health + 0.01f32 * 0.01f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 0.025f32 * ctx.enemy_max_health + 0.0025f32 * 0.01f32 * ctx.ap,2 => 0.03125f32 * ctx.enemy_max_health + 0.0025f32 * 0.01f32 * ctx.ap,3 => 0.0375f32 * ctx.enemy_max_health + 0.0025f32 * 0.01f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:650f32,per_level:110f32},mana:CachedChampionStatsMap{flat:340f32,per_level:45f32},armor:CachedChampionStatsMap{flat:37f32,per_level:3.9f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:68f32,per_level:4f32},attack_speed:CachedChampionStatsMap{flat:0.67f32,per_level:2.9f32},movespeed:350f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.6700000166893f32,attack_range:175f32,aram_damage_taken:1.05f32,aram_damage_dealt:1f32,urf_damage_taken:0.95f32,urf_damage_dealt:0.95f32,},
                    merge_data: &[],
                };pub static TRYNDAMERE: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1), 
                damage_type: DamageType::Unknown, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Unknown, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 0.5f32 + 0.012f32 * ctx.ap,2 => 0.95f32 + 0.012f32 * ctx.ap,3 => 1.4f32 + 0.012f32 * ctx.ap,4 => 1.85f32 + 0.012f32 * ctx.ap,5 => 2.3f32 + 0.012f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 0.3f32,2 => 0.375f32,3 => 0.45f32,4 => 0.525f32,5 => 0.6f32,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 75f32 + 1.3f32 * ctx.bonus_ad + 0.8f32 * ctx.ap,2 => 105f32 + 1.3f32 * ctx.bonus_ad + 0.8f32 * ctx.ap,3 => 135f32 + 1.3f32 * ctx.bonus_ad + 0.8f32 * ctx.ap,4 => 165f32 + 1.3f32 * ctx.bonus_ad + 0.8f32 * ctx.ap,5 => 195f32 + 1.3f32 * ctx.bonus_ad + 0.8f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:696f32,per_level:108f32},mana:CachedChampionStatsMap{flat:100f32,per_level:0f32},armor:CachedChampionStatsMap{flat:33f32,per_level:4.8f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:66f32,per_level:4f32},attack_speed:CachedChampionStatsMap{flat:0.67f32,per_level:3.4f32},movespeed:345f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.694f32,attack_range:175f32,aram_damage_taken:0.9f32,aram_damage_dealt:1.1f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static TWISTEDFATE: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Bottom,Position::Middle,Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_2Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 60f32 + 0.5f32 * ctx.bonus_ad + 0.85f32 * ctx.ap,2 => 105f32 + 0.5f32 * ctx.bonus_ad + 0.85f32 * ctx.ap,3 => 150f32 + 0.5f32 * ctx.bonus_ad + 0.85f32 * ctx.ap,4 => 195f32 + 0.5f32 * ctx.bonus_ad + 0.85f32 * ctx.ap,5 => 240f32 + 0.5f32 * ctx.bonus_ad + 0.85f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 40f32 + ctx.ad + ctx.ap,2 => 60f32 + ctx.ad + ctx.ap,3 => 80f32 + ctx.ad + ctx.ap,4 => 100f32 + ctx.ad + ctx.ap,5 => 120f32 + ctx.ad + ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 65f32 + 0.2f32 * ctx.bonus_ad + 0.4f32 * ctx.ap,2 => 90f32 + 0.2f32 * ctx.bonus_ad + 0.4f32 * ctx.ap,3 => 115f32 + 0.2f32 * ctx.bonus_ad + 0.4f32 * ctx.ap,4 => 140f32 + 0.2f32 * ctx.bonus_ad + 0.4f32 * ctx.ap,5 => 165f32 + 0.2f32 * ctx.bonus_ad + 0.4f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:604f32,per_level:108f32},mana:CachedChampionStatsMap{flat:333f32,per_level:39f32},armor:CachedChampionStatsMap{flat:24f32,per_level:4.35f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:52f32,per_level:2.5f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:2.5f32},movespeed:330f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.651000022888183f32,attack_range:525f32,aram_damage_taken:1f32,aram_damage_dealt:1f32,urf_damage_taken:1f32,urf_damage_dealt:0.95f32,},
                    merge_data: &[],
                };pub static TWITCH: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Bottom,Position::Support],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.e_level {1 => 15f32 + 0.35f32 * ctx.bonus_ad,2 => 20f32 + 0.35f32 * ctx.bonus_ad,3 => 25f32 + 0.35f32 * ctx.bonus_ad,4 => 30f32 + 0.35f32 * ctx.bonus_ad,5 => 35f32 + 0.35f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 30f32,2 => 45f32,3 => 60f32,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:630f32,per_level:104f32},mana:CachedChampionStatsMap{flat:300f32,per_level:40f32},armor:CachedChampionStatsMap{flat:27f32,per_level:4.2f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:59f32,per_level:3.1f32},attack_speed:CachedChampionStatsMap{flat:0.679f32,per_level:3.38f32},movespeed:330f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.67900002002716f32,attack_range:550f32,aram_damage_taken:0.95f32,aram_damage_dealt:1f32,urf_damage_taken:1.1f32,urf_damage_dealt:0.85f32,},
                    merge_data: &[],
                };pub static UDYR: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Jungle,Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1), 
                damage_type: DamageType::Unknown, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_3), 
                damage_type: DamageType::Unknown, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_2Max), 
                damage_type: DamageType::Unknown, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_2Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 0.03f32 * ctx.enemy_max_health + 0.04f32 * 0.01f32 * ctx.bonus_ad,2 => 0.04f32 * ctx.enemy_max_health + 0.04f32 * 0.01f32 * ctx.bonus_ad,3 => 0.05f32 * ctx.enemy_max_health + 0.04f32 * 0.01f32 * ctx.bonus_ad,4 => 0.06f32 * ctx.enemy_max_health + 0.04f32 * 0.01f32 * ctx.bonus_ad,5 => 0.07f32 * ctx.enemy_max_health + 0.04f32 * 0.01f32 * ctx.bonus_ad,6 => 0.08f32 * ctx.enemy_max_health + 0.04f32 * 0.01f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 5f32 + 0.25f32 * ctx.bonus_ad,2 => 11f32 + 0.25f32 * ctx.bonus_ad,3 => 17f32 + 0.25f32 * ctx.bonus_ad,4 => 23f32 + 0.25f32 * ctx.bonus_ad,5 => 29f32 + 0.25f32 * ctx.bonus_ad,6 => 35f32 + 0.25f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 0.06f32 * ctx.enemy_max_health + 0.08f32 * 0.01f32 * ctx.bonus_ad,2 => 0.08f32 * ctx.enemy_max_health + 0.08f32 * 0.01f32 * ctx.bonus_ad,3 => 0.1f32 * ctx.enemy_max_health + 0.08f32 * 0.01f32 * ctx.bonus_ad,4 => 0.12f32 * ctx.enemy_max_health + 0.08f32 * 0.01f32 * ctx.bonus_ad,5 => 0.14f32 * ctx.enemy_max_health + 0.08f32 * 0.01f32 * ctx.bonus_ad,6 => 0.16f32 * ctx.enemy_max_health + 0.08f32 * 0.01f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 0.15f32,2 => 0.18f32,3 => 0.21f32,4 => 0.24f32,5 => 0.27f32,6 => 0.3f32,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 10f32 + 0.175f32 * ctx.ap,2 => 18f32 + 0.175f32 * ctx.ap,3 => 26f32 + 0.175f32 * ctx.ap,4 => 34f32 + 0.175f32 * ctx.ap,5 => 42f32 + 0.175f32 * ctx.ap,6 => 50f32 + 0.175f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:664f32,per_level:92f32},mana:CachedChampionStatsMap{flat:271f32,per_level:50f32},armor:CachedChampionStatsMap{flat:31f32,per_level:4.7f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:62f32,per_level:4f32},attack_speed:CachedChampionStatsMap{flat:0.65f32,per_level:3f32},movespeed:350f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.6499999761581421f32,attack_range:125f32,aram_damage_taken:1f32,aram_damage_dealt:1f32,urf_damage_taken:0.92f32,urf_damage_dealt:1.1f32,},
                    merge_data: &[],
                };pub static URGOT: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Unknown, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 25f32 + 0.7f32 * ctx.ad,2 => 70f32 + 0.7f32 * ctx.ad,3 => 115f32 + 0.7f32 * ctx.ad,4 => 160f32 + 0.7f32 * ctx.ad,5 => 205f32 + 0.7f32 * ctx.ad,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 90f32 + ctx.bonus_ad,2 => 120f32 + ctx.bonus_ad,3 => 150f32 + ctx.bonus_ad,4 => 180f32 + ctx.bonus_ad,5 => 210f32 + ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 100f32 + 0.5f32 * ctx.bonus_ad,2 => 225f32 + 0.5f32 * ctx.bonus_ad,3 => 350f32 + 0.5f32 * ctx.bonus_ad,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:655f32,per_level:102f32},mana:CachedChampionStatsMap{flat:340f32,per_level:45f32},armor:CachedChampionStatsMap{flat:36f32,per_level:5f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:63f32,per_level:4f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:3.75f32},movespeed:330f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:350f32,aram_damage_taken:1.05f32,aram_damage_dealt:1f32,urf_damage_taken:0.85f32,urf_damage_dealt:1.15f32,},
                    merge_data: &[],
                };pub static VARUS: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Bottom],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_2Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Minion1), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Minion2), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_2), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_3), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_4), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_5), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 53.33f32 + 0.8667f32 * ctx.bonus_ad,2 => 100f32 + 0.9333f32 * ctx.bonus_ad,3 => 146.67f32 + ctx.bonus_ad,4 => 193.33f32 + 1.0667f32 * ctx.bonus_ad,5 => 240f32 + 1.1333f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 80f32 + 1.3f32 * ctx.bonus_ad,2 => 150f32 + 1.4f32 * ctx.bonus_ad,3 => 220f32 + 1.5f32 * ctx.bonus_ad,4 => 290f32 + 1.6f32 * ctx.bonus_ad,5 => 360f32 + 1.7f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 17.6f32 + 0.28600000000000003f32 * ctx.bonus_ad,2 => 33f32 + 0.308f32 * ctx.bonus_ad,3 => 48.4f32 + 0.33f32 * ctx.bonus_ad,4 => 63.8f32 + 0.35200000000000004f32 * ctx.bonus_ad,5 => 79.2f32 + 0.374f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 26.4f32 + 0.429f32 * ctx.bonus_ad,2 => 49.5f32 + 0.462f32 * ctx.bonus_ad,3 => 72.6f32 + 0.495f32 * ctx.bonus_ad,4 => 95.7f32 + 0.528f32 * ctx.bonus_ad,5 => 118.8f32 + 0.561f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 0.03f32 * ctx.enemy_max_health + 0.015f32 * 0.01f32 * ctx.ap,2 => 0.035f32 * ctx.enemy_max_health + 0.015f32 * 0.01f32 * ctx.ap,3 => 0.04f32 * ctx.enemy_max_health + 0.015f32 * 0.01f32 * ctx.ap,4 => 0.045f32 * ctx.enemy_max_health + 0.015f32 * 0.01f32 * ctx.ap,5 => 0.05f32 * ctx.enemy_max_health + 0.015f32 * 0.01f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 0.09f32 * ctx.enemy_max_health + 0.045f32 * 0.01f32 * ctx.ap,2 => 0.105f32 * ctx.enemy_max_health + 0.045f32 * 0.01f32 * ctx.ap,3 => 0.12f32 * ctx.enemy_max_health + 0.045f32 * 0.01f32 * ctx.ap,4 => 0.135f32 * ctx.enemy_max_health + 0.045f32 * 0.01f32 * ctx.ap,5 => 0.15f32 * ctx.enemy_max_health + 0.045f32 * 0.01f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 0.045f32 * ctx.enemy_max_health + 0.0225f32 * 0.01f32 * ctx.ap,2 => 0.0525f32 * ctx.enemy_max_health + 0.0225f32 * 0.01f32 * ctx.ap,3 => 0.06f32 * ctx.enemy_max_health + 0.0225f32 * 0.01f32 * ctx.ap,4 => 0.0675f32 * ctx.enemy_max_health + 0.0225f32 * 0.01f32 * ctx.ap,5 => 0.075f32 * ctx.enemy_max_health + 0.0225f32 * 0.01f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 0.135f32 * ctx.enemy_max_health + 0.0675f32 * 0.01f32 * ctx.ap,2 => 0.1575f32 * ctx.enemy_max_health + 0.0675f32 * 0.01f32 * ctx.ap,3 => 0.18f32 * ctx.enemy_max_health + 0.0675f32 * 0.01f32 * ctx.ap,4 => 0.2025f32 * ctx.enemy_max_health + 0.0675f32 * 0.01f32 * ctx.ap,5 => 0.225f32 * ctx.enemy_max_health + 0.0675f32 * 0.01f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 6f32 + 0.35f32 * ctx.ap,2 => 14f32 + 0.35f32 * ctx.ap,3 => 22f32 + 0.35f32 * ctx.ap,4 => 30f32 + 0.35f32 * ctx.ap,5 => 38f32 + 0.35f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 60f32 + 0.9f32 * ctx.bonus_ad,2 => 90f32 + 0.9f32 * ctx.bonus_ad,3 => 120f32 + 0.9f32 * ctx.bonus_ad,4 => 150f32 + 0.9f32 * ctx.bonus_ad,5 => 180f32 + 0.9f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 150f32 + ctx.ap,2 => 250f32 + ctx.ap,3 => 350f32 + ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:600f32,per_level:105f32},mana:CachedChampionStatsMap{flat:320f32,per_level:40f32},armor:CachedChampionStatsMap{flat:24f32,per_level:4.6f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:59f32,per_level:3.4f32},attack_speed:CachedChampionStatsMap{flat:0.658f32,per_level:3.5f32},movespeed:330f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.657999992370605f32,attack_range:575f32,aram_damage_taken:1f32,aram_damage_dealt:1f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static VAYNE: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Bottom,Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1), 
                damage_type: DamageType::True, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_2), 
                damage_type: DamageType::True, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_2), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_3Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1), 
                damage_type: DamageType::Unknown, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 0.75f32 * ctx.ad + 0.5f32 * ctx.ap,2 => 0.85f32 * ctx.ad + 0.5f32 * ctx.ap,3 => 0.95f32 * ctx.ad + 0.5f32 * ctx.ap,4 => 1.05f32 * ctx.ad + 0.5f32 * ctx.ap,5 => 1.15f32 * ctx.ad + 0.5f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 0.06f32 * ctx.enemy_max_health,2 => 0.07f32 * ctx.enemy_max_health,3 => 0.08f32 * ctx.enemy_max_health,4 => 0.09f32 * ctx.enemy_max_health,5 => 0.1f32 * ctx.enemy_max_health,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 50f32,2 => 65f32,3 => 80f32,4 => 95f32,5 => 110f32,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 75f32 + 0.75f32 * ctx.bonus_ad,2 => 127.5f32 + 0.75f32 * ctx.bonus_ad,3 => 180f32 + 0.75f32 * ctx.bonus_ad,4 => 232.5f32 + 0.75f32 * ctx.bonus_ad,5 => 285f32 + 0.75f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 125f32 + 1.25f32 * ctx.bonus_ad,2 => 212.5f32 + 1.25f32 * ctx.bonus_ad,3 => 300f32 + 1.25f32 * ctx.bonus_ad,4 => 387.5f32 + 1.25f32 * ctx.bonus_ad,5 => 475f32 + 1.25f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 50f32 + 0.5f32 * ctx.bonus_ad,2 => 85f32 + 0.5f32 * ctx.bonus_ad,3 => 120f32 + 0.5f32 * ctx.bonus_ad,4 => 155f32 + 0.5f32 * ctx.bonus_ad,5 => 190f32 + 0.5f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 0.0f32,2 => 0.0f32,3 => 0.0f32,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:550f32,per_level:103f32},mana:CachedChampionStatsMap{flat:232f32,per_level:35f32},armor:CachedChampionStatsMap{flat:23f32,per_level:4.6f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:60f32,per_level:2.35f32},attack_speed:CachedChampionStatsMap{flat:0.658f32,per_level:3.3f32},movespeed:330f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.657999992370605f32,attack_range:550f32,aram_damage_taken:0.95f32,aram_damage_dealt:1f32,urf_damage_taken:1.05f32,urf_damage_dealt:0.95f32,},
                    merge_data: &[],
                };pub static VEIGAR: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Middle,Position::Support],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::Minion1), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 80f32 + 0.5f32 * ctx.ap,2 => 120f32 + 0.55f32 * ctx.ap,3 => 160f32 + 0.6f32 * ctx.ap,4 => 200f32 + 0.65f32 * ctx.ap,5 => 240f32 + 0.7f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 85f32 + 0.7f32 * ctx.ap,2 => 140f32 + 0.8f32 * ctx.ap,3 => 195f32 + 0.9f32 * ctx.ap,4 => 250f32 + ctx.ap,5 => 305f32 + 1.1f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 175f32 + 0.65f32 * ctx.ap,2 => 250f32 + 0.7f32 * ctx.ap,3 => 325f32 + 0.75f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 350f32 + 1.3f32 * ctx.ap,2 => 500f32 + 1.4f32 * ctx.ap,3 => 650f32 + 1.5f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:580f32,per_level:108f32},mana:CachedChampionStatsMap{flat:490f32,per_level:26f32},armor:CachedChampionStatsMap{flat:18f32,per_level:5.2f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:52f32,per_level:2.7f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:2.24f32},movespeed:340f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:550f32,aram_damage_taken:1.1f32,aram_damage_dealt:0.93f32,urf_damage_taken:1f32,urf_damage_dealt:0.9f32,},
                    merge_data: &[],
                };pub static VELKOZ: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Middle,Position::Support],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_3Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_2Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 80f32 + 0.9f32 * ctx.ap,2 => 120f32 + 0.9f32 * ctx.ap,3 => 160f32 + 0.9f32 * ctx.ap,4 => 200f32 + 0.9f32 * ctx.ap,5 => 240f32 + 0.9f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 75f32 + 0.45f32 * ctx.ap,2 => 125f32 + 0.45f32 * ctx.ap,3 => 175f32 + 0.45f32 * ctx.ap,4 => 225f32 + 0.45f32 * ctx.ap,5 => 275f32 + 0.45f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 30f32 + 0.2f32 * ctx.ap,2 => 50f32 + 0.2f32 * ctx.ap,3 => 70f32 + 0.2f32 * ctx.ap,4 => 90f32 + 0.2f32 * ctx.ap,5 => 110f32 + 0.2f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 45f32 + 0.25f32 * ctx.ap,2 => 75f32 + 0.25f32 * ctx.ap,3 => 105f32 + 0.25f32 * ctx.ap,4 => 135f32 + 0.25f32 * ctx.ap,5 => 165f32 + 0.25f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 70f32 + 0.3f32 * ctx.ap,2 => 100f32 + 0.3f32 * ctx.ap,3 => 130f32 + 0.3f32 * ctx.ap,4 => 160f32 + 0.3f32 * ctx.ap,5 => 190f32 + 0.3f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:590f32,per_level:102f32},mana:CachedChampionStatsMap{flat:469f32,per_level:21f32},armor:CachedChampionStatsMap{flat:22f32,per_level:4.7f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:55f32,per_level:3.1416f32},attack_speed:CachedChampionStatsMap{flat:0.643f32,per_level:1.59f32},movespeed:340f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:525f32,aram_damage_taken:1.05f32,aram_damage_dealt:0.95f32,urf_damage_taken:1f32,urf_damage_dealt:0.95f32,},
                    merge_data: &[],
                };pub static VEX: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Middle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::Minion), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 70f32 + 0.7f32 * ctx.ap,2 => 115f32 + 0.7f32 * ctx.ap,3 => 160f32 + 0.7f32 * ctx.ap,4 => 205f32 + 0.7f32 * ctx.ap,5 => 250f32 + 0.7f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 50f32 + 0.75f32 * ctx.ap,2 => 75f32 + 0.75f32 * ctx.ap,3 => 100f32 + 0.75f32 * ctx.ap,4 => 125f32 + 0.75f32 * ctx.ap,5 => 150f32 + 0.75f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 50f32 + 0.4f32 * ctx.ap,2 => 70f32 + 0.45f32 * ctx.ap,3 => 90f32 + 0.5f32 * ctx.ap,4 => 110f32 + 0.55f32 * ctx.ap,5 => 130f32 + 0.6f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 225f32 + 0.7f32 * ctx.ap,2 => 375f32 + 0.7f32 * ctx.ap,3 => 525f32 + 0.7f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 150f32 + 0.5f32 * ctx.ap,2 => 250f32 + 0.5f32 * ctx.ap,3 => 350f32 + 0.5f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 75f32 + 0.2f32 * ctx.ap,2 => 125f32 + 0.2f32 * ctx.ap,3 => 175f32 + 0.2f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:590f32,per_level:104f32},mana:CachedChampionStatsMap{flat:490f32,per_level:32f32},armor:CachedChampionStatsMap{flat:23f32,per_level:4.45f32},magic_resist:CachedChampionStatsMap{flat:28f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:54f32,per_level:2.75f32},attack_speed:CachedChampionStatsMap{flat:0.669f32,per_level:1f32},movespeed:335f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:550f32,aram_damage_taken:1f32,aram_damage_dealt:0.95f32,urf_damage_taken:1f32,urf_damage_dealt:0.9f32,},
                    merge_data: &[],
                };pub static VI: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Jungle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.w_level {1 => 0.3f32,2 => 0.35f32,3 => 0.4f32,4 => 0.45f32,5 => 0.5f32,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 10f32 + 1.1f32 * ctx.ad + ctx.ap,2 => 30f32 + 1.1f32 * ctx.ad + ctx.ap,3 => 50f32 + 1.1f32 * ctx.ad + ctx.ap,4 => 70f32 + 1.1f32 * ctx.ad + ctx.ap,5 => 90f32 + 1.1f32 * ctx.ad + ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 150f32 + 0.9f32 * ctx.bonus_ad,2 => 250f32 + 0.9f32 * ctx.bonus_ad,3 => 350f32 + 0.9f32 * ctx.bonus_ad,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:655f32,per_level:99f32},mana:CachedChampionStatsMap{flat:295f32,per_level:65f32},armor:CachedChampionStatsMap{flat:30f32,per_level:4.7f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:63f32,per_level:3f32},attack_speed:CachedChampionStatsMap{flat:0.644f32,per_level:2f32},movespeed:340f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.643999993801116f32,attack_range:125f32,aram_damage_taken:0.95f32,aram_damage_dealt:1.05f32,urf_damage_taken:1.05f32,urf_damage_dealt:0.95f32,},
                    merge_data: &[],
                };pub static VIEGO: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Jungle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_2), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_3), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Monster1), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 25f32 + 0.7f32 * ctx.ad,2 => 40f32 + 0.7f32 * ctx.ad,3 => 55f32 + 0.7f32 * ctx.ad,4 => 70f32 + 0.7f32 * ctx.ad,5 => 85f32 + 0.7f32 * ctx.ad,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 43.75f32 + 1.225f32 * ctx.ad,2 => 70f32 + 1.225f32 * ctx.ad,3 => 96.25f32 + 1.225f32 * ctx.ad,4 => 122.5f32 + 1.225f32 * ctx.ad,5 => 148.75f32 + 1.225f32 * ctx.ad,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 10f32,2 => 15f32,3 => 20f32,4 => 25f32,5 => 30f32,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 0.02f32 * ctx.enemy_current_health,2 => 0.03f32 * ctx.enemy_current_health,3 => 0.04f32 * ctx.enemy_current_health,4 => 0.05f32 * ctx.enemy_current_health,5 => 0.06f32 * ctx.enemy_current_health,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 0.12f32 * ctx.enemy_missing_health + 0.05f32 * 0.01f32 * ctx.bonus_ad,2 => 0.16f32 * ctx.enemy_missing_health + 0.05f32 * 0.01f32 * ctx.bonus_ad,3 => 0.2f32 * ctx.enemy_missing_health + 0.05f32 * 0.01f32 * ctx.bonus_ad,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:630f32,per_level:109f32},mana:CachedChampionStatsMap{flat:0f32,per_level:0f32},armor:CachedChampionStatsMap{flat:34f32,per_level:4.6f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:57f32,per_level:3.5f32},attack_speed:CachedChampionStatsMap{flat:0.658f32,per_level:2.25f32},movespeed:345f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.658f32,attack_range:200f32,aram_damage_taken:1f32,aram_damage_dealt:1.05f32,urf_damage_taken:0.9f32,urf_damage_dealt:1.05f32,},
                    merge_data: &[],
                };pub static VIKTOR: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Middle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_3Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_2Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 60f32 + 0.4f32 * ctx.ap,2 => 75f32 + 0.4f32 * ctx.ap,3 => 90f32 + 0.4f32 * ctx.ap,4 => 105f32 + 0.4f32 * ctx.ap,5 => 120f32 + 0.4f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 90f32 + 1.3f32 * ctx.ap,2 => 160f32 + 1.3f32 * ctx.ap,3 => 230f32 + 1.3f32 * ctx.ap,4 => 300f32 + 1.3f32 * ctx.ap,5 => 370f32 + 1.3f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 70f32 + 0.5f32 * ctx.ap,2 => 110f32 + 0.5f32 * ctx.ap,3 => 150f32 + 0.5f32 * ctx.ap,4 => 190f32 + 0.5f32 * ctx.ap,5 => 230f32 + 0.5f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 20f32 + 0.8f32 * ctx.ap,2 => 50f32 + 0.8f32 * ctx.ap,3 => 80f32 + 0.8f32 * ctx.ap,4 => 110f32 + 0.8f32 * ctx.ap,5 => 140f32 + 0.8f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 100f32 + 0.5f32 * ctx.ap,2 => 175f32 + 0.5f32 * ctx.ap,3 => 250f32 + 0.5f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:600f32,per_level:100f32},mana:CachedChampionStatsMap{flat:405f32,per_level:45f32},armor:CachedChampionStatsMap{flat:23f32,per_level:4.4f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:53f32,per_level:3f32},attack_speed:CachedChampionStatsMap{flat:0.658f32,per_level:2.11f32},movespeed:335f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.657999992370605f32,attack_range:525f32,aram_damage_taken:1.05f32,aram_damage_dealt:0.95f32,urf_damage_taken:1f32,urf_damage_dealt:0.95f32,},
                    merge_data: &[],
                };pub static VLADIMIR: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Middle,Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_2Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_2Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 148f32 + 1.11f32 * ctx.ap,2 => 185f32 + 1.11f32 * ctx.ap,3 => 222f32 + 1.11f32 * ctx.ap,4 => 259f32 + 1.11f32 * ctx.ap,5 => 296f32 + 1.11f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 20f32 + 0.35f32 * ctx.ap,2 => 25f32 + 0.35f32 * ctx.ap,3 => 30f32 + 0.35f32 * ctx.ap,4 => 35f32 + 0.35f32 * ctx.ap,5 => 40f32 + 0.35f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 80f32 + 0.15f32 * ctx.bonus_health,2 => 135f32 + 0.15f32 * ctx.bonus_health,3 => 190f32 + 0.15f32 * ctx.bonus_health,4 => 245f32 + 0.15f32 * ctx.bonus_health,5 => 300f32 + 0.15f32 * ctx.bonus_health,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 20f32 + 0.0375f32 * ctx.bonus_health,2 => 33.75f32 + 0.0375f32 * ctx.bonus_health,3 => 47.5f32 + 0.0375f32 * ctx.bonus_health,4 => 61.25f32 + 0.0375f32 * ctx.bonus_health,5 => 75f32 + 0.0375f32 * ctx.bonus_health,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 150f32 + 0.7f32 * ctx.ap,2 => 250f32 + 0.7f32 * ctx.ap,3 => 350f32 + 0.7f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:607f32,per_level:110f32},mana:CachedChampionStatsMap{flat:2f32,per_level:0f32},armor:CachedChampionStatsMap{flat:27f32,per_level:4.5f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:55f32,per_level:3f32},attack_speed:CachedChampionStatsMap{flat:0.658f32,per_level:2f32},movespeed:330f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.657999992370605f32,attack_range:450f32,aram_damage_taken:1.05f32,aram_damage_dealt:1f32,urf_damage_taken:1.05f32,urf_damage_dealt:0.92f32,},
                    merge_data: &[],
                };pub static VOLIBEAR: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Jungle,Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 10f32 + 1.2f32 * ctx.bonus_ad,2 => 30f32 + 1.2f32 * ctx.bonus_ad,3 => 50f32 + 1.2f32 * ctx.bonus_ad,4 => 70f32 + 1.2f32 * ctx.bonus_ad,5 => 90f32 + 1.2f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 5f32 + ctx.ad + 0.06f32 * ctx.bonus_health,2 => 30f32 + ctx.ad + 0.06f32 * ctx.bonus_health,3 => 55f32 + ctx.ad + 0.06f32 * ctx.bonus_health,4 => 80f32 + ctx.ad + 0.06f32 * ctx.bonus_health,5 => 105f32 + ctx.ad + 0.06f32 * ctx.bonus_health,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 80f32 + 0.7f32 * ctx.ap + 0.11f32 * ctx.enemy_max_health,2 => 110f32 + 0.7f32 * ctx.ap + 0.12f32 * ctx.enemy_max_health,3 => 140f32 + 0.7f32 * ctx.ap + 0.13f32 * ctx.enemy_max_health,4 => 170f32 + 0.7f32 * ctx.ap + 0.14f32 * ctx.enemy_max_health,5 => 200f32 + 0.7f32 * ctx.ap + 0.15f32 * ctx.enemy_max_health,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:650f32,per_level:104f32},mana:CachedChampionStatsMap{flat:350f32,per_level:70f32},armor:CachedChampionStatsMap{flat:31f32,per_level:5.2f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:60f32,per_level:3.5f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:2f32},movespeed:340f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.7f32,attack_range:150f32,aram_damage_taken:1f32,aram_damage_dealt:1f32,urf_damage_taken:1.1f32,urf_damage_dealt:0.95f32,},
                    merge_data: &[],
                };pub static WARWICK: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Jungle,Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Monster), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::Void), 
                damage_type: DamageType::Unknown, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 150f32 + 1.2f32 * ctx.ad + ctx.ap,2 => 165f32 + 1.2f32 * ctx.ad + ctx.ap,3 => 180f32 + 1.2f32 * ctx.ad + ctx.ap,4 => 195f32 + 1.2f32 * ctx.ad + ctx.ap,5 => 210f32 + 1.2f32 * ctx.ad + ctx.ap,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 0.25f32,2 => 0.375f32,3 => 0.5f32,4 => 0.625f32,5 => 0.75f32,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 0.35f32,2 => 0.4f32,3 => 0.45f32,4 => 0.5f32,5 => 0.55f32,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 175f32 + 1.67f32 * ctx.bonus_ad,2 => 350f32 + 1.67f32 * ctx.bonus_ad,3 => 525f32 + 1.67f32 * ctx.bonus_ad,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:620f32,per_level:99f32},mana:CachedChampionStatsMap{flat:280f32,per_level:35f32},armor:CachedChampionStatsMap{flat:33f32,per_level:4.4f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:65f32,per_level:2.5f32},attack_speed:CachedChampionStatsMap{flat:0.638f32,per_level:2f32},movespeed:335f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.638000011444091f32,attack_range:125f32,aram_damage_taken:0.95f32,aram_damage_dealt:1.05f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static XAYAH: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Bottom],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_2Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_4Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_3Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 90f32 + ctx.bonus_ad,2 => 120f32 + ctx.bonus_ad,3 => 150f32 + ctx.bonus_ad,4 => 180f32 + ctx.bonus_ad,5 => 210f32 + ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 45f32 + 0.5f32 * ctx.bonus_ad,2 => 60f32 + 0.5f32 * ctx.bonus_ad,3 => 75f32 + 0.5f32 * ctx.bonus_ad,4 => 90f32 + 0.5f32 * ctx.bonus_ad,5 => 105f32 + 0.5f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 45f32 + 0.5f32 * ctx.bonus_ad,2 => 60f32 + 0.5f32 * ctx.bonus_ad,3 => 75f32 + 0.5f32 * ctx.bonus_ad,4 => 90f32 + 0.5f32 * ctx.bonus_ad,5 => 105f32 + 0.5f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 22.5f32 + 0.25f32 * ctx.bonus_ad,2 => 30f32 + 0.25f32 * ctx.bonus_ad,3 => 37.5f32 + 0.25f32 * ctx.bonus_ad,4 => 45f32 + 0.25f32 * ctx.bonus_ad,5 => 52.5f32 + 0.25f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 200f32 + ctx.bonus_ad,2 => 300f32 + ctx.bonus_ad,3 => 400f32 + ctx.bonus_ad,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:630f32,per_level:107f32},mana:CachedChampionStatsMap{flat:340f32,per_level:40f32},armor:CachedChampionStatsMap{flat:25f32,per_level:4.2f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:60f32,per_level:3.5f32},attack_speed:CachedChampionStatsMap{flat:0.658f32,per_level:3.9f32},movespeed:330f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.658f32,attack_range:525f32,aram_damage_taken:1f32,aram_damage_dealt:1.05f32,urf_damage_taken:1.1f32,urf_damage_dealt:0.85f32,},
                    merge_data: &[],
                };pub static XERATH: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Middle,Position::Support],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_2Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_3Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_2Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 75f32 + 0.9f32 * ctx.ap,2 => 115f32 + 0.9f32 * ctx.ap,3 => 155f32 + 0.9f32 * ctx.ap,4 => 195f32 + 0.9f32 * ctx.ap,5 => 235f32 + 0.9f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 83.35f32 + 1.08355f32 * ctx.ap,2 => 141.695f32 + 1.08355f32 * ctx.ap,3 => 200.04f32 + 1.08355f32 * ctx.ap,4 => 258.385f32 + 1.08355f32 * ctx.ap,5 => 316.73f32 + 1.08355f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 50f32 + 0.65f32 * ctx.ap,2 => 85f32 + 0.65f32 * ctx.ap,3 => 120f32 + 0.65f32 * ctx.ap,4 => 155f32 + 0.65f32 * ctx.ap,5 => 190f32 + 0.65f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 70f32 + 0.45f32 * ctx.ap,2 => 100f32 + 0.45f32 * ctx.ap,3 => 130f32 + 0.45f32 * ctx.ap,4 => 160f32 + 0.45f32 * ctx.ap,5 => 190f32 + 0.45f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 170f32 + 0.45f32 * ctx.ap,2 => 220f32 + 0.45f32 * ctx.ap,3 => 270f32 + 0.45f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 20f32 + 0.05f32 * ctx.ap,2 => 25f32 + 0.05f32 * ctx.ap,3 => 30f32 + 0.05f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 3f32,2 => 4f32,3 => 5f32,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:596f32,per_level:106f32},mana:CachedChampionStatsMap{flat:400f32,per_level:22f32},armor:CachedChampionStatsMap{flat:22f32,per_level:4.7f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:55f32,per_level:3f32},attack_speed:CachedChampionStatsMap{flat:0.658f32,per_level:1.36f32},movespeed:340f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:525f32,aram_damage_taken:1.05f32,aram_damage_dealt:0.93f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static XINZHAO: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Jungle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_2), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_3Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_2Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 15f32 + 0.4f32 * ctx.bonus_ad,2 => 30f32 + 0.4f32 * ctx.bonus_ad,3 => 45f32 + 0.4f32 * ctx.bonus_ad,4 => 60f32 + 0.4f32 * ctx.bonus_ad,5 => 75f32 + 0.4f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 45f32 + 1.2f32 * ctx.bonus_ad,2 => 90f32 + 1.2f32 * ctx.bonus_ad,3 => 135f32 + 1.2f32 * ctx.bonus_ad,4 => 180f32 + 1.2f32 * ctx.bonus_ad,5 => 225f32 + 1.2f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 50f32 + 0.9f32 * ctx.ad + 0.65f32 * ctx.ap,2 => 85f32 + 0.9f32 * ctx.ad + 0.65f32 * ctx.ap,3 => 120f32 + 0.9f32 * ctx.ad + 0.65f32 * ctx.ap,4 => 155f32 + 0.9f32 * ctx.ad + 0.65f32 * ctx.ap,5 => 190f32 + 0.9f32 * ctx.ad + 0.65f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 7.5f32 + 0.075f32 * ctx.ad,2 => 10f32 + 0.075f32 * ctx.ad,3 => 12.5f32 + 0.075f32 * ctx.ad,4 => 15f32 + 0.075f32 * ctx.ad,5 => 17.5f32 + 0.075f32 * ctx.ad,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 30f32 + 0.3f32 * ctx.ad,2 => 40f32 + 0.3f32 * ctx.ad,3 => 50f32 + 0.3f32 * ctx.ad,4 => 60f32 + 0.3f32 * ctx.ad,5 => 70f32 + 0.3f32 * ctx.ad,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 50f32 + 0.6f32 * ctx.ap,2 => 75f32 + 0.6f32 * ctx.ap,3 => 100f32 + 0.6f32 * ctx.ap,4 => 125f32 + 0.6f32 * ctx.ap,5 => 150f32 + 0.6f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:620f32,per_level:106f32},mana:CachedChampionStatsMap{flat:274f32,per_level:55f32},armor:CachedChampionStatsMap{flat:35f32,per_level:4.4f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:63f32,per_level:3f32},attack_speed:CachedChampionStatsMap{flat:0.645f32,per_level:3.5f32},movespeed:345f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.644999980926513f32,attack_range:175f32,aram_damage_taken:1f32,aram_damage_dealt:0.95f32,urf_damage_taken:1.1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static YASUO: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Bottom,Position::Middle,Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 20f32 + 1.05f32 * ctx.ad,2 => 45f32 + 1.05f32 * ctx.ad,3 => 70f32 + 1.05f32 * ctx.ad,4 => 95f32 + 1.05f32 * ctx.ad,5 => 120f32 + 1.05f32 * ctx.ad,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 70f32 + 0.2f32 * ctx.bonus_ad + 0.6f32 * ctx.ap,2 => 85f32 + 0.2f32 * ctx.bonus_ad + 0.6f32 * ctx.ap,3 => 100f32 + 0.2f32 * ctx.bonus_ad + 0.6f32 * ctx.ap,4 => 115f32 + 0.2f32 * ctx.bonus_ad + 0.6f32 * ctx.ap,5 => 130f32 + 0.2f32 * ctx.bonus_ad + 0.6f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:590f32,per_level:110f32},mana:CachedChampionStatsMap{flat:100f32,per_level:0f32},armor:CachedChampionStatsMap{flat:32f32,per_level:4.6f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:60f32,per_level:2.5f32},attack_speed:CachedChampionStatsMap{flat:0.697f32,per_level:3.5f32},movespeed:345f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.6700000166893f32,attack_range:175f32,aram_damage_taken:1f32,aram_damage_dealt:1f32,urf_damage_taken:0.9f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static YONE: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Middle,Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_3Max), 
                damage_type: DamageType::Unknown, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Unknown, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_2Min), 
                damage_type: DamageType::Unknown, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_3Max), 
                damage_type: DamageType::Unknown, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Unknown, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_2Min), 
                damage_type: DamageType::Unknown, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 20f32 + 1.05f32 * ctx.ad,2 => 45f32 + 1.05f32 * ctx.ad,3 => 70f32 + 1.05f32 * ctx.ad,4 => 95f32 + 1.05f32 * ctx.ad,5 => 120f32 + 1.05f32 * ctx.ad,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 5f32 + 0.04f32 * ctx.enemy_max_health,2 => 10f32 + 0.045f32 * ctx.enemy_max_health,3 => 15f32 + 0.05f32 * ctx.enemy_max_health,4 => 20f32 + 0.055f32 * ctx.enemy_max_health,5 => 25f32 + 0.06f32 * ctx.enemy_max_health,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 10f32 + 0.08f32 * ctx.enemy_max_health,2 => 20f32 + 0.09f32 * ctx.enemy_max_health,3 => 30f32 + 0.1f32 * ctx.enemy_max_health,4 => 40f32 + 0.11f32 * ctx.enemy_max_health,5 => 50f32 + 0.12f32 * ctx.enemy_max_health,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 5f32 + 0.04f32 * ctx.enemy_max_health,2 => 10f32 + 0.045f32 * ctx.enemy_max_health,3 => 15f32 + 0.05f32 * ctx.enemy_max_health,4 => 20f32 + 0.055f32 * ctx.enemy_max_health,5 => 25f32 + 0.06f32 * ctx.enemy_max_health,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 100f32 + 0.4f32 * ctx.bonus_ad,2 => 200f32 + 0.4f32 * ctx.bonus_ad,3 => 300f32 + 0.4f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 200f32 + 0.8f32 * ctx.bonus_ad,2 => 400f32 + 0.8f32 * ctx.bonus_ad,3 => 600f32 + 0.8f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 100f32 + 0.4f32 * ctx.bonus_ad,2 => 200f32 + 0.4f32 * ctx.bonus_ad,3 => 300f32 + 0.4f32 * ctx.bonus_ad,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:620f32,per_level:105f32},mana:CachedChampionStatsMap{flat:500f32,per_level:0f32},armor:CachedChampionStatsMap{flat:33f32,per_level:4.6f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:60f32,per_level:2f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:3.5f32},movespeed:345f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:175f32,aram_damage_taken:0.97f32,aram_damage_dealt:1.05f32,urf_damage_taken:0.9f32,urf_damage_dealt:1.1f32,},
                    merge_data: &[],
                };pub static YORICK: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::Minion1), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::Monster1), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 30f32 + 0.5f32 * ctx.ad,2 => 50f32 + 0.5f32 * ctx.ad,3 => 70f32 + 0.5f32 * ctx.ad,4 => 90f32 + 0.5f32 * ctx.ad,5 => 110f32 + 0.5f32 * ctx.ad,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 70f32 + ctx.ap,2 => 105f32 + ctx.ap,3 => 140f32 + ctx.ap,4 => 175f32 + ctx.ap,5 => 210f32 + ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 50f32 + ctx.ap,2 => 75f32 + ctx.ap,3 => 100f32 + ctx.ap,4 => 125f32 + ctx.ap,5 => 150f32 + ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 0.06f32 * ctx.enemy_max_health + 0.03f32 * 0.01f32 * ctx.ap,2 => 0.065f32 * ctx.enemy_max_health + 0.03f32 * 0.01f32 * ctx.ap,3 => 0.07f32 * ctx.enemy_max_health + 0.03f32 * 0.01f32 * ctx.ap,4 => 0.075f32 * ctx.enemy_max_health + 0.03f32 * 0.01f32 * ctx.ap,5 => 0.08f32 * ctx.enemy_max_health + 0.03f32 * 0.01f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:650f32,per_level:114f32},mana:CachedChampionStatsMap{flat:300f32,per_level:60f32},armor:CachedChampionStatsMap{flat:36f32,per_level:4.5f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:62f32,per_level:5f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:2f32},movespeed:340f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:175f32,aram_damage_taken:1f32,aram_damage_dealt:1f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static YUNARA: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Bottom],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1), 
                damage_type: DamageType::Unknown, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_3Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_2Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 5f32 + 0.2f32 * ctx.ap,2 => 10f32 + 0.2f32 * ctx.ap,3 => 15f32 + 0.2f32 * ctx.ap,4 => 20f32 + 0.2f32 * ctx.ap,5 => 25f32 + 0.2f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 5f32 + 0.4f32 * ctx.ad + 0.25f32 * ctx.ap,2 => 20f32 + 0.4f32 * ctx.ad + 0.25f32 * ctx.ap,3 => 35f32 + 0.4f32 * ctx.ad + 0.25f32 * ctx.ap,4 => 50f32 + 0.4f32 * ctx.ad + 0.25f32 * ctx.ap,5 => 65f32 + 0.4f32 * ctx.ad + 0.25f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 5f32 + 0.85f32 * ctx.ad + 0.5f32 * ctx.ap,2 => 30f32 + 0.85f32 * ctx.ad + 0.5f32 * ctx.ap,3 => 55f32 + 0.85f32 * ctx.ad + 0.5f32 * ctx.ap,4 => 80f32 + 0.85f32 * ctx.ad + 0.5f32 * ctx.ap,5 => 105f32 + 0.85f32 * ctx.ad + 0.5f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 1.25f32 + 0.1f32 * ctx.ad + 0.0625f32 * ctx.ap,2 => 5f32 + 0.1f32 * ctx.ad + 0.0625f32 * ctx.ap,3 => 8.75f32 + 0.1f32 * ctx.ad + 0.0625f32 * ctx.ap,4 => 12.5f32 + 0.1f32 * ctx.ad + 0.0625f32 * ctx.ap,5 => 16.25f32 + 0.1f32 * ctx.ad + 0.0625f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:590f32,per_level:110f32},mana:CachedChampionStatsMap{flat:275f32,per_level:45f32},armor:CachedChampionStatsMap{flat:25f32,per_level:4.4f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:55f32,per_level:2.5f32},attack_speed:CachedChampionStatsMap{flat:0.65f32,per_level:2f32},movespeed:325f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.65f32,attack_range:575f32,aram_damage_taken:1f32,aram_damage_dealt:1f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static YUUMI: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Support],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_3), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_2Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_2Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 10f32 + 0.05f32 * ctx.ap,2 => 12f32 + 0.05f32 * ctx.ap,3 => 14f32 + 0.05f32 * ctx.ap,4 => 16f32 + 0.05f32 * ctx.ap,5 => 18f32 + 0.05f32 * ctx.ap,6 => 20f32 + 0.05f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 0.5f32,2 => 0.53f32,3 => 0.56f32,4 => 0.59f32,5 => 0.62f32,6 => 0.65f32,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 60f32 + 0.2f32 * ctx.ap,2 => 95f32 + 0.2f32 * ctx.ap,3 => 130f32 + 0.2f32 * ctx.ap,4 => 165f32 + 0.2f32 * ctx.ap,5 => 200f32 + 0.2f32 * ctx.ap,6 => 235f32 + 0.2f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 39f32 + 0.13f32 * ctx.ap,2 => 52f32 + 0.13f32 * ctx.ap,3 => 65f32 + 0.13f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 195f32 + 0.65f32 * ctx.ap,2 => 260f32 + 0.65f32 * ctx.ap,3 => 325f32 + 0.65f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:500f32,per_level:69f32},mana:CachedChampionStatsMap{flat:440f32,per_level:45f32},armor:CachedChampionStatsMap{flat:25f32,per_level:4.2f32},magic_resist:CachedChampionStatsMap{flat:25f32,per_level:1.1f32},attack_damage:CachedChampionStatsMap{flat:49f32,per_level:3.1f32},attack_speed:CachedChampionStatsMap{flat:0.625f32,per_level:1f32},movespeed:330f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:425f32,aram_damage_taken:1f32,aram_damage_dealt:1f32,urf_damage_taken:0.9f32,urf_damage_dealt:0.75f32,},
                    merge_data: &[],
                };pub static ZAC: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Jungle,Position::Support,Position::Top],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_2Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_2Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 80f32 + 0.6f32 * ctx.ap + 0.06f32 * ctx.max_health,2 => 110f32 + 0.6f32 * ctx.ap + 0.06f32 * ctx.max_health,3 => 140f32 + 0.6f32 * ctx.ap + 0.06f32 * ctx.max_health,4 => 170f32 + 0.6f32 * ctx.ap + 0.06f32 * ctx.max_health,5 => 200f32 + 0.6f32 * ctx.ap + 0.06f32 * ctx.max_health,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 40f32 + 0.3f32 * ctx.ap + 0.03f32 * ctx.max_health,2 => 55f32 + 0.3f32 * ctx.ap + 0.03f32 * ctx.max_health,3 => 70f32 + 0.3f32 * ctx.ap + 0.03f32 * ctx.max_health,4 => 85f32 + 0.3f32 * ctx.ap + 0.03f32 * ctx.max_health,5 => 100f32 + 0.3f32 * ctx.ap + 0.03f32 * ctx.max_health,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 40f32 + 0.04f32 * ctx.enemy_max_health + 0.03f32 * 0.01f32 * ctx.ap,2 => 50f32 + 0.05f32 * ctx.enemy_max_health + 0.03f32 * 0.01f32 * ctx.ap,3 => 60f32 + 0.06f32 * ctx.enemy_max_health + 0.03f32 * 0.01f32 * ctx.ap,4 => 70f32 + 0.07f32 * ctx.enemy_max_health + 0.03f32 * 0.01f32 * ctx.ap,5 => 80f32 + 0.08f32 * ctx.enemy_max_health + 0.03f32 * 0.01f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 240f32,2 => 250f32,3 => 260f32,4 => 270f32,5 => 280f32,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 60f32 + 0.8f32 * ctx.ap,2 => 105f32 + 0.8f32 * ctx.ap,3 => 150f32 + 0.8f32 * ctx.ap,4 => 195f32 + 0.8f32 * ctx.ap,5 => 240f32 + 0.8f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:685f32,per_level:109f32},mana:CachedChampionStatsMap{flat:0f32,per_level:0f32},armor:CachedChampionStatsMap{flat:33f32,per_level:4.7f32},magic_resist:CachedChampionStatsMap{flat:32f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:60f32,per_level:3.4f32},attack_speed:CachedChampionStatsMap{flat:0.736f32,per_level:1.6f32},movespeed:340f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.638000011444091f32,attack_range:175f32,aram_damage_taken:0.96f32,aram_damage_dealt:1f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static ZED: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Melee,
                    positions: &[Position::Jungle,Position::Middle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::Void), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::Void), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 80f32 + ctx.bonus_ad,2 => 120f32 + ctx.bonus_ad,3 => 160f32 + ctx.bonus_ad,4 => 200f32 + ctx.bonus_ad,5 => 240f32 + ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 48f32 + 0.6f32 * ctx.bonus_ad,2 => 72f32 + 0.6f32 * ctx.bonus_ad,3 => 96f32 + 0.6f32 * ctx.bonus_ad,4 => 120f32 + 0.6f32 * ctx.bonus_ad,5 => 144f32 + 0.6f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 70f32 + 0.8f32 * ctx.bonus_ad,2 => 95f32 + 0.8f32 * ctx.bonus_ad,3 => 120f32 + 0.8f32 * ctx.bonus_ad,4 => 145f32 + 0.8f32 * ctx.bonus_ad,5 => 170f32 + 0.8f32 * ctx.bonus_ad,_ => 0.0 }},|ctx| { match ctx.r_level {1 => ctx.ad + 0.25f32 * 100.0f32,2 => ctx.ad + 0.4f32 * 100.0f32,3 => ctx.ad + 0.55f32 * 100.0f32,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:654f32,per_level:99f32},mana:CachedChampionStatsMap{flat:200f32,per_level:0f32},armor:CachedChampionStatsMap{flat:32f32,per_level:4.7f32},magic_resist:CachedChampionStatsMap{flat:29f32,per_level:2.05f32},attack_damage:CachedChampionStatsMap{flat:63f32,per_level:3.4f32},attack_speed:CachedChampionStatsMap{flat:0.651f32,per_level:3.3f32},movespeed:345f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.651000022888183f32,attack_range:125f32,aram_damage_taken:0.95f32,aram_damage_dealt:1.05f32,urf_damage_taken:1.1f32,urf_damage_dealt:0.85f32,},
                    merge_data: &[],
                };pub static ZERI: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Physical,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Bottom],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Max), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Min), 
                damage_type: DamageType::Physical, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::Void), 
                damage_type: DamageType::Unknown, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 2.14f32 + 0.14859999999999998f32 * ctx.ad,2 => 2.43f32 + 0.1543f32 * ctx.ad,3 => 2.71f32 + 0.16f32 * ctx.ad,4 => 3f32 + 0.16570000000000001f32 * ctx.ad,5 => 3.29f32 + 0.1714f32 * ctx.ad,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 15f32 + 1.04f32 * ctx.ad,2 => 17f32 + 1.08f32 * ctx.ad,3 => 19f32 + 1.12f32 * ctx.ad,4 => 21f32 + 1.16f32 * ctx.ad,5 => 23f32 + 1.2f32 * ctx.ad,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 30f32 + 1.3f32 * ctx.ad + 0.25f32 * ctx.ap,2 => 70f32 + 1.3f32 * ctx.ad + 0.25f32 * ctx.ap,3 => 110f32 + 1.3f32 * ctx.ad + 0.25f32 * ctx.ap,4 => 150f32 + 1.3f32 * ctx.ad + 0.25f32 * ctx.ap,5 => 190f32 + 1.3f32 * ctx.ad + 0.25f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 0.8f32,2 => 0.85f32,3 => 0.9f32,4 => 0.95f32,5 => 1f32,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 20f32 + 0.12f32 * ctx.bonus_ad + 0.2f32 * ctx.ap,2 => 22f32 + 0.12f32 * ctx.bonus_ad + 0.2f32 * ctx.ap,3 => 24f32 + 0.12f32 * ctx.bonus_ad + 0.2f32 * ctx.ap,4 => 26f32 + 0.12f32 * ctx.bonus_ad + 0.2f32 * ctx.ap,5 => 28f32 + 0.12f32 * ctx.bonus_ad + 0.2f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 200f32 + ctx.bonus_ad + 1.1f32 * ctx.ap,2 => 300f32 + ctx.bonus_ad + 1.1f32 * ctx.ap,3 => 400f32 + ctx.bonus_ad + 1.1f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:600f32,per_level:110f32},mana:CachedChampionStatsMap{flat:250f32,per_level:45f32},armor:CachedChampionStatsMap{flat:24f32,per_level:4.2f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:56f32,per_level:2f32},attack_speed:CachedChampionStatsMap{flat:0.658f32,per_level:2f32},movespeed:330f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:500f32,aram_damage_taken:0.95f32,aram_damage_dealt:1.1f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static ZIGGS: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Bottom,Position::Middle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_2Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_3Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_1Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::_2Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 80f32 + 0.6f32 * ctx.ap,2 => 130f32 + 0.65f32 * ctx.ap,3 => 180f32 + 0.7f32 * ctx.ap,4 => 230f32 + 0.75f32 * ctx.ap,5 => 280f32 + 0.8f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 70f32 + 0.5f32 * ctx.ap,2 => 105f32 + 0.5f32 * ctx.ap,3 => 140f32 + 0.5f32 * ctx.ap,4 => 175f32 + 0.5f32 * ctx.ap,5 => 210f32 + 0.5f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 12f32 + 0.1f32 * ctx.ap,2 => 28f32 + 0.12f32 * ctx.ap,3 => 44f32 + 0.14f32 * ctx.ap,4 => 60f32 + 0.16f32 * ctx.ap,5 => 76f32 + 0.18f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 30f32 + 0.25f32 * ctx.ap,2 => 70f32 + 0.3f32 * ctx.ap,3 => 110f32 + 0.35f32 * ctx.ap,4 => 150f32 + 0.4f32 * ctx.ap,5 => 190f32 + 0.45f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 150f32 + 1.25f32 * ctx.ap,2 => 350f32 + 1.5f32 * ctx.ap,3 => 550f32 + 1.75f32 * ctx.ap,4 => 750f32 + 2f32 * ctx.ap,5 => 950f32 + 2.25f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 300f32 + ctx.ap,2 => 500f32 + ctx.ap,3 => 700f32 + ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 195f32 + 0.65f32 * ctx.ap,2 => 325f32 + 0.65f32 * ctx.ap,3 => 455f32 + 0.65f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:606f32,per_level:106f32},mana:CachedChampionStatsMap{flat:480f32,per_level:23.5f32},armor:CachedChampionStatsMap{flat:21f32,per_level:4.7f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:55f32,per_level:3.1f32},attack_speed:CachedChampionStatsMap{flat:0.656f32,per_level:2f32},movespeed:325f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.656000018119812f32,attack_range:550f32,aram_damage_taken:1.2f32,aram_damage_dealt:0.92f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static ZILEAN: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Support],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 75f32 + 0.9f32 * ctx.ap,2 => 115f32 + 0.9f32 * ctx.ap,3 => 165f32 + 0.9f32 * ctx.ap,4 => 230f32 + 0.9f32 * ctx.ap,5 => 300f32 + 0.9f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:574f32,per_level:96f32},mana:CachedChampionStatsMap{flat:452f32,per_level:50f32},armor:CachedChampionStatsMap{flat:24f32,per_level:5f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:52f32,per_level:3f32},attack_speed:CachedChampionStatsMap{flat:0.658f32,per_level:2.13f32},movespeed:335f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:550f32,aram_damage_taken:0.95f32,aram_damage_dealt:1.05f32,urf_damage_taken:0.9f32,urf_damage_dealt:1.05f32,},
                    merge_data: &[],
                };pub static ZOE: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Middle],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Minion), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::W(AbilityName::Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::_2), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::Max), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::Min), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 2f32 + 50f32 + 0.6f32 * ctx.ap,2 => 4.8235294117647065f32 + 80f32 + 0.6f32 * ctx.ap,3 => 7.647058823529412f32 + 110f32 + 0.6f32 * ctx.ap,4 => 10.470588235294118f32 + 140f32 + 0.6f32 * ctx.ap,5 => 13.294117647058824f32 + 170f32 + 0.6f32 * ctx.ap,6 => 16.11764705882353f32,7 => 18.941176470588236f32,8 => 21.764705882352946f32,9 => 24.58823529411765f32,10 => 27.411764705882355f32,11 => 30.23529411764706f32,12 => 33.05882352941177f32,13 => 35.88235294117647f32,14 => 38.70588235294118f32,15 => 41.52941176470589f32,16 => 44.352941176470594f32,17 => 47.1764705882353f32,18 => 50f32,_ => 0.0 }},|ctx| { match ctx.q_level {1 => 5f32 + 125f32 + 1.5f32 * ctx.ap,2 => 12.058823529411764f32 + 200f32 + 1.5f32 * ctx.ap,3 => 19.11764705882353f32 + 275f32 + 1.5f32 * ctx.ap,4 => 26.176470588235293f32 + 350f32 + 1.5f32 * ctx.ap,5 => 33.23529411764706f32 + 425f32 + 1.5f32 * ctx.ap,6 => 40.294117647058826f32,7 => 47.35294117647059f32,8 => 54.41176470588235f32,9 => 61.47058823529411f32,10 => 68.52941176470588f32,11 => 75.58823529411765f32,12 => 82.6470588235294f32,13 => 89.70588235294117f32,14 => 96.76470588235294f32,15 => 103.8235294117647f32,16 => 110.88235294117646f32,17 => 117.94117647058825f32,18 => 125f32,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 60f32 + 0.45f32 * ctx.ap,2 => 90f32 + 0.45f32 * ctx.ap,3 => 120f32 + 0.45f32 * ctx.ap,4 => 150f32 + 0.45f32 * ctx.ap,5 => 180f32 + 0.45f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.w_level {1 => 20f32 + 0.15f32 * ctx.ap,2 => 30f32 + 0.15f32 * ctx.ap,3 => 40f32 + 0.15f32 * ctx.ap,4 => 50f32 + 0.15f32 * ctx.ap,5 => 60f32 + 0.15f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 70f32 + 0.45f32 * ctx.ap,2 => 110f32 + 0.45f32 * ctx.ap,3 => 150f32 + 0.45f32 * ctx.ap,4 => 190f32 + 0.45f32 * ctx.ap,5 => 230f32 + 0.45f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 140f32 + 0.9f32 * ctx.ap,2 => 220f32 + 0.9f32 * ctx.ap,3 => 300f32 + 0.9f32 * ctx.ap,4 => 380f32 + 0.9f32 * ctx.ap,5 => 460f32 + 0.9f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 70f32 + 0.45f32 * ctx.ap,2 => 110f32 + 0.45f32 * ctx.ap,3 => 150f32 + 0.45f32 * ctx.ap,4 => 190f32 + 0.45f32 * ctx.ap,5 => 230f32 + 0.45f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:630f32,per_level:106f32},mana:CachedChampionStatsMap{flat:425f32,per_level:25f32},armor:CachedChampionStatsMap{flat:21f32,per_level:4.7f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:58f32,per_level:3.3f32},attack_speed:CachedChampionStatsMap{flat:0.658f32,per_level:2.5f32},movespeed:340f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:550f32,aram_damage_taken:0.95f32,aram_damage_dealt:1.1f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static ZYRA: CachedChampion = CachedChampion {
                    adaptative_type: AdaptativeType::Magic,
                    attack_type: AttackType::Ranged,
                    positions: &[Position::Support],
                    metadata: &[TypeMetadata { 
                kind: AbilityLike::Q(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::E(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            },TypeMetadata { 
                kind: AbilityLike::R(AbilityName::Void), 
                damage_type: DamageType::Magic, 
                attributes: Attrs::None 
            }],
                    closures: &[|ctx| { match ctx.q_level {1 => 60f32 + 0.65f32 * ctx.ap,2 => 100f32 + 0.65f32 * ctx.ap,3 => 140f32 + 0.65f32 * ctx.ap,4 => 180f32 + 0.65f32 * ctx.ap,5 => 220f32 + 0.65f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.e_level {1 => 60f32 + 0.6f32 * ctx.ap,2 => 95f32 + 0.6f32 * ctx.ap,3 => 130f32 + 0.6f32 * ctx.ap,4 => 165f32 + 0.6f32 * ctx.ap,5 => 200f32 + 0.6f32 * ctx.ap,_ => 0.0 }},|ctx| { match ctx.r_level {1 => 200f32 + 0.7f32 * ctx.ap,2 => 300f32 + 0.7f32 * ctx.ap,3 => 400f32 + 0.7f32 * ctx.ap,_ => 0.0 }}],
                    stats: CachedChampionStats {health:CachedChampionStatsMap{flat:574f32,per_level:93f32},mana:CachedChampionStatsMap{flat:418f32,per_level:25f32},armor:CachedChampionStatsMap{flat:29f32,per_level:4.2f32},magic_resist:CachedChampionStatsMap{flat:30f32,per_level:1.3f32},attack_damage:CachedChampionStatsMap{flat:53f32,per_level:3.2f32},attack_speed:CachedChampionStatsMap{flat:0.681f32,per_level:2.11f32},movespeed:340f32,critical_strike_damage:175f32,critical_strike_damage_modifier:1f32,attack_speed_ratio:0.625f32,attack_range:575f32,aram_damage_taken:1.05f32,aram_damage_dealt:0.9f32,urf_damage_taken:1f32,urf_damage_dealt:1f32,},
                    merge_data: &[],
                };pub static CHAMPION_NAME_TO_ID:phf::Map<&'static str,ChampionId>=phf::phf_map!{"آتروكس"=>ChampionId::Aatrox,"Aatrox"=>ChampionId::Aatrox,"Άατροξ"=>ChampionId::Aatrox,"エイトロックス"=>ChampionId::Aatrox,"아트록스"=>ChampionId::Aatrox,"Атрокс"=>ChampionId::Aatrox,"暗裔剑魔"=>ChampionId::Aatrox,"厄萨斯"=>ChampionId::Aatrox,"厄薩斯"=>ChampionId::Aatrox,"آري"=>ChampionId::Ahri,"Ahri"=>ChampionId::Ahri,"Άρι"=>ChampionId::Ahri,"アーリ"=>ChampionId::Ahri,"아리"=>ChampionId::Ahri,"Ари"=>ChampionId::Ahri,"九尾妖狐"=>ChampionId::Ahri,"阿璃"=>ChampionId::Ahri,"أكالي"=>ChampionId::Akali,"Akali"=>ChampionId::Akali,"Ακάλι"=>ChampionId::Akali,"アカリ"=>ChampionId::Akali,"아칼리"=>ChampionId::Akali,"Акали"=>ChampionId::Akali,"离群之刺"=>ChampionId::Akali,"阿卡莉"=>ChampionId::Akali,"أكشان"=>ChampionId::Akshan,"Akshan"=>ChampionId::Akshan,"Ακσάν"=>ChampionId::Akshan,"アクシャン"=>ChampionId::Akshan,"아크샨"=>ChampionId::Akshan,"Акшан"=>ChampionId::Akshan,"影哨"=>ChampionId::Akshan,"埃可尚"=>ChampionId::Akshan,"أليستار"=>ChampionId::Alistar,"Alistar"=>ChampionId::Alistar,"Άλισταρ"=>ChampionId::Alistar,"アリスター"=>ChampionId::Alistar,"알리스타"=>ChampionId::Alistar,"Алистар"=>ChampionId::Alistar,"牛头酋长"=>ChampionId::Alistar,"亚历斯塔"=>ChampionId::Alistar,"亞歷斯塔"=>ChampionId::Alistar,"أمبيسا"=>ChampionId::Ambessa,"Ambessa"=>ChampionId::Ambessa,"Αμπέσα"=>ChampionId::Ambessa,"アンベッサ"=>ChampionId::Ambessa,"암베사"=>ChampionId::Ambessa,"Амбесса"=>ChampionId::Ambessa,"铁血狼母"=>ChampionId::Ambessa,"安蓓萨"=>ChampionId::Ambessa,"安比薩"=>ChampionId::Ambessa,"أمومو"=>ChampionId::Amumu,"Amumu"=>ChampionId::Amumu,"Αμούμου"=>ChampionId::Amumu,"アムム"=>ChampionId::Amumu,"아무무"=>ChampionId::Amumu,"Амуму"=>ChampionId::Amumu,"殇之木乃伊"=>ChampionId::Amumu,"阿姆姆"=>ChampionId::Amumu,"أنيفيا"=>ChampionId::Anivia,"Anivia"=>ChampionId::Anivia,"Ανίβια"=>ChampionId::Anivia,"アニビア"=>ChampionId::Anivia,"애니비아"=>ChampionId::Anivia,"Анивия"=>ChampionId::Anivia,"冰晶凤凰"=>ChampionId::Anivia,"艾妮维亚"=>ChampionId::Anivia,"艾妮維亞"=>ChampionId::Anivia,"آني"=>ChampionId::Annie,"Annie"=>ChampionId::Annie,"Άνι"=>ChampionId::Annie,"アニー"=>ChampionId::Annie,"애니"=>ChampionId::Annie,"Энни"=>ChampionId::Annie,"黑暗之女"=>ChampionId::Annie,"安妮"=>ChampionId::Annie,"أفيليوس"=>ChampionId::Aphelios,"Aphelios"=>ChampionId::Aphelios,"Αφέλιος"=>ChampionId::Aphelios,"アフェリオス"=>ChampionId::Aphelios,"아펠리오스"=>ChampionId::Aphelios,"Афелий"=>ChampionId::Aphelios,"残月之肃"=>ChampionId::Aphelios,"亚菲利欧"=>ChampionId::Aphelios,"亞菲利歐"=>ChampionId::Aphelios,"آش"=>ChampionId::Ashe,"Ashe"=>ChampionId::Ashe,"Ας"=>ChampionId::Ashe,"アッシュ"=>ChampionId::Ashe,"애쉬"=>ChampionId::Ashe,"Эш"=>ChampionId::Ashe,"寒冰射手"=>ChampionId::Ashe,"艾希"=>ChampionId::Ashe,"أوريليون سول"=>ChampionId::AurelionSol,"Aurelion Sol"=>ChampionId::AurelionSol,"Ωρέλιον Σολ"=>ChampionId::AurelionSol,"オレリオン・ソル"=>ChampionId::AurelionSol,"아우렐리온 솔"=>ChampionId::AurelionSol,"Аурелион Сол"=>ChampionId::AurelionSol,"铸星龙王"=>ChampionId::AurelionSol,"翱锐龙兽"=>ChampionId::AurelionSol,"翱銳龍獸"=>ChampionId::AurelionSol,"أورورا"=>ChampionId::Aurora,"Aurora"=>ChampionId::Aurora,"Ωρόρα"=>ChampionId::Aurora,"オーロラ"=>ChampionId::Aurora,"오로라"=>ChampionId::Aurora,"Аврора"=>ChampionId::Aurora,"双界灵兔"=>ChampionId::Aurora,"极光"=>ChampionId::Aurora,"歐羅拉"=>ChampionId::Aurora,"أزير"=>ChampionId::Azir,"Azir"=>ChampionId::Azir,"Αζίρ"=>ChampionId::Azir,"アジール"=>ChampionId::Azir,"아지르"=>ChampionId::Azir,"Азир"=>ChampionId::Azir,"沙漠皇帝"=>ChampionId::Azir,"阿祈尔"=>ChampionId::Azir,"阿祈爾"=>ChampionId::Azir,"بارد"=>ChampionId::Bard,"Bard"=>ChampionId::Bard,"Βάρδος"=>ChampionId::Bard,"Bardo"=>ChampionId::Bard,"バード"=>ChampionId::Bard,"바드"=>ChampionId::Bard,"Бард"=>ChampionId::Bard,"星界游神"=>ChampionId::Bard,"巴德"=>ChampionId::Bard,"بيلفيث"=>ChampionId::Belveth,"Bel'Veth"=>ChampionId::Belveth,"Μπελ'Βεθ"=>ChampionId::Belveth,"ベル＝ヴェス"=>ChampionId::Belveth,"벨베스"=>ChampionId::Belveth,"Бел'Вет"=>ChampionId::Belveth,"虚空女皇"=>ChampionId::Belveth,"贝尔薇斯"=>ChampionId::Belveth,"貝爾薇斯"=>ChampionId::Belveth,"بليتزكرانك"=>ChampionId::Blitzcrank,"Blitzcrank"=>ChampionId::Blitzcrank,"Μπλίτζκρανκ"=>ChampionId::Blitzcrank,"ブリッツクランク"=>ChampionId::Blitzcrank,"블리츠크랭크"=>ChampionId::Blitzcrank,"Блицкранк"=>ChampionId::Blitzcrank,"蒸汽机器人"=>ChampionId::Blitzcrank,"布里茨"=>ChampionId::Blitzcrank,"براند"=>ChampionId::Brand,"Brand"=>ChampionId::Brand,"Μπραντ"=>ChampionId::Brand,"ブランド"=>ChampionId::Brand,"브랜드"=>ChampionId::Brand,"Брэнд"=>ChampionId::Brand,"复仇焰魂"=>ChampionId::Brand,"布兰德"=>ChampionId::Brand,"布蘭德"=>ChampionId::Brand,"بروم"=>ChampionId::Braum,"Braum"=>ChampionId::Braum,"Μπράουμ"=>ChampionId::Braum,"ブラウム"=>ChampionId::Braum,"브라움"=>ChampionId::Braum,"Браум"=>ChampionId::Braum,"弗雷尔卓德之心"=>ChampionId::Braum,"布郎姆"=>ChampionId::Braum,"براير"=>ChampionId::Briar,"Briar"=>ChampionId::Briar,"Μπράιαρ"=>ChampionId::Briar,"ブライアー"=>ChampionId::Briar,"브라이어"=>ChampionId::Briar,"Брайер"=>ChampionId::Briar,"狂厄蔷薇"=>ChampionId::Briar,"布蕾尔"=>ChampionId::Briar,"布蕾爾"=>ChampionId::Briar,"كايتلين"=>ChampionId::Caitlyn,"Caitlyn"=>ChampionId::Caitlyn,"Κέιτλιν"=>ChampionId::Caitlyn,"ケイトリン"=>ChampionId::Caitlyn,"케이틀린"=>ChampionId::Caitlyn,"Кейтлин"=>ChampionId::Caitlyn,"皮城女警"=>ChampionId::Caitlyn,"凯特琳"=>ChampionId::Caitlyn,"凱特琳"=>ChampionId::Caitlyn,"كاميل"=>ChampionId::Camille,"Camille"=>ChampionId::Camille,"Καμίλ"=>ChampionId::Camille,"カミール"=>ChampionId::Camille,"카밀"=>ChampionId::Camille,"Камилла"=>ChampionId::Camille,"青钢影"=>ChampionId::Camille,"卡蜜儿"=>ChampionId::Camille,"卡蜜兒"=>ChampionId::Camille,"كاسيوبيا"=>ChampionId::Cassiopeia,"Cassiopeia"=>ChampionId::Cassiopeia,"Κασσιόπη"=>ChampionId::Cassiopeia,"カシオペア"=>ChampionId::Cassiopeia,"카시오페아"=>ChampionId::Cassiopeia,"Кассиопея"=>ChampionId::Cassiopeia,"魔蛇之拥"=>ChampionId::Cassiopeia,"卡莎碧雅"=>ChampionId::Cassiopeia,"تشوغاث"=>ChampionId::Chogath,"Cho'Gath"=>ChampionId::Chogath,"Τσο'Γκαθ"=>ChampionId::Chogath,"チョ＝ガス"=>ChampionId::Chogath,"초가스"=>ChampionId::Chogath,"Чо'Гат"=>ChampionId::Chogath,"虚空恐惧"=>ChampionId::Chogath,"科加斯"=>ChampionId::Chogath,"كوركي"=>ChampionId::Corki,"Corki"=>ChampionId::Corki,"Κόρκι"=>ChampionId::Corki,"コーキ"=>ChampionId::Corki,"코르키"=>ChampionId::Corki,"Корки"=>ChampionId::Corki,"英勇投弹手"=>ChampionId::Corki,"库奇"=>ChampionId::Corki,"庫奇"=>ChampionId::Corki,"داريوس"=>ChampionId::Darius,"Darius"=>ChampionId::Darius,"Ντάριους"=>ChampionId::Darius,"ダリウス"=>ChampionId::Darius,"다리우스"=>ChampionId::Darius,"Дариус"=>ChampionId::Darius,"诺克萨斯之手"=>ChampionId::Darius,"达瑞斯"=>ChampionId::Darius,"達瑞斯"=>ChampionId::Darius,"ديانا"=>ChampionId::Diana,"Diana"=>ChampionId::Diana,"Ντιάνα"=>ChampionId::Diana,"ダイアナ"=>ChampionId::Diana,"다이애나"=>ChampionId::Diana,"Диана"=>ChampionId::Diana,"皎月女神"=>ChampionId::Diana,"黛安娜"=>ChampionId::Diana,"د. موندو"=>ChampionId::DrMundo,"Dr. Mundo"=>ChampionId::DrMundo,"Δρ. Μούντο"=>ChampionId::DrMundo,"ドクター・ムンド"=>ChampionId::DrMundo,"문도 박사"=>ChampionId::DrMundo,"Dr Mundo"=>ChampionId::DrMundo,"Доктор Мундо"=>ChampionId::DrMundo,"祖安狂人"=>ChampionId::DrMundo,"蒙多医生"=>ChampionId::DrMundo,"蒙多醫生"=>ChampionId::DrMundo,"درايفن"=>ChampionId::Draven,"Draven"=>ChampionId::Draven,"Ντρέιβεν"=>ChampionId::Draven,"ドレイヴン"=>ChampionId::Draven,"드레이븐"=>ChampionId::Draven,"Дрейвен"=>ChampionId::Draven,"荣耀行刑官"=>ChampionId::Draven,"达瑞文"=>ChampionId::Draven,"達瑞文"=>ChampionId::Draven,"إيكو"=>ChampionId::Ekko,"Ekko"=>ChampionId::Ekko,"Έκκο"=>ChampionId::Ekko,"エコー"=>ChampionId::Ekko,"에코"=>ChampionId::Ekko,"Экко"=>ChampionId::Ekko,"时间刺客"=>ChampionId::Ekko,"艾克"=>ChampionId::Ekko,"إليز"=>ChampionId::Elise,"Elise"=>ChampionId::Elise,"Ελίζ"=>ChampionId::Elise,"エリス"=>ChampionId::Elise,"엘리스"=>ChampionId::Elise,"Элиза"=>ChampionId::Elise,"蜘蛛女皇"=>ChampionId::Elise,"伊莉丝"=>ChampionId::Elise,"伊莉絲"=>ChampionId::Elise,"إيفلين"=>ChampionId::Evelynn,"Evelynn"=>ChampionId::Evelynn,"Έβελιν"=>ChampionId::Evelynn,"イブリン"=>ChampionId::Evelynn,"이블린"=>ChampionId::Evelynn,"Эвелинн"=>ChampionId::Evelynn,"痛苦之拥"=>ChampionId::Evelynn,"伊芙琳"=>ChampionId::Evelynn,"إزريال"=>ChampionId::Ezreal,"Ezreal"=>ChampionId::Ezreal,"Έζρεαλ"=>ChampionId::Ezreal,"エズリアル"=>ChampionId::Ezreal,"이즈리얼"=>ChampionId::Ezreal,"Эзреаль"=>ChampionId::Ezreal,"探险家"=>ChampionId::Ezreal,"伊泽瑞尔"=>ChampionId::Ezreal,"伊澤瑞爾"=>ChampionId::Ezreal,"فيدل ستيكس"=>ChampionId::Fiddlesticks,"Fiddlesticks"=>ChampionId::Fiddlesticks,"Φίντλστιξ"=>ChampionId::Fiddlesticks,"フィドルスティックス"=>ChampionId::Fiddlesticks,"피들스틱"=>ChampionId::Fiddlesticks,"Фиддлстикс"=>ChampionId::Fiddlesticks,"远古恐惧"=>ChampionId::Fiddlesticks,"费德提克"=>ChampionId::Fiddlesticks,"費德提克"=>ChampionId::Fiddlesticks,"فيورا"=>ChampionId::Fiora,"Fiora"=>ChampionId::Fiora,"Φιόρα"=>ChampionId::Fiora,"フィオラ"=>ChampionId::Fiora,"피오라"=>ChampionId::Fiora,"Фиора"=>ChampionId::Fiora,"无双剑姬"=>ChampionId::Fiora,"菲欧拉"=>ChampionId::Fiora,"菲歐拉"=>ChampionId::Fiora,"فيز"=>ChampionId::Fizz,"Fizz"=>ChampionId::Fizz,"Φιζ"=>ChampionId::Fizz,"フィズ"=>ChampionId::Fizz,"피즈"=>ChampionId::Fizz,"Физз"=>ChampionId::Fizz,"潮汐海灵"=>ChampionId::Fizz,"飞斯"=>ChampionId::Fizz,"飛斯"=>ChampionId::Fizz,"غاليو"=>ChampionId::Galio,"Galio"=>ChampionId::Galio,"Γκάλιο"=>ChampionId::Galio,"ガリオ"=>ChampionId::Galio,"갈리오"=>ChampionId::Galio,"Галио"=>ChampionId::Galio,"正义巨像"=>ChampionId::Galio,"加里欧"=>ChampionId::Galio,"加里歐"=>ChampionId::Galio,"غانغ بلانك"=>ChampionId::Gangplank,"Gangplank"=>ChampionId::Gangplank,"Γκάνγκπλανκ"=>ChampionId::Gangplank,"ガングプランク"=>ChampionId::Gangplank,"갱플랭크"=>ChampionId::Gangplank,"Гангпланк"=>ChampionId::Gangplank,"海洋之灾"=>ChampionId::Gangplank,"刚普朗克"=>ChampionId::Gangplank,"剛普朗克"=>ChampionId::Gangplank,"غارين"=>ChampionId::Garen,"Garen"=>ChampionId::Garen,"Γκάρεν"=>ChampionId::Garen,"ガレン"=>ChampionId::Garen,"가렌"=>ChampionId::Garen,"Гарен"=>ChampionId::Garen,"德玛西亚之力"=>ChampionId::Garen,"盖伦"=>ChampionId::Garen,"蓋倫"=>ChampionId::Garen,"غنار"=>ChampionId::Gnar,"Gnar"=>ChampionId::Gnar,"Γκναρ"=>ChampionId::Gnar,"ナー"=>ChampionId::Gnar,"나르"=>ChampionId::Gnar,"Гнар"=>ChampionId::Gnar,"迷失之牙"=>ChampionId::Gnar,"呐儿"=>ChampionId::Gnar,"吶兒"=>ChampionId::Gnar,"غراغاس"=>ChampionId::Gragas,"Gragas"=>ChampionId::Gragas,"Γκράγκας"=>ChampionId::Gragas,"グラガス"=>ChampionId::Gragas,"그라가스"=>ChampionId::Gragas,"Грагас"=>ChampionId::Gragas,"酒桶"=>ChampionId::Gragas,"古拉格斯"=>ChampionId::Gragas,"غرايفز"=>ChampionId::Graves,"Graves"=>ChampionId::Graves,"Γκρέιβς"=>ChampionId::Graves,"グレイブス"=>ChampionId::Graves,"그레이브즈"=>ChampionId::Graves,"Грейвз"=>ChampionId::Graves,"法外狂徒"=>ChampionId::Graves,"葛雷夫"=>ChampionId::Graves,"غوين"=>ChampionId::Gwen,"Gwen"=>ChampionId::Gwen,"Γκουέν"=>ChampionId::Gwen,"グウェン"=>ChampionId::Gwen,"그웬"=>ChampionId::Gwen,"Гвен"=>ChampionId::Gwen,"灵罗娃娃"=>ChampionId::Gwen,"关"=>ChampionId::Gwen,"關"=>ChampionId::Gwen,"هيكاريم"=>ChampionId::Hecarim,"Hecarim"=>ChampionId::Hecarim,"Χέκαριμ"=>ChampionId::Hecarim,"ヘカリム"=>ChampionId::Hecarim,"헤카림"=>ChampionId::Hecarim,"Гекарим"=>ChampionId::Hecarim,"战争之影"=>ChampionId::Hecarim,"赫克林"=>ChampionId::Hecarim,"هايمردينغر"=>ChampionId::Heimerdinger,"Heimerdinger"=>ChampionId::Heimerdinger,"Χάιμερντιγκερ"=>ChampionId::Heimerdinger,"ハイマーディンガー"=>ChampionId::Heimerdinger,"하이머딩거"=>ChampionId::Heimerdinger,"Хеймердингер"=>ChampionId::Heimerdinger,"大发明家"=>ChampionId::Heimerdinger,"汉默丁格"=>ChampionId::Heimerdinger,"漢默丁格"=>ChampionId::Heimerdinger,"هواي"=>ChampionId::Hwei,"Hwei"=>ChampionId::Hwei,"Χουέι"=>ChampionId::Hwei,"フェイ"=>ChampionId::Hwei,"흐웨이"=>ChampionId::Hwei,"Хвэй"=>ChampionId::Hwei,"异画师"=>ChampionId::Hwei,"慧"=>ChampionId::Hwei,"赫威"=>ChampionId::Hwei,"إيلاوي"=>ChampionId::Illaoi,"Illaoi"=>ChampionId::Illaoi,"Ιλλαόη"=>ChampionId::Illaoi,"イラオイ"=>ChampionId::Illaoi,"일라오이"=>ChampionId::Illaoi,"Иллаой"=>ChampionId::Illaoi,"海兽祭司"=>ChampionId::Illaoi,"伊罗旖"=>ChampionId::Illaoi,"伊羅旖"=>ChampionId::Illaoi,"إيريليا"=>ChampionId::Irelia,"Irelia"=>ChampionId::Irelia,"Ιρέλια"=>ChampionId::Irelia,"イレリア"=>ChampionId::Irelia,"이렐리아"=>ChampionId::Irelia,"Ирелия"=>ChampionId::Irelia,"刀锋舞者"=>ChampionId::Irelia,"伊瑞莉雅"=>ChampionId::Irelia,"آيفرن"=>ChampionId::Ivern,"Ivern"=>ChampionId::Ivern,"Άιβερν"=>ChampionId::Ivern,"アイバーン"=>ChampionId::Ivern,"아이번"=>ChampionId::Ivern,"Иверн"=>ChampionId::Ivern,"翠神"=>ChampionId::Ivern,"埃尔文"=>ChampionId::Ivern,"埃爾文"=>ChampionId::Ivern,"جانا"=>ChampionId::Janna,"Janna"=>ChampionId::Janna,"Τζάνα"=>ChampionId::Janna,"ジャンナ"=>ChampionId::Janna,"잔나"=>ChampionId::Janna,"Жанна"=>ChampionId::Janna,"风暴之怒"=>ChampionId::Janna,"珍娜"=>ChampionId::Janna,"جارفان الرابع"=>ChampionId::JarvanIV,"Jarvan IV"=>ChampionId::JarvanIV,"Jarvan IV."=>ChampionId::JarvanIV,"Τζάρβαν ο Δ'"=>ChampionId::JarvanIV,"IV. Jarvan"=>ChampionId::JarvanIV,"ジャーヴァンⅣ"=>ChampionId::JarvanIV,"자르반 4세"=>ChampionId::JarvanIV,"Джарван IV"=>ChampionId::JarvanIV,"德玛西亚皇子"=>ChampionId::JarvanIV,"嘉文四世"=>ChampionId::JarvanIV,"جاكس"=>ChampionId::Jax,"Jax"=>ChampionId::Jax,"Τζαξ"=>ChampionId::Jax,"ジャックス"=>ChampionId::Jax,"잭스"=>ChampionId::Jax,"Джакс"=>ChampionId::Jax,"武器大师"=>ChampionId::Jax,"贾克斯"=>ChampionId::Jax,"賈克斯"=>ChampionId::Jax,"جايس"=>ChampionId::Jayce,"Jayce"=>ChampionId::Jayce,"Τζέις"=>ChampionId::Jayce,"ジェイス"=>ChampionId::Jayce,"제이스"=>ChampionId::Jayce,"Джейс"=>ChampionId::Jayce,"未来守护者"=>ChampionId::Jayce,"杰西"=>ChampionId::Jayce,"جين"=>ChampionId::Jhin,"Jhin"=>ChampionId::Jhin,"Τζιν"=>ChampionId::Jhin,"ジン"=>ChampionId::Jhin,"진"=>ChampionId::Jhin,"Джин"=>ChampionId::Jhin,"戏命师"=>ChampionId::Jhin,"烬"=>ChampionId::Jhin,"燼"=>ChampionId::Jhin,"جينكس"=>ChampionId::Jinx,"Jinx"=>ChampionId::Jinx,"Τζινξ"=>ChampionId::Jinx,"ジンクス"=>ChampionId::Jinx,"징크스"=>ChampionId::Jinx,"Джинкс"=>ChampionId::Jinx,"暴走萝莉"=>ChampionId::Jinx,"吉茵珂丝"=>ChampionId::Jinx,"吉茵珂絲"=>ChampionId::Jinx,"كاسانتي"=>ChampionId::KSante,"K'Sante"=>ChampionId::KSante,"Κα'Σάντι"=>ChampionId::KSante,"K'Santé"=>ChampionId::KSante,"カ・サンテ"=>ChampionId::KSante,"크산테"=>ChampionId::KSante,"К'Санте"=>ChampionId::KSante,"纳祖芒荣耀"=>ChampionId::KSante,"卡桑帝"=>ChampionId::KSante,"كايسا"=>ChampionId::Kaisa,"Kai'Sa"=>ChampionId::Kaisa,"Κάι'Σα"=>ChampionId::Kaisa,"カイ＝サ"=>ChampionId::Kaisa,"카이사"=>ChampionId::Kaisa,"Кай'Са"=>ChampionId::Kaisa,"虚空之女"=>ChampionId::Kaisa,"凯莎"=>ChampionId::Kaisa,"凱莎"=>ChampionId::Kaisa,"كاليستا"=>ChampionId::Kalista,"Kalista"=>ChampionId::Kalista,"Καλίστα"=>ChampionId::Kalista,"カリスタ"=>ChampionId::Kalista,"칼리스타"=>ChampionId::Kalista,"Калиста"=>ChampionId::Kalista,"复仇之矛"=>ChampionId::Kalista,"克黎思妲"=>ChampionId::Kalista,"كارما"=>ChampionId::Karma,"Karma"=>ChampionId::Karma,"Κάρμα"=>ChampionId::Karma,"カルマ"=>ChampionId::Karma,"카르마"=>ChampionId::Karma,"Карма"=>ChampionId::Karma,"天启者"=>ChampionId::Karma,"卡玛"=>ChampionId::Karma,"卡瑪"=>ChampionId::Karma,"كارثوس"=>ChampionId::Karthus,"Karthus"=>ChampionId::Karthus,"Κάρθους"=>ChampionId::Karthus,"カーサス"=>ChampionId::Karthus,"카서스"=>ChampionId::Karthus,"Картус"=>ChampionId::Karthus,"死亡颂唱者"=>ChampionId::Karthus,"卡尔瑟斯"=>ChampionId::Karthus,"卡爾瑟斯"=>ChampionId::Karthus,"كاسادين"=>ChampionId::Kassadin,"Kassadin"=>ChampionId::Kassadin,"Κάσαντιν"=>ChampionId::Kassadin,"カサディン"=>ChampionId::Kassadin,"카사딘"=>ChampionId::Kassadin,"Кассадин"=>ChampionId::Kassadin,"虚空行者"=>ChampionId::Kassadin,"卡萨丁"=>ChampionId::Kassadin,"卡薩丁"=>ChampionId::Kassadin,"كاتارينا"=>ChampionId::Katarina,"Katarina"=>ChampionId::Katarina,"Καταρίνα"=>ChampionId::Katarina,"カタリナ"=>ChampionId::Katarina,"카타리나"=>ChampionId::Katarina,"Катарина"=>ChampionId::Katarina,"不祥之刃"=>ChampionId::Katarina,"卡特莲娜"=>ChampionId::Katarina,"卡特蓮娜"=>ChampionId::Katarina,"كايل"=>ChampionId::Kayle,"Kayle"=>ChampionId::Kayle,"Κέιλ"=>ChampionId::Kayle,"ケイル"=>ChampionId::Kayle,"케일"=>ChampionId::Kayle,"Кейл"=>ChampionId::Kayle,"正义天使"=>ChampionId::Kayle,"凯尔"=>ChampionId::Kayle,"凱爾"=>ChampionId::Kayle,"كاين"=>ChampionId::Kayn,"Kayn"=>ChampionId::Kayn,"Κέιν"=>ChampionId::Kayn,"ケイン"=>ChampionId::Kayn,"케인"=>ChampionId::Kayn,"Каин"=>ChampionId::Kayn,"影流之镰"=>ChampionId::Kayn,"慨影"=>ChampionId::Kayn,"كينين"=>ChampionId::Kennen,"Kennen"=>ChampionId::Kennen,"Κένεν"=>ChampionId::Kennen,"ケネン"=>ChampionId::Kennen,"케넨"=>ChampionId::Kennen,"Кеннен"=>ChampionId::Kennen,"狂暴之心"=>ChampionId::Kennen,"凯能"=>ChampionId::Kennen,"凱能"=>ChampionId::Kennen,"كازيكس"=>ChampionId::Khazix,"Kha'Zix"=>ChampionId::Khazix,"Κα'Ζιξ"=>ChampionId::Khazix,"カ＝ジックス"=>ChampionId::Khazix,"카직스"=>ChampionId::Khazix,"Ка'Зикс"=>ChampionId::Khazix,"虚空掠夺者"=>ChampionId::Khazix,"卡力斯"=>ChampionId::Khazix,"كيندريد"=>ChampionId::Kindred,"Kindred"=>ChampionId::Kindred,"Κίντρεντ"=>ChampionId::Kindred,"キンドレッド"=>ChampionId::Kindred,"킨드레드"=>ChampionId::Kindred,"Киндред"=>ChampionId::Kindred,"永猎双子"=>ChampionId::Kindred,"镜爪"=>ChampionId::Kindred,"鏡爪"=>ChampionId::Kindred,"كليد"=>ChampionId::Kled,"Kled"=>ChampionId::Kled,"Κλεντ"=>ChampionId::Kled,"クレッド"=>ChampionId::Kled,"클레드"=>ChampionId::Kled,"Клед"=>ChampionId::Kled,"暴怒骑士"=>ChampionId::Kled,"克雷德"=>ChampionId::Kled,"كوغ ماو"=>ChampionId::KogMaw,"Kog'Maw"=>ChampionId::KogMaw,"Κογκ'Μο"=>ChampionId::KogMaw,"コグ＝マウ"=>ChampionId::KogMaw,"코그모"=>ChampionId::KogMaw,"Ког'Мао"=>ChampionId::KogMaw,"深渊巨口"=>ChampionId::KogMaw,"寇格魔"=>ChampionId::KogMaw,"لوبلانك"=>ChampionId::Leblanc,"LeBlanc"=>ChampionId::Leblanc,"ΛεΜπλάν"=>ChampionId::Leblanc,"ルブラン"=>ChampionId::Leblanc,"르블랑"=>ChampionId::Leblanc,"Ле Блан"=>ChampionId::Leblanc,"诡术妖姬"=>ChampionId::Leblanc,"勒布朗"=>ChampionId::Leblanc,"لي سين"=>ChampionId::LeeSin,"Lee Sin"=>ChampionId::LeeSin,"Λι Σιν"=>ChampionId::LeeSin,"リー・シン"=>ChampionId::LeeSin,"리 신"=>ChampionId::LeeSin,"Ли Син"=>ChampionId::LeeSin,"盲僧"=>ChampionId::LeeSin,"李星"=>ChampionId::LeeSin,"ليونا"=>ChampionId::Leona,"Leona"=>ChampionId::Leona,"Λεόνα"=>ChampionId::Leona,"レオナ"=>ChampionId::Leona,"레오나"=>ChampionId::Leona,"Леона"=>ChampionId::Leona,"曙光女神"=>ChampionId::Leona,"雷欧娜"=>ChampionId::Leona,"雷歐娜"=>ChampionId::Leona,"ليليا"=>ChampionId::Lillia,"Lillia"=>ChampionId::Lillia,"Λίλια"=>ChampionId::Lillia,"リリア"=>ChampionId::Lillia,"릴리아"=>ChampionId::Lillia,"Лиллия"=>ChampionId::Lillia,"含羞蓓蕾"=>ChampionId::Lillia,"莉莉亚"=>ChampionId::Lillia,"莉莉亞"=>ChampionId::Lillia,"ليساندرا"=>ChampionId::Lissandra,"Lissandra"=>ChampionId::Lissandra,"Λισάντρα"=>ChampionId::Lissandra,"リサンドラ"=>ChampionId::Lissandra,"리산드라"=>ChampionId::Lissandra,"Лиссандра"=>ChampionId::Lissandra,"冰霜女巫"=>ChampionId::Lissandra,"丽珊卓"=>ChampionId::Lissandra,"麗珊卓"=>ChampionId::Lissandra,"لوشيان"=>ChampionId::Lucian,"Lucian"=>ChampionId::Lucian,"Λούσιαν"=>ChampionId::Lucian,"ルシアン"=>ChampionId::Lucian,"루시안"=>ChampionId::Lucian,"Люциан"=>ChampionId::Lucian,"圣枪游侠"=>ChampionId::Lucian,"路西恩"=>ChampionId::Lucian,"لولو"=>ChampionId::Lulu,"Lulu"=>ChampionId::Lulu,"Λούλου"=>ChampionId::Lulu,"ルル"=>ChampionId::Lulu,"룰루"=>ChampionId::Lulu,"Лулу"=>ChampionId::Lulu,"仙灵女巫"=>ChampionId::Lulu,"露璐"=>ChampionId::Lulu,"لكس"=>ChampionId::Lux,"Lux"=>ChampionId::Lux,"Λουξ"=>ChampionId::Lux,"ラックス"=>ChampionId::Lux,"럭스"=>ChampionId::Lux,"Люкс"=>ChampionId::Lux,"光辉女郎"=>ChampionId::Lux,"拉克丝"=>ChampionId::Lux,"拉克絲"=>ChampionId::Lux,"مالفايت"=>ChampionId::Malphite,"Malphite"=>ChampionId::Malphite,"Μάλφαϊτ"=>ChampionId::Malphite,"マルファイト"=>ChampionId::Malphite,"말파이트"=>ChampionId::Malphite,"Мальфит"=>ChampionId::Malphite,"熔岩巨兽"=>ChampionId::Malphite,"墨菲特"=>ChampionId::Malphite,"مالزاهار"=>ChampionId::Malzahar,"Malzahar"=>ChampionId::Malzahar,"Μάλζαχαρ"=>ChampionId::Malzahar,"マルザハール"=>ChampionId::Malzahar,"말자하"=>ChampionId::Malzahar,"Мальзахар"=>ChampionId::Malzahar,"虚空先知"=>ChampionId::Malzahar,"马尔札哈"=>ChampionId::Malzahar,"馬爾札哈"=>ChampionId::Malzahar,"ماوكاي"=>ChampionId::Maokai,"Maokai"=>ChampionId::Maokai,"Μαοκάι"=>ChampionId::Maokai,"マオカイ"=>ChampionId::Maokai,"마오카이"=>ChampionId::Maokai,"Маокай"=>ChampionId::Maokai,"扭曲树精"=>ChampionId::Maokai,"茂凯"=>ChampionId::Maokai,"茂凱"=>ChampionId::Maokai,"ماستر يي"=>ChampionId::MasterYi,"Master Yi"=>ChampionId::MasterYi,"Mistr Yi"=>ChampionId::MasterYi,"Μάστερ Γι"=>ChampionId::MasterYi,"Maestro Yi"=>ChampionId::MasterYi,"Maître Yi"=>ChampionId::MasterYi,"マスター・イー"=>ChampionId::MasterYi,"마스터 이"=>ChampionId::MasterYi,"Мастер Йи"=>ChampionId::MasterYi,"无极剑圣"=>ChampionId::MasterYi,"易大师"=>ChampionId::MasterYi,"易大師"=>ChampionId::MasterYi,"ميل"=>ChampionId::Mel,"Mel"=>ChampionId::Mel,"Μελ"=>ChampionId::Mel,"メル"=>ChampionId::Mel,"멜"=>ChampionId::Mel,"Мэл"=>ChampionId::Mel,"流光镜影"=>ChampionId::Mel,"梅尔"=>ChampionId::Mel,"梅爾"=>ChampionId::Mel,"ميليو"=>ChampionId::Milio,"Milio"=>ChampionId::Milio,"Μίλιο"=>ChampionId::Milio,"ミリオ"=>ChampionId::Milio,"밀리오"=>ChampionId::Milio,"Милио"=>ChampionId::Milio,"明烛"=>ChampionId::Milio,"米里欧"=>ChampionId::Milio,"米里歐"=>ChampionId::Milio,"ميس فورتشن"=>ChampionId::MissFortune,"Miss Fortune"=>ChampionId::MissFortune,"Μις Φόρτσουν"=>ChampionId::MissFortune,"ミス・フォーチュン"=>ChampionId::MissFortune,"미스 포츈"=>ChampionId::MissFortune,"Мисс Фортуна"=>ChampionId::MissFortune,"赏金猎人"=>ChampionId::MissFortune,"好运姐"=>ChampionId::MissFortune,"好運姐"=>ChampionId::MissFortune,"ووكونغ"=>ChampionId::MonkeyKing,"Wukong"=>ChampionId::MonkeyKing,"Γουκόνγκ"=>ChampionId::MonkeyKing,"ウーコン"=>ChampionId::MonkeyKing,"오공"=>ChampionId::MonkeyKing,"Вуконг"=>ChampionId::MonkeyKing,"Ngộ Không"=>ChampionId::MonkeyKing,"齐天大圣"=>ChampionId::MonkeyKing,"悟空"=>ChampionId::MonkeyKing,"مورديكايزر"=>ChampionId::Mordekaiser,"Mordekaiser"=>ChampionId::Mordekaiser,"Μορντεκάιζερ"=>ChampionId::Mordekaiser,"モルデカイザー"=>ChampionId::Mordekaiser,"모데카이저"=>ChampionId::Mordekaiser,"Мордекайзер"=>ChampionId::Mordekaiser,"铁铠冥魂"=>ChampionId::Mordekaiser,"魔斗凯萨"=>ChampionId::Mordekaiser,"魔鬥凱薩"=>ChampionId::Mordekaiser,"مورغانا"=>ChampionId::Morgana,"Morgana"=>ChampionId::Morgana,"Μοργκάνα"=>ChampionId::Morgana,"モルガナ"=>ChampionId::Morgana,"모르가나"=>ChampionId::Morgana,"Моргана"=>ChampionId::Morgana,"堕落天使"=>ChampionId::Morgana,"魔甘娜"=>ChampionId::Morgana,"نافيري"=>ChampionId::Naafiri,"Naafiri"=>ChampionId::Naafiri,"Νααφίρι"=>ChampionId::Naafiri,"ナフィーリ"=>ChampionId::Naafiri,"나피리"=>ChampionId::Naafiri,"Наафири"=>ChampionId::Naafiri,"百裂冥犬"=>ChampionId::Naafiri,"纳菲利"=>ChampionId::Naafiri,"娜菲芮"=>ChampionId::Naafiri,"نامي"=>ChampionId::Nami,"Nami"=>ChampionId::Nami,"Νάμι"=>ChampionId::Nami,"ナミ"=>ChampionId::Nami,"나미"=>ChampionId::Nami,"Нами"=>ChampionId::Nami,"唤潮鲛姬"=>ChampionId::Nami,"娜米"=>ChampionId::Nami,"ناسوس"=>ChampionId::Nasus,"Nasus"=>ChampionId::Nasus,"Νάσους"=>ChampionId::Nasus,"ナサス"=>ChampionId::Nasus,"나서스"=>ChampionId::Nasus,"Насус"=>ChampionId::Nasus,"沙漠死神"=>ChampionId::Nasus,"纳瑟斯"=>ChampionId::Nasus,"納瑟斯"=>ChampionId::Nasus,"نوتيلوس"=>ChampionId::Nautilus,"Nautilus"=>ChampionId::Nautilus,"Νότιλους"=>ChampionId::Nautilus,"ノーチラス"=>ChampionId::Nautilus,"노틸러스"=>ChampionId::Nautilus,"Наутилус"=>ChampionId::Nautilus,"深海泰坦"=>ChampionId::Nautilus,"纳帝鲁斯"=>ChampionId::Nautilus,"納帝魯斯"=>ChampionId::Nautilus,"نِكو"=>ChampionId::Neeko,"Neeko"=>ChampionId::Neeko,"Νίκκο"=>ChampionId::Neeko,"ニーコ"=>ChampionId::Neeko,"니코"=>ChampionId::Neeko,"Нико"=>ChampionId::Neeko,"万花通灵"=>ChampionId::Neeko,"妮可"=>ChampionId::Neeko,"نيدالي"=>ChampionId::Nidalee,"Nidalee"=>ChampionId::Nidalee,"Νίνταλι"=>ChampionId::Nidalee,"ニダリー"=>ChampionId::Nidalee,"니달리"=>ChampionId::Nidalee,"Нидали"=>ChampionId::Nidalee,"狂野女猎手"=>ChampionId::Nidalee,"奈德丽"=>ChampionId::Nidalee,"奈德麗"=>ChampionId::Nidalee,"نيلا"=>ChampionId::Nilah,"Nilah"=>ChampionId::Nilah,"Νάιλα"=>ChampionId::Nilah,"ニーラ"=>ChampionId::Nilah,"닐라"=>ChampionId::Nilah,"Нила"=>ChampionId::Nilah,"不羁之悦"=>ChampionId::Nilah,"淣菈"=>ChampionId::Nilah,"نوكتورن"=>ChampionId::Nocturne,"Nocturne"=>ChampionId::Nocturne,"Νόκτουρν"=>ChampionId::Nocturne,"ノクターン"=>ChampionId::Nocturne,"녹턴"=>ChampionId::Nocturne,"Ноктюрн"=>ChampionId::Nocturne,"永恒梦魇"=>ChampionId::Nocturne,"夜曲"=>ChampionId::Nocturne,"نونو وويلامب"=>ChampionId::Nunu,"Nunu & Willump"=>ChampionId::Nunu,"Nunu a Willump"=>ChampionId::Nunu,"Νούνου και Γουίλαμπ"=>ChampionId::Nunu,"Nunu y Willump"=>ChampionId::Nunu,"Nunu et Willump"=>ChampionId::Nunu,"Nunu és Willump"=>ChampionId::Nunu,"Nunu e Willump"=>ChampionId::Nunu,"ヌヌ＆ウィルンプ"=>ChampionId::Nunu,"누누와 윌럼프"=>ChampionId::Nunu,"Nunu i Willump"=>ChampionId::Nunu,"Nunu și Willump"=>ChampionId::Nunu,"Нуну и Виллумп"=>ChampionId::Nunu,"Nunu ve Willump"=>ChampionId::Nunu,"雪原双子"=>ChampionId::Nunu,"努努和威朗普"=>ChampionId::Nunu,"أولاف"=>ChampionId::Olaf,"Olaf"=>ChampionId::Olaf,"Όλαφ"=>ChampionId::Olaf,"オラフ"=>ChampionId::Olaf,"올라프"=>ChampionId::Olaf,"Олаф"=>ChampionId::Olaf,"狂战士"=>ChampionId::Olaf,"欧拉夫"=>ChampionId::Olaf,"歐拉夫"=>ChampionId::Olaf,"أوريانا"=>ChampionId::Orianna,"Orianna"=>ChampionId::Orianna,"Οριάνα"=>ChampionId::Orianna,"オリアナ"=>ChampionId::Orianna,"오리아나"=>ChampionId::Orianna,"Орианна"=>ChampionId::Orianna,"发条魔灵"=>ChampionId::Orianna,"奥莉安娜"=>ChampionId::Orianna,"奧莉安娜"=>ChampionId::Orianna,"أورن"=>ChampionId::Ornn,"Ornn"=>ChampionId::Ornn,"Ορν"=>ChampionId::Ornn,"オーン"=>ChampionId::Ornn,"오른"=>ChampionId::Ornn,"Орн"=>ChampionId::Ornn,"山隐之焰"=>ChampionId::Ornn,"鄂尔"=>ChampionId::Ornn,"鄂爾"=>ChampionId::Ornn,"بانثيون"=>ChampionId::Pantheon,"Pantheon"=>ChampionId::Pantheon,"Πάνθεον"=>ChampionId::Pantheon,"パンテオン"=>ChampionId::Pantheon,"판테온"=>ChampionId::Pantheon,"Пантеон"=>ChampionId::Pantheon,"不屈之枪"=>ChampionId::Pantheon,"潘森"=>ChampionId::Pantheon,"بوبي"=>ChampionId::Poppy,"Poppy"=>ChampionId::Poppy,"Πόπι"=>ChampionId::Poppy,"ポッピー"=>ChampionId::Poppy,"뽀삐"=>ChampionId::Poppy,"Поппи"=>ChampionId::Poppy,"圣锤之毅"=>ChampionId::Poppy,"波比"=>ChampionId::Poppy,"بايك"=>ChampionId::Pyke,"Pyke"=>ChampionId::Pyke,"Πάικ"=>ChampionId::Pyke,"パイク"=>ChampionId::Pyke,"파이크"=>ChampionId::Pyke,"Пайк"=>ChampionId::Pyke,"血港鬼影"=>ChampionId::Pyke,"派克"=>ChampionId::Pyke,"كيانا"=>ChampionId::Qiyana,"Qiyana"=>ChampionId::Qiyana,"Κιάνα"=>ChampionId::Qiyana,"キヤナ"=>ChampionId::Qiyana,"키아나"=>ChampionId::Qiyana,"Киана"=>ChampionId::Qiyana,"元素女皇"=>ChampionId::Qiyana,"姬亚娜"=>ChampionId::Qiyana,"姬亞娜"=>ChampionId::Qiyana,"كوين"=>ChampionId::Quinn,"Quinn"=>ChampionId::Quinn,"Κουίν"=>ChampionId::Quinn,"クイン"=>ChampionId::Quinn,"퀸"=>ChampionId::Quinn,"Квинн"=>ChampionId::Quinn,"德玛西亚之翼"=>ChampionId::Quinn,"葵恩"=>ChampionId::Quinn,"راكان"=>ChampionId::Rakan,"Rakan"=>ChampionId::Rakan,"Ρακάν"=>ChampionId::Rakan,"ラカン"=>ChampionId::Rakan,"라칸"=>ChampionId::Rakan,"Рэйкан"=>ChampionId::Rakan,"幻翎"=>ChampionId::Rakan,"锐空"=>ChampionId::Rakan,"銳空"=>ChampionId::Rakan,"راموس"=>ChampionId::Rammus,"Rammus"=>ChampionId::Rammus,"Ράμους"=>ChampionId::Rammus,"ラムス"=>ChampionId::Rammus,"람머스"=>ChampionId::Rammus,"Раммус"=>ChampionId::Rammus,"披甲龙龟"=>ChampionId::Rammus,"拉姆斯"=>ChampionId::Rammus,"ريكساي"=>ChampionId::RekSai,"Rek'Sai"=>ChampionId::RekSai,"Ρεκ'Σάι"=>ChampionId::RekSai,"レク＝サイ"=>ChampionId::RekSai,"렉사이"=>ChampionId::RekSai,"Рек'Сай"=>ChampionId::RekSai,"虚空遁地兽"=>ChampionId::RekSai,"雷珂煞"=>ChampionId::RekSai,"ريل"=>ChampionId::Rell,"Rell"=>ChampionId::Rell,"Ρελ"=>ChampionId::Rell,"レル"=>ChampionId::Rell,"렐"=>ChampionId::Rell,"Релл"=>ChampionId::Rell,"镕铁少女"=>ChampionId::Rell,"锐儿"=>ChampionId::Rell,"銳兒"=>ChampionId::Rell,"ريناتا غلاسك"=>ChampionId::Renata,"Renata Glasc"=>ChampionId::Renata,"Ρενάτα Γκλασκ"=>ChampionId::Renata,"レナータ・グラスク"=>ChampionId::Renata,"레나타 글라스크"=>ChampionId::Renata,"Рената Гласк"=>ChampionId::Renata,"炼金男爵"=>ChampionId::Renata,"睿娜妲‧格莱斯克"=>ChampionId::Renata,"睿娜妲‧格萊斯克"=>ChampionId::Renata,"رينيكتون"=>ChampionId::Renekton,"Renekton"=>ChampionId::Renekton,"Ρένεκτον"=>ChampionId::Renekton,"レネクトン"=>ChampionId::Renekton,"레넥톤"=>ChampionId::Renekton,"Ренектон"=>ChampionId::Renekton,"荒漠屠夫"=>ChampionId::Renekton,"雷尼克顿"=>ChampionId::Renekton,"雷尼克頓"=>ChampionId::Renekton,"رينغار"=>ChampionId::Rengar,"Rengar"=>ChampionId::Rengar,"Ρένγκαρ"=>ChampionId::Rengar,"レンガー"=>ChampionId::Rengar,"렝가"=>ChampionId::Rengar,"Ренгар"=>ChampionId::Rengar,"傲之追猎者"=>ChampionId::Rengar,"雷葛尔"=>ChampionId::Rengar,"雷葛爾"=>ChampionId::Rengar,"ريفين"=>ChampionId::Riven,"Riven"=>ChampionId::Riven,"Ρίβεν"=>ChampionId::Riven,"リヴェン"=>ChampionId::Riven,"리븐"=>ChampionId::Riven,"Ривен"=>ChampionId::Riven,"放逐之刃"=>ChampionId::Riven,"雷玟"=>ChampionId::Riven,"رامبل"=>ChampionId::Rumble,"Rumble"=>ChampionId::Rumble,"Ραμπλ"=>ChampionId::Rumble,"ランブル"=>ChampionId::Rumble,"럼블"=>ChampionId::Rumble,"Рамбл"=>ChampionId::Rumble,"机械公敌"=>ChampionId::Rumble,"蓝宝"=>ChampionId::Rumble,"藍寶"=>ChampionId::Rumble,"رايز"=>ChampionId::Ryze,"Ryze"=>ChampionId::Ryze,"Ράιζ"=>ChampionId::Ryze,"ライズ"=>ChampionId::Ryze,"라이즈"=>ChampionId::Ryze,"Райз"=>ChampionId::Ryze,"符文法师"=>ChampionId::Ryze,"雷兹"=>ChampionId::Ryze,"雷茲"=>ChampionId::Ryze,"سميرة"=>ChampionId::Samira,"Samira"=>ChampionId::Samira,"Σαμίρα"=>ChampionId::Samira,"サミーラ"=>ChampionId::Samira,"사미라"=>ChampionId::Samira,"Самира"=>ChampionId::Samira,"沙漠玫瑰"=>ChampionId::Samira,"煞蜜拉"=>ChampionId::Samira,"سيجواني"=>ChampionId::Sejuani,"Sejuani"=>ChampionId::Sejuani,"Σεζουάνι"=>ChampionId::Sejuani,"セジュアニ"=>ChampionId::Sejuani,"세주아니"=>ChampionId::Sejuani,"Седжуани"=>ChampionId::Sejuani,"北地之怒"=>ChampionId::Sejuani,"史瓦妮"=>ChampionId::Sejuani,"سينا"=>ChampionId::Senna,"Senna"=>ChampionId::Senna,"Σέννα"=>ChampionId::Senna,"セナ"=>ChampionId::Senna,"세나"=>ChampionId::Senna,"Сенна"=>ChampionId::Senna,"涤魂圣枪"=>ChampionId::Senna,"姗娜"=>ChampionId::Senna,"姍娜"=>ChampionId::Senna,"سيرافين"=>ChampionId::Seraphine,"Seraphine"=>ChampionId::Seraphine,"Σεραφίν"=>ChampionId::Seraphine,"Séraphine"=>ChampionId::Seraphine,"セラフィーン"=>ChampionId::Seraphine,"세라핀"=>ChampionId::Seraphine,"Серафина"=>ChampionId::Seraphine,"星籁歌姬"=>ChampionId::Seraphine,"瑟菈纷"=>ChampionId::Seraphine,"瑟菈紛"=>ChampionId::Seraphine,"سيت"=>ChampionId::Sett,"Sett"=>ChampionId::Sett,"Σεττ"=>ChampionId::Sett,"セト"=>ChampionId::Sett,"세트"=>ChampionId::Sett,"Сетт"=>ChampionId::Sett,"腕豪"=>ChampionId::Sett,"赛特"=>ChampionId::Sett,"賽特"=>ChampionId::Sett,"شاكو"=>ChampionId::Shaco,"Shaco"=>ChampionId::Shaco,"Σάκο"=>ChampionId::Shaco,"シャコ"=>ChampionId::Shaco,"샤코"=>ChampionId::Shaco,"Шако"=>ChampionId::Shaco,"恶魔小丑"=>ChampionId::Shaco,"萨科"=>ChampionId::Shaco,"薩科"=>ChampionId::Shaco,"شين"=>ChampionId::Shen,"Shen"=>ChampionId::Shen,"Σεν"=>ChampionId::Shen,"シェン"=>ChampionId::Shen,"쉔"=>ChampionId::Shen,"Шен"=>ChampionId::Shen,"暮光之眼"=>ChampionId::Shen,"慎"=>ChampionId::Shen,"شيفانا"=>ChampionId::Shyvana,"Shyvana"=>ChampionId::Shyvana,"Σιβάνα"=>ChampionId::Shyvana,"シヴァーナ"=>ChampionId::Shyvana,"쉬바나"=>ChampionId::Shyvana,"Шивана"=>ChampionId::Shyvana,"龙血武姬"=>ChampionId::Shyvana,"希瓦娜"=>ChampionId::Shyvana,"سينجد"=>ChampionId::Singed,"Singed"=>ChampionId::Singed,"Σιντζντ"=>ChampionId::Singed,"シンジド"=>ChampionId::Singed,"신지드"=>ChampionId::Singed,"Синджед"=>ChampionId::Singed,"炼金术士"=>ChampionId::Singed,"辛吉德"=>ChampionId::Singed,"سايون"=>ChampionId::Sion,"Sion"=>ChampionId::Sion,"Σάιον"=>ChampionId::Sion,"サイオン"=>ChampionId::Sion,"사이온"=>ChampionId::Sion,"Сион"=>ChampionId::Sion,"亡灵战神"=>ChampionId::Sion,"赛恩"=>ChampionId::Sion,"賽恩"=>ChampionId::Sion,"سيفير"=>ChampionId::Sivir,"Sivir"=>ChampionId::Sivir,"Σίβιρ"=>ChampionId::Sivir,"シヴィア"=>ChampionId::Sivir,"시비르"=>ChampionId::Sivir,"Сивир"=>ChampionId::Sivir,"战争女神"=>ChampionId::Sivir,"希维尔"=>ChampionId::Sivir,"希維爾"=>ChampionId::Sivir,"سكارنر"=>ChampionId::Skarner,"Skarner"=>ChampionId::Skarner,"Σκάρνερ"=>ChampionId::Skarner,"スカーナー"=>ChampionId::Skarner,"스카너"=>ChampionId::Skarner,"Скарнер"=>ChampionId::Skarner,"上古领主"=>ChampionId::Skarner,"史加纳"=>ChampionId::Skarner,"史加納"=>ChampionId::Skarner,"سمولدر"=>ChampionId::Smolder,"Smolder"=>ChampionId::Smolder,"Σμόλντερ"=>ChampionId::Smolder,"スモルダー"=>ChampionId::Smolder,"스몰더"=>ChampionId::Smolder,"Смолдер"=>ChampionId::Smolder,"炽炎雏龙"=>ChampionId::Smolder,"烟炎"=>ChampionId::Smolder,"史矛德"=>ChampionId::Smolder,"سونا"=>ChampionId::Sona,"Sona"=>ChampionId::Sona,"Σόνα"=>ChampionId::Sona,"ソナ"=>ChampionId::Sona,"소나"=>ChampionId::Sona,"Сона"=>ChampionId::Sona,"琴瑟仙女"=>ChampionId::Sona,"索娜"=>ChampionId::Sona,"سوراكا"=>ChampionId::Soraka,"Soraka"=>ChampionId::Soraka,"Σοράκα"=>ChampionId::Soraka,"ソラカ"=>ChampionId::Soraka,"소라카"=>ChampionId::Soraka,"Сорака"=>ChampionId::Soraka,"众星之子"=>ChampionId::Soraka,"索拉卡"=>ChampionId::Soraka,"سواين"=>ChampionId::Swain,"Swain"=>ChampionId::Swain,"Σουέιν"=>ChampionId::Swain,"スウェイン"=>ChampionId::Swain,"스웨인"=>ChampionId::Swain,"Свейн"=>ChampionId::Swain,"诺克萨斯统领"=>ChampionId::Swain,"斯温"=>ChampionId::Swain,"斯溫"=>ChampionId::Swain,"سايلاس"=>ChampionId::Sylas,"Sylas"=>ChampionId::Sylas,"Σάιλας"=>ChampionId::Sylas,"サイラス"=>ChampionId::Sylas,"사일러스"=>ChampionId::Sylas,"Сайлас"=>ChampionId::Sylas,"解脱者"=>ChampionId::Sylas,"赛勒斯"=>ChampionId::Sylas,"賽勒斯"=>ChampionId::Sylas,"سيندرا"=>ChampionId::Syndra,"Syndra"=>ChampionId::Syndra,"Σίντρα"=>ChampionId::Syndra,"シンドラ"=>ChampionId::Syndra,"신드라"=>ChampionId::Syndra,"Синдра"=>ChampionId::Syndra,"暗黑元首"=>ChampionId::Syndra,"星朵拉"=>ChampionId::Syndra,"تام كينش"=>ChampionId::TahmKench,"Tahm Kench"=>ChampionId::TahmKench,"Ταμ Κεντς"=>ChampionId::TahmKench,"タム・ケンチ"=>ChampionId::TahmKench,"탐 켄치"=>ChampionId::TahmKench,"Таам Кенч"=>ChampionId::TahmKench,"河流之王"=>ChampionId::TahmKench,"贪啃奇"=>ChampionId::TahmKench,"貪啃奇"=>ChampionId::TahmKench,"تاليا"=>ChampionId::Taliyah,"Taliyah"=>ChampionId::Taliyah,"Τάλια"=>ChampionId::Taliyah,"タリヤ"=>ChampionId::Taliyah,"탈리야"=>ChampionId::Taliyah,"Талия"=>ChampionId::Taliyah,"岩雀"=>ChampionId::Taliyah,"塔莉雅"=>ChampionId::Taliyah,"تالون"=>ChampionId::Talon,"Talon"=>ChampionId::Talon,"Τάλον"=>ChampionId::Talon,"タロン"=>ChampionId::Talon,"탈론"=>ChampionId::Talon,"Талон"=>ChampionId::Talon,"刀锋之影"=>ChampionId::Talon,"塔隆"=>ChampionId::Talon,"تاريك"=>ChampionId::Taric,"Taric"=>ChampionId::Taric,"Τάρικ"=>ChampionId::Taric,"タリック"=>ChampionId::Taric,"타릭"=>ChampionId::Taric,"Тарик"=>ChampionId::Taric,"瓦洛兰之盾"=>ChampionId::Taric,"塔里克"=>ChampionId::Taric,"تيمو"=>ChampionId::Teemo,"Teemo"=>ChampionId::Teemo,"Τίμο"=>ChampionId::Teemo,"ティーモ"=>ChampionId::Teemo,"티모"=>ChampionId::Teemo,"Тимо"=>ChampionId::Teemo,"迅捷斥候"=>ChampionId::Teemo,"提摩"=>ChampionId::Teemo,"ثريش"=>ChampionId::Thresh,"Thresh"=>ChampionId::Thresh,"Θρες"=>ChampionId::Thresh,"スレッシュ"=>ChampionId::Thresh,"쓰레쉬"=>ChampionId::Thresh,"Треш"=>ChampionId::Thresh,"魂锁典狱长"=>ChampionId::Thresh,"瑟雷西"=>ChampionId::Thresh,"تريستانا"=>ChampionId::Tristana,"Tristana"=>ChampionId::Tristana,"Τριστάνα"=>ChampionId::Tristana,"トリスターナ"=>ChampionId::Tristana,"트리스타나"=>ChampionId::Tristana,"Тристана"=>ChampionId::Tristana,"麦林炮手"=>ChampionId::Tristana,"崔丝塔娜"=>ChampionId::Tristana,"崔絲塔娜"=>ChampionId::Tristana,"ترندل"=>ChampionId::Trundle,"Trundle"=>ChampionId::Trundle,"Τραντλ"=>ChampionId::Trundle,"トランドル"=>ChampionId::Trundle,"트런들"=>ChampionId::Trundle,"Трандл"=>ChampionId::Trundle,"巨魔之王"=>ChampionId::Trundle,"特朗德"=>ChampionId::Trundle,"تريندامير"=>ChampionId::Tryndamere,"Tryndamere"=>ChampionId::Tryndamere,"Τρίνταμερ"=>ChampionId::Tryndamere,"トリンダメア"=>ChampionId::Tryndamere,"트린다미어"=>ChampionId::Tryndamere,"Триндамир"=>ChampionId::Tryndamere,"蛮族之王"=>ChampionId::Tryndamere,"泰达米尔"=>ChampionId::Tryndamere,"泰達米爾"=>ChampionId::Tryndamere,"تويستد فايت"=>ChampionId::TwistedFate,"Twisted Fate"=>ChampionId::TwistedFate,"Τουίστεντ Φέιτ"=>ChampionId::TwistedFate,"ツイステッド・フェイト"=>ChampionId::TwistedFate,"트위스티드 페이트"=>ChampionId::TwistedFate,"Твистед Фэйт"=>ChampionId::TwistedFate,"卡牌大师"=>ChampionId::TwistedFate,"逆命"=>ChampionId::TwistedFate,"تويتش"=>ChampionId::Twitch,"Twitch"=>ChampionId::Twitch,"Τουίτς"=>ChampionId::Twitch,"トゥイッチ"=>ChampionId::Twitch,"트위치"=>ChampionId::Twitch,"Твич"=>ChampionId::Twitch,"瘟疫之源"=>ChampionId::Twitch,"图奇"=>ChampionId::Twitch,"圖奇"=>ChampionId::Twitch,"أودير"=>ChampionId::Udyr,"Udyr"=>ChampionId::Udyr,"Ούντιρ"=>ChampionId::Udyr,"ウディア"=>ChampionId::Udyr,"우디르"=>ChampionId::Udyr,"Удир"=>ChampionId::Udyr,"兽灵行者"=>ChampionId::Udyr,"乌迪尔"=>ChampionId::Udyr,"烏迪爾"=>ChampionId::Udyr,"أورغوت"=>ChampionId::Urgot,"Urgot"=>ChampionId::Urgot,"Ούργκοτ"=>ChampionId::Urgot,"アーゴット"=>ChampionId::Urgot,"우르곳"=>ChampionId::Urgot,"Ургот"=>ChampionId::Urgot,"无畏战车"=>ChampionId::Urgot,"乌尔加特"=>ChampionId::Urgot,"烏爾加特"=>ChampionId::Urgot,"فاروس"=>ChampionId::Varus,"Varus"=>ChampionId::Varus,"Βάρους"=>ChampionId::Varus,"ヴァルス"=>ChampionId::Varus,"바루스"=>ChampionId::Varus,"Варус"=>ChampionId::Varus,"惩戒之箭"=>ChampionId::Varus,"法洛士"=>ChampionId::Varus,"فاين"=>ChampionId::Vayne,"Vayne"=>ChampionId::Vayne,"Βέιν"=>ChampionId::Vayne,"ヴェイン"=>ChampionId::Vayne,"베인"=>ChampionId::Vayne,"Вейн"=>ChampionId::Vayne,"暗夜猎手"=>ChampionId::Vayne,"汎"=>ChampionId::Vayne,"فيغار"=>ChampionId::Veigar,"Veigar"=>ChampionId::Veigar,"Βέιγκαρ"=>ChampionId::Veigar,"ベイガー"=>ChampionId::Veigar,"베이가"=>ChampionId::Veigar,"Вейгар"=>ChampionId::Veigar,"邪恶小法师"=>ChampionId::Veigar,"维迦"=>ChampionId::Veigar,"維迦"=>ChampionId::Veigar,"فيلكوز"=>ChampionId::Velkoz,"Vel'Koz"=>ChampionId::Velkoz,"Βελ'Κοζ"=>ChampionId::Velkoz,"ヴェル＝コズ"=>ChampionId::Velkoz,"벨코즈"=>ChampionId::Velkoz,"Вел'Коз"=>ChampionId::Velkoz,"虚空之眼"=>ChampionId::Velkoz,"威寇兹"=>ChampionId::Velkoz,"威寇茲"=>ChampionId::Velkoz,"فيكس"=>ChampionId::Vex,"Vex"=>ChampionId::Vex,"Βεξ"=>ChampionId::Vex,"ヴェックス"=>ChampionId::Vex,"벡스"=>ChampionId::Vex,"Векс"=>ChampionId::Vex,"愁云使者"=>ChampionId::Vex,"薇可丝"=>ChampionId::Vex,"薇可絲"=>ChampionId::Vex,"فاي"=>ChampionId::Vi,"Vi"=>ChampionId::Vi,"Βάι"=>ChampionId::Vi,"ヴァイ"=>ChampionId::Vi,"바이"=>ChampionId::Vi,"Вай"=>ChampionId::Vi,"皮城执法官"=>ChampionId::Vi,"菲艾"=>ChampionId::Vi,"فييغو"=>ChampionId::Viego,"Viego"=>ChampionId::Viego,"Βιέγκο"=>ChampionId::Viego,"ヴィエゴ"=>ChampionId::Viego,"비에고"=>ChampionId::Viego,"Виего"=>ChampionId::Viego,"破败之王"=>ChampionId::Viego,"维尔戈"=>ChampionId::Viego,"維爾戈"=>ChampionId::Viego,"فيكتور"=>ChampionId::Viktor,"Viktor"=>ChampionId::Viktor,"Βίκτορ"=>ChampionId::Viktor,"ビクター"=>ChampionId::Viktor,"빅토르"=>ChampionId::Viktor,"Виктор"=>ChampionId::Viktor,"奥术先驱"=>ChampionId::Viktor,"维克特"=>ChampionId::Viktor,"維克特"=>ChampionId::Viktor,"فلاديمير"=>ChampionId::Vladimir,"Vladimir"=>ChampionId::Vladimir,"Βλάντιμιρ"=>ChampionId::Vladimir,"ブラッドミア"=>ChampionId::Vladimir,"블라디미르"=>ChampionId::Vladimir,"Владимир"=>ChampionId::Vladimir,"猩红收割者"=>ChampionId::Vladimir,"弗拉迪米尔"=>ChampionId::Vladimir,"弗拉迪米爾"=>ChampionId::Vladimir,"فوليبير"=>ChampionId::Volibear,"Volibear"=>ChampionId::Volibear,"Βόλιμπεαρ"=>ChampionId::Volibear,"ボリベア"=>ChampionId::Volibear,"볼리베어"=>ChampionId::Volibear,"Волибир"=>ChampionId::Volibear,"不灭狂雷"=>ChampionId::Volibear,"弗力贝尔"=>ChampionId::Volibear,"弗力貝爾"=>ChampionId::Volibear,"وارويك"=>ChampionId::Warwick,"Warwick"=>ChampionId::Warwick,"Γουόργουικ"=>ChampionId::Warwick,"ワーウィック"=>ChampionId::Warwick,"워윅"=>ChampionId::Warwick,"Варвик"=>ChampionId::Warwick,"祖安怒兽"=>ChampionId::Warwick,"沃维克"=>ChampionId::Warwick,"沃維克"=>ChampionId::Warwick,"زايا"=>ChampionId::Xayah,"Xayah"=>ChampionId::Xayah,"Ζάια"=>ChampionId::Xayah,"ザヤ"=>ChampionId::Xayah,"자야"=>ChampionId::Xayah,"Шая"=>ChampionId::Xayah,"逆羽"=>ChampionId::Xayah,"刹雅"=>ChampionId::Xayah,"剎雅"=>ChampionId::Xayah,"زيراث"=>ChampionId::Xerath,"Xerath"=>ChampionId::Xerath,"Ζέραθ"=>ChampionId::Xerath,"ゼラス"=>ChampionId::Xerath,"제라스"=>ChampionId::Xerath,"Зерат"=>ChampionId::Xerath,"远古巫灵"=>ChampionId::Xerath,"齐勒斯"=>ChampionId::Xerath,"齊勒斯"=>ChampionId::Xerath,"شين جاو"=>ChampionId::XinZhao,"Xin Zhao"=>ChampionId::XinZhao,"Ζιν Ζάο"=>ChampionId::XinZhao,"シン・ジャオ"=>ChampionId::XinZhao,"신 짜오"=>ChampionId::XinZhao,"Ксин Жао"=>ChampionId::XinZhao,"德邦总管"=>ChampionId::XinZhao,"赵信"=>ChampionId::XinZhao,"趙信"=>ChampionId::XinZhao,"ياسو"=>ChampionId::Yasuo,"Yasuo"=>ChampionId::Yasuo,"Υασούο"=>ChampionId::Yasuo,"ヤスオ"=>ChampionId::Yasuo,"야스오"=>ChampionId::Yasuo,"Ясуо"=>ChampionId::Yasuo,"疾风剑豪"=>ChampionId::Yasuo,"犽宿"=>ChampionId::Yasuo,"يوني"=>ChampionId::Yone,"Yone"=>ChampionId::Yone,"Γιόνε"=>ChampionId::Yone,"ヨネ"=>ChampionId::Yone,"요네"=>ChampionId::Yone,"Ёнэ"=>ChampionId::Yone,"封魔剑魂"=>ChampionId::Yone,"犽凝"=>ChampionId::Yone,"يوريك"=>ChampionId::Yorick,"Yorick"=>ChampionId::Yorick,"Γιόρικ"=>ChampionId::Yorick,"ヨリック"=>ChampionId::Yorick,"요릭"=>ChampionId::Yorick,"Йорик"=>ChampionId::Yorick,"牧魂人"=>ChampionId::Yorick,"约瑞科"=>ChampionId::Yorick,"約瑞科"=>ChampionId::Yorick,"يونارا"=>ChampionId::Yunara,"Yunara"=>ChampionId::Yunara,"Γιουνάρα"=>ChampionId::Yunara,"ユナラ"=>ChampionId::Yunara,"유나라"=>ChampionId::Yunara,"Юнара"=>ChampionId::Yunara,"不破之誓"=>ChampionId::Yunara,"尤娜拉"=>ChampionId::Yunara,"يومي"=>ChampionId::Yuumi,"Yuumi"=>ChampionId::Yuumi,"Γιούμι"=>ChampionId::Yuumi,"ユーミ"=>ChampionId::Yuumi,"유미"=>ChampionId::Yuumi,"Юми"=>ChampionId::Yuumi,"魔法猫咪"=>ChampionId::Yuumi,"悠咪"=>ChampionId::Yuumi,"زاك"=>ChampionId::Zac,"Zac"=>ChampionId::Zac,"Ζακ"=>ChampionId::Zac,"ザック"=>ChampionId::Zac,"자크"=>ChampionId::Zac,"Зак"=>ChampionId::Zac,"生化魔人"=>ChampionId::Zac,"札克"=>ChampionId::Zac,"زيد"=>ChampionId::Zed,"Zed"=>ChampionId::Zed,"Ζεντ"=>ChampionId::Zed,"ゼド"=>ChampionId::Zed,"제드"=>ChampionId::Zed,"Зед"=>ChampionId::Zed,"影流之主"=>ChampionId::Zed,"劫"=>ChampionId::Zed,"زيري"=>ChampionId::Zeri,"Zeri"=>ChampionId::Zeri,"Ζέρι"=>ChampionId::Zeri,"ゼリ"=>ChampionId::Zeri,"제리"=>ChampionId::Zeri,"Зери"=>ChampionId::Zeri,"祖安花火"=>ChampionId::Zeri,"婕莉"=>ChampionId::Zeri,"زيغز"=>ChampionId::Ziggs,"Ziggs"=>ChampionId::Ziggs,"Ζιγκζ"=>ChampionId::Ziggs,"ジグス"=>ChampionId::Ziggs,"직스"=>ChampionId::Ziggs,"Зиггс"=>ChampionId::Ziggs,"爆破鬼才"=>ChampionId::Ziggs,"希格斯"=>ChampionId::Ziggs,"زيليان"=>ChampionId::Zilean,"Zilean"=>ChampionId::Zilean,"Ζίλεαν"=>ChampionId::Zilean,"ジリアン"=>ChampionId::Zilean,"질리언"=>ChampionId::Zilean,"Зилеан"=>ChampionId::Zilean,"时光守护者"=>ChampionId::Zilean,"极灵"=>ChampionId::Zilean,"極靈"=>ChampionId::Zilean,"زوي"=>ChampionId::Zoe,"Zoe"=>ChampionId::Zoe,"Ζόη"=>ChampionId::Zoe,"Zoé"=>ChampionId::Zoe,"ゾーイ"=>ChampionId::Zoe,"조이"=>ChampionId::Zoe,"Зои"=>ChampionId::Zoe,"暮光星灵"=>ChampionId::Zoe,"柔依"=>ChampionId::Zoe,"زايرا"=>ChampionId::Zyra,"Zyra"=>ChampionId::Zyra,"Ζάιρα"=>ChampionId::Zyra,"ザイラ"=>ChampionId::Zyra,"자이라"=>ChampionId::Zyra,"Зайра"=>ChampionId::Zyra,"荆棘之兴"=>ChampionId::Zyra,"枷萝"=>ChampionId::Zyra,"枷蘿"=>ChampionId::Zyra};pub static INTERNAL_CHAMPIONS:[&CachedChampion;171]=[&AATROX,&AHRI,&AKALI,&AKSHAN,&ALISTAR,&AMBESSA,&AMUMU,&ANIVIA,&ANNIE,&APHELIOS,&ASHE,&AURELIONSOL,&AURORA,&AZIR,&BARD,&BELVETH,&BLITZCRANK,&BRAND,&BRAUM,&BRIAR,&CAITLYN,&CAMILLE,&CASSIOPEIA,&CHOGATH,&CORKI,&DARIUS,&DIANA,&DRMUNDO,&DRAVEN,&EKKO,&ELISE,&EVELYNN,&EZREAL,&FIDDLESTICKS,&FIORA,&FIZZ,&GALIO,&GANGPLANK,&GAREN,&GNAR,&GRAGAS,&GRAVES,&GWEN,&HECARIM,&HEIMERDINGER,&HWEI,&ILLAOI,&IRELIA,&IVERN,&JANNA,&JARVANIV,&JAX,&JAYCE,&JHIN,&JINX,&KSANTE,&KAISA,&KALISTA,&KARMA,&KARTHUS,&KASSADIN,&KATARINA,&KAYLE,&KAYN,&KENNEN,&KHAZIX,&KINDRED,&KLED,&KOGMAW,&LEBLANC,&LEESIN,&LEONA,&LILLIA,&LISSANDRA,&LUCIAN,&LULU,&LUX,&MALPHITE,&MALZAHAR,&MAOKAI,&MASTERYI,&MEL,&MILIO,&MISSFORTUNE,&MONKEYKING,&MORDEKAISER,&MORGANA,&NAAFIRI,&NAMI,&NASUS,&NAUTILUS,&NEEKO,&NIDALEE,&NILAH,&NOCTURNE,&NUNU,&OLAF,&ORIANNA,&ORNN,&PANTHEON,&POPPY,&PYKE,&QIYANA,&QUINN,&RAKAN,&RAMMUS,&REKSAI,&RELL,&RENATA,&RENEKTON,&RENGAR,&RIVEN,&RUMBLE,&RYZE,&SAMIRA,&SEJUANI,&SENNA,&SERAPHINE,&SETT,&SHACO,&SHEN,&SHYVANA,&SINGED,&SION,&SIVIR,&SKARNER,&SMOLDER,&SONA,&SORAKA,&SWAIN,&SYLAS,&SYNDRA,&TAHMKENCH,&TALIYAH,&TALON,&TARIC,&TEEMO,&THRESH,&TRISTANA,&TRUNDLE,&TRYNDAMERE,&TWISTEDFATE,&TWITCH,&UDYR,&URGOT,&VARUS,&VAYNE,&VEIGAR,&VELKOZ,&VEX,&VI,&VIEGO,&VIKTOR,&VLADIMIR,&VOLIBEAR,&WARWICK,&XAYAH,&XERATH,&XINZHAO,&YASUO,&YONE,&YORICK,&YUNARA,&YUUMI,&ZAC,&ZED,&ZERI,&ZIGGS,&ZILEAN,&ZOE,&ZYRA,];pub static BASIC_ATTACK: DamageExpression = DamageExpression {
    attributes: Attrs::OnhitMin,
    damage_type: DamageType::Physical,
    minimum_damage: |ctx| ctx.ad,
    maximum_damage: zero,
};pub static CRITICAL_STRIKE: DamageExpression = DamageExpression {
    attributes: Attrs::OnhitMax,
    damage_type: DamageType::Physical,
    minimum_damage: |ctx| {
        ctx.ad * ctx.crit_damage / 100.0
    },
    maximum_damage: zero,
};