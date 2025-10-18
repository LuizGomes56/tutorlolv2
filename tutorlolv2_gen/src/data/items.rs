use super::*;pub static INTERNAL_ITEMS:[&CachedItem;314]=[&ABYSSAL_MASK_8020,&AEGIS_OF_THE_LEGION_3105,&AETHER_WISP_3113,&AMPLIFYING_TOME_1052,&ANATHEMAS_CHAINS_228001,&ANTI_TOWER_SOCKS_1508,&ARCANE_SWEEPER_TRINKET_3348,&ARCHANGELS_STAFF_3003,&ARDENT_CENSER_3504,&ARMORED_ADVANCE_3174,&ATMAS_RECKONING_223039,&AXIOM_ARC_6696,&B_F_SWORD_1038,&BAMIS_CINDER_6660,&BANDLEGLASS_MIRROR_4642,&BANSHEES_VEIL_3102,&BASE_TURRET_REINFORCED_ARMOR_TURRET_ITEM_1506,&BERSERKERS_GREAVES_3006,&BLACK_CLEAVER_3071,&BLACK_HOLE_GAUNTLET_447122,&BLACK_SPEAR_3599,&BLACKFIRE_TORCH_2503,&BLADE_OF_THE_RUINED_KING_3153,&BLASTING_WAND_1026,&BLIGHTING_JEWEL_4630,&BLOODLETTERS_CURSE_8010,&BLOODSONG_3877,&BLOODTHIRSTER_3072,&BOOTS_1001,&BOOTS_OF_SWIFTNESS_3009,&BOUNTY_OF_WORLDS_3867,&BRAMBLE_VEST_3076,&CAPPA_JUICE_2141,&CATALYST_OF_AEONS_3803,&CAULFIELDS_WARHAMMER_3133,&CELESTIAL_OPPOSITION_3869,&CHAIN_VEST_1031,&CHAINLACED_CRUSHERS_3173,&CHEMPUNK_CHAINSWORD_6609,&CLOAK_OF_AGILITY_1018,&CLOAK_OF_STARRY_NIGHT_443059,&CLOTH_ARMOR_1029,&CONTROL_WARD_2055,&COSMIC_DRIVE_4629,&CRIMSON_LUCIDITY_3171,&CROWN_OF_THE_SHATTERED_QUEEN_444644,&CRUELTY_447109,&CRYPTBLOOM_3137,&CRYSTALLINE_BRACER_3801,&CULL_1083,&DAGGER_1042,&DARK_SEAL_1082,&DARKSTEEL_TALONS_443054,&DAWNCORE_6621,&DEAD_MANS_PLATE_3742,&DEATHS_DANCE_6333,&DEATHS_DAUGHTER_3902,&DECAPITATOR_447107,&DEMON_KINGS_CROWN_443056,&DEMONIC_EMBRACE_444637,&DETONATION_ORB_447113,&DIAMOND_TIPPED_SPEAR_447120,&DIVINE_SUNDERER_446632,&DORANS_BLADE_1055,&DORANS_RING_1056,&DORANS_SHIELD_1054,&DRAGONHEART_447106,&DREAM_MAKER_3870,&DUSKBLADE_OF_DRAKTHARR_446691,&ECHOES_OF_HELIA_6620,&ECLIPSE_6692,&EDGE_OF_NIGHT_3814,&ELEISAS_MIRACLE_443063,&ELIXIR_OF_AVARICE_2151,&ELIXIR_OF_FORCE_2152,&ELIXIR_OF_IRON_2138,&ELIXIR_OF_SKILL_2150,&ELIXIR_OF_SORCERY_2139,&ELIXIR_OF_WRATH_2140,&EMPYREAN_PROMISE_447105,&ENHANCED_LUCKY_DICE_2146,&ESSENCE_REAVER_3508,&EVERFROST_446656,&EXECUTIONERS_CALLING_3123,&EXPERIMENTAL_HEXPLATE_3073,&EYE_OF_THE_HERALD_3513,&FAERIE_CHARM_1004,&FARSIGHT_ALTERATION_3363,&FATED_ASHES_2508,&FIENDISH_CODEX_3108,&FIMBULWINTER_3121,&FIRE_AT_WILL_3901,&FLESHEATER_447112,&FORBIDDEN_IDOL_3114,&FORCE_OF_ENTROPY_443061,&FORCE_OF_NATURE_4401,&FOREVER_FORWARD_3176,&FROZEN_HEART_3110,&FULMINATION_443055,&GALEFORCE_446671,&GAMBLERS_BLADE_447101,&GARGOYLE_STONEPLATE_443193,&GHOSTCRAWLERS_223005,&GIANTS_BELT_1011,&GLACIAL_BUCKLER_3024,&GLOWING_MOTE_2022,&GOREDRINKER_226630,&GUARDIAN_ANGEL_3026,&GUARDIANS_AMULET_2049,&GUARDIANS_BLADE_3177,&GUARDIANS_DIRK_223185,&GUARDIANS_HAMMER_3184,&GUARDIANS_HORN_2051,&GUARDIANS_ORB_3112,&GUARDIANS_SHROUD_2050,&GUINSOOS_RAGEBLADE_3124,&GUSTO_1509,&GUSTWALKER_HATCHLING_1102,&HAMSTRINGER_443069,&HAUNTING_GUISE_3147,&HEALTH_POTION_2003,&HEARTHBOUND_AXE_3051,&HEARTSTEEL_3084,&HELLFIRE_HATCHET_4017,&HEMOMANCERS_HELM_447103,&HEXBOLT_COMPANION_443081,&HEXDRINKER_3155,&HEXTECH_ALTERNATOR_3145,&HEXTECH_GUNBLADE_223146,&HEXTECH_ROCKETBELT_3152,&HOLLOW_RADIANCE_6664,&HORIZON_FOCUS_4628,&HUBRIS_6697,&HULLBREAKER_3181,&ICEBORN_GAUNTLET_6662,&IMMORTAL_SHIELDBOW_6673,&IMPERIAL_MANDATE_4005,&INFINITY_EDGE_3031,&INNERVATING_LOCKET_447104,&IONIAN_BOOTS_OF_LUCIDITY_3158,&JAK_SHO_THE_PROTEAN_6665,&JUICE_OF_HASTE_2144,&JUICE_OF_POWER_2142,&JUICE_OF_VITALITY_2143,&KAENIC_ROOKERN_2504,&KINDLEGEM_3067,&KINKOU_JITTE_447116,&KNIGHTS_VOW_3109,&KRAKEN_SLAYER_6672,&LANE_SWAP_DETECTOR_1501,&LAST_WHISPER_3035,&LEGENDARY_ASSASSIN_ITEM_220003,&LEGENDARY_FIGHTER_ITEM_220001,&LEGENDARY_MAGE_ITEM_220004,&LEGENDARY_MARKSMAN_ITEM_220002,&LEGENDARY_SUPPORT_ITEM_220006,&LEGENDARY_TANK_ITEM_220005,&LIANDRYS_TORMENT_6653,&LICH_BANE_3100,&LIFELINE_4003,&LIGHTNING_ROD_447119,&LOCKET_OF_THE_IRON_SOLARI_3190,&LONG_SWORD_1036,&LORD_DOMINIKS_REGARDS_3036,&LOST_CHAPTER_3802,&LUCKY_DICE_2145,&LUDENS_COMPANION_6655,&MALIGNANCE_3118,&MANAMUNE_3004,&MAW_OF_MALMORTIUS_3156,&MEJAIS_SOULSTEALER_3041,&MERCURIAL_SCIMITAR_3139,&MERCURYS_TREADS_3111,&MIKAELS_BLESSING_3222,&MIRAGE_BLADE_447100,&MOONFLAIR_SPELLBLADE_447110,&MOONSTONE_RENEWER_6617,&MORELLONOMICON_3165,&MORTAL_REMINDER_3033,&MOSSTOMPER_SEEDLING_1103,&MURAMANA_3042,&NASHORS_TOOTH_3115,&NAVORI_FLICKERBLADE_6675,&NEEDLESSLY_LARGE_ROD_1058,&NEGATRON_CLOAK_1057,&NIGHT_HARVESTER_444636,&NOONQUIVER_6670,&NULL_MAGIC_MANTLE_1033,&OBLIVION_ORB_3916,&OHMWRECKER_TURRET_ITEM_1500,&OPPORTUNITY_6701,&ORACLE_LENS_3364,&OVERCHARGED_1507,&OVERLORDS_BLOODMAIL_2501,&PERPLEXITY_4015,&PHAGE_3044,&PHANTOM_DANCER_3046,&PHREAKISH_GUSTO_1510,&PICKAXE_1037,&PLATED_STEELCAPS_3047,&PORO_SNAX_2052,&PRISMATIC_ITEM_220007,&PROFANE_HYDRA_6698,&PROWLERS_CLAW_446693,&PUPPETEER_447123,&PYROMANCERS_CLOAK_447118,&QUICKSILVER_SASH_3140,&RABADONS_DEATHCAP_3089,&RADIANT_VIRTUE_446667,&RAISE_MORALE_3903,&RANDUINS_OMEN_3143,&RAPID_FIRECANNON_3094,&RAVENOUS_HYDRA_3074,&REALITY_FRACTURE_447102,&REAPERS_TOLL_443090,&RECTRIX_6690,&RECURVE_BOW_1043,&REDEMPTION_3107,&REFILLABLE_POTION_2031,&REGICIDE_447115,&REINFORCED_ARMOR_TURRET_ITEM_1502,&REJUVENATION_BEAD_1006,&REVERBERATION_447114,&RIFTMAKER_4633,&RITE_OF_RUIN_3430,&ROD_OF_AGES_6657,&RUBY_CRYSTAL_1028,&RUNAANS_HURRICANE_3085,&RUNECARVER_447108,&RUNIC_COMPASS_3866,&RYLAIS_CRYSTAL_SCEPTER_3116,&SANGUINE_GIFT_443062,&SAPPHIRE_CRYSTAL_1027,&SCARECROW_EFFIGY_3330,&SCORCHCLAW_PUP_1101,&SCOUTS_SLINGSHOT_3144,&SEEKERS_ARMGUARD_2420,&SERAPHS_EMBRACE_3040,&SERPENTS_FANG_6695,&SERRATED_DIRK_3134,&SERYLDAS_GRUDGE_6694,&SHADOWFLAME_4645,&SHATTERED_ARMGUARD_2421,&SHEEN_3057,&SHIELD_OF_MOLTEN_STONE_443058,&SHURELYAS_BATTLESONG_2065,&SLIGHTLY_MAGICAL_BOOTS_2422,&SOLSTICE_SLEIGH_3876,&SORCERERS_SHOES_3020,&SPEAR_OF_SHOJIN_3161,&SPECTRAL_CUTLASS_224004,&SPECTRES_COWL_3211,&SPELLSLINGERS_SHOES_3175,&SPIRIT_VISAGE_3065,&STAFF_OF_FLOWING_WATER_6616,&STAT_BONUS_220000,&STATIKK_SHIV_3087,&STEALTH_WARD_3340,&STEEL_SIGIL_2019,&STERAKS_GAGE_3053,&STORMRAZOR_223095,&STORMSURGE_4646,&STRIDEBREAKER_6631,&SUNDERED_SKY_6610,&SUNFIRE_AEGIS_3068,&SUPER_MECH_ARMOR_1511,&SUPER_MECH_POWER_FIELD_1512,&SWIFTMARCH_3170,&SWORD_OF_BLOSSOMING_DAWN_4011,&SWORD_OF_THE_DIVINE_443060,&SYMBIOTIC_SOLES_3010,&SYNCHRONIZED_SOULS_3013,&TALISMAN_OF_ASCENSION_443064,&TEAR_OF_THE_GODDESS_3070,&TERMINUS_3302,&THE_BRUTALIZER_2020,&THE_COLLECTOR_6676,&THE_GOLDEN_SPATULA_224403,&THORNMAIL_3075,&TIAMAT_3077,&TITANIC_HYDRA_3748,&TOTAL_BISCUIT_OF_EVERLASTING_WILL_2010,&TRAILBLAZER_3002,&TRINITY_FORCE_3078,&TUNNELER_2021,&TURBO_CHEMTANK_443079,&TURRET_PLATING_1515,&TWILIGHTS_EDGE_447121,&TWIN_MASK_443080,&UMBRAL_GLAIVE_3179,&UNENDING_DESPAIR_2502,&VAMPIRIC_SCEPTER_1053,&VERDANT_BARRIER_4632,&VIGILANT_WARDSTONE_4643,&VOID_STAFF_3135,&VOLTAIC_CYCLOSWORD_6699,&WARDENS_EYE_1503,&WARDENS_MAIL_3082,&WARMOGS_ARMOR_3083,&WATCHFUL_WARDSTONE_4638,&WINGED_MOONPLATE_3066,&WINTERS_APPROACH_3119,&WITS_END_3091,&WOOGLETS_WITCHCAP_228002,&WORDLESS_PROMISE_4016,&WORLD_ATLAS_3865,&YOUMUUS_GHOSTBLADE_3142,&YOUR_CUT_3400,&YUN_TAL_WILDARROWS_3032,&ZAZ_ZAKS_REALMSPIKE_3871,&ZEAL_3086,&ZEKES_CONVERGENCE_3050,&ZEPHYR_3172,&ZHONYAS_HOURGLASS_3157,];#[derive(Debug,Copy,Clone,Ord,Eq,PartialOrd,PartialEq,Decode,Encode)]#[repr(u16)]
        pub enum ItemId {AbyssalMask,AegisoftheLegion,AetherWisp,AmplifyingTome,AnathemasChains,AntiTowerSocks,ArcaneSweeperTrinket,ArchangelsStaff,ArdentCenser,ArmoredAdvance,AtmasReckoning,AxiomArc,BFSword,BamisCinder,BandleglassMirror,BansheesVeil,BaseTurretReinforcedArmorTurretItem,BerserkersGreaves,BlackCleaver,BlackHoleGauntlet,BlackSpear,BlackfireTorch,BladeoftheRuinedKing,BlastingWand,BlightingJewel,BloodlettersCurse,Bloodsong,Bloodthirster,Boots,BootsofSwiftness,BountyofWorlds,BrambleVest,CappaJuice,CatalystofAeons,CaulfieldsWarhammer,CelestialOpposition,ChainVest,ChainlacedCrushers,ChempunkChainsword,CloakofAgility,CloakofStarryNight,ClothArmor,ControlWard,CosmicDrive,CrimsonLucidity,CrownoftheShatteredQueen,Cruelty,Cryptbloom,CrystallineBracer,Cull,Dagger,DarkSeal,DarksteelTalons,Dawncore,DeadMansPlate,DeathsDance,DeathsDaughter,Decapitator,DemonKingsCrown,DemonicEmbrace,DetonationOrb,DiamondTippedSpear,DivineSunderer,DoransBlade,DoransRing,DoransShield,Dragonheart,DreamMaker,DuskbladeofDraktharr,EchoesofHelia,Eclipse,EdgeofNight,EleisasMiracle,ElixirofAvarice,ElixirofForce,ElixirofIron,ElixirofSkill,ElixirofSorcery,ElixirofWrath,EmpyreanPromise,EnhancedLuckyDice,EssenceReaver,Everfrost,ExecutionersCalling,ExperimentalHexplate,EyeoftheHerald,FaerieCharm,FarsightAlteration,FatedAshes,FiendishCodex,Fimbulwinter,FireatWill,Flesheater,ForbiddenIdol,ForceofEntropy,ForceofNature,ForeverForward,FrozenHeart,Fulmination,Galeforce,GamblersBlade,GargoyleStoneplate,Ghostcrawlers,GiantsBelt,GlacialBuckler,GlowingMote,Goredrinker,GuardianAngel,GuardiansAmulet,GuardiansBlade,GuardiansDirk,GuardiansHammer,GuardiansHorn,GuardiansOrb,GuardiansShroud,GuinsoosRageblade,Gusto,GustwalkerHatchling,Hamstringer,HauntingGuise,HealthPotion,HearthboundAxe,Heartsteel,HellfireHatchet,HemomancersHelm,HexboltCompanion,Hexdrinker,HextechAlternator,HextechGunblade,HextechRocketbelt,HollowRadiance,HorizonFocus,Hubris,Hullbreaker,IcebornGauntlet,ImmortalShieldbow,ImperialMandate,InfinityEdge,InnervatingLocket,IonianBootsofLucidity,JakShoTheProtean,JuiceofHaste,JuiceofPower,JuiceofVitality,KaenicRookern,Kindlegem,KinkouJitte,KnightsVow,KrakenSlayer,LaneSwapDetector,LastWhisper,LegendaryAssassinItem,LegendaryFighterItem,LegendaryMageItem,LegendaryMarksmanItem,LegendarySupportItem,LegendaryTankItem,LiandrysTorment,LichBane,Lifeline,LightningRod,LocketoftheIronSolari,LongSword,LordDominiksRegards,LostChapter,LuckyDice,LudensCompanion,Malignance,Manamune,MawofMalmortius,MejaisSoulstealer,MercurialScimitar,MercurysTreads,MikaelsBlessing,MirageBlade,MoonflairSpellblade,MoonstoneRenewer,Morellonomicon,MortalReminder,MosstomperSeedling,Muramana,NashorsTooth,NavoriFlickerblade,NeedlesslyLargeRod,NegatronCloak,NightHarvester,Noonquiver,NullMagicMantle,OblivionOrb,OhmwreckerTurretItem,Opportunity,OracleLens,Overcharged,OverlordsBloodmail,Perplexity,Phage,PhantomDancer,PhreakishGusto,Pickaxe,PlatedSteelcaps,PoroSnax,PrismaticItem,ProfaneHydra,ProwlersClaw,Puppeteer,PyromancersCloak,QuicksilverSash,RabadonsDeathcap,RadiantVirtue,RaiseMorale,RanduinsOmen,RapidFirecannon,RavenousHydra,RealityFracture,ReapersToll,Rectrix,RecurveBow,Redemption,RefillablePotion,Regicide,ReinforcedArmorTurretItem,RejuvenationBead,Reverberation,Riftmaker,RiteofRuin,RodofAges,RubyCrystal,RunaansHurricane,Runecarver,RunicCompass,RylaisCrystalScepter,SanguineGift,SapphireCrystal,ScarecrowEffigy,ScorchclawPup,ScoutsSlingshot,SeekersArmguard,SeraphsEmbrace,SerpentsFang,SerratedDirk,SeryldasGrudge,Shadowflame,ShatteredArmguard,Sheen,ShieldofMoltenStone,ShurelyasBattlesong,SlightlyMagicalBoots,SolsticeSleigh,SorcerersShoes,SpearofShojin,SpectralCutlass,SpectresCowl,SpellslingersShoes,SpiritVisage,StaffofFlowingWater,StatBonus,StatikkShiv,StealthWard,SteelSigil,SteraksGage,Stormrazor,Stormsurge,Stridebreaker,SunderedSky,SunfireAegis,SuperMechArmor,SuperMechPowerField,Swiftmarch,SwordofBlossomingDawn,SwordoftheDivine,SymbioticSoles,SynchronizedSouls,TalismanofAscension,TearoftheGoddess,Terminus,TheBrutalizer,TheCollector,TheGoldenSpatula,Thornmail,Tiamat,TitanicHydra,TotalBiscuitofEverlastingWill,Trailblazer,TrinityForce,Tunneler,TurboChemtank,TurretPlating,TwilightsEdge,TwinMask,UmbralGlaive,UnendingDespair,VampiricScepter,VerdantBarrier,VigilantWardstone,VoidStaff,VoltaicCyclosword,WardensEye,WardensMail,WarmogsArmor,WatchfulWardstone,WingedMoonplate,WintersApproach,WitsEnd,WoogletsWitchcap,WordlessPromise,WorldAtlas,YoumuusGhostblade,YourCut,YunTalWildarrows,ZazZaksRealmspike,Zeal,ZekesConvergence,Zephyr,ZhonyasHourglass}impl ItemId {pub const fn to_riot_id(&self)->u32{ITEM_ID_TO_RIOT_ID[*self as usize]}
        pub const fn from_riot_id(id:u32)->Self{match id {8020=>Self::AbyssalMask,3105=>Self::AegisoftheLegion,3113=>Self::AetherWisp,1052=>Self::AmplifyingTome,228001=>Self::AnathemasChains,1508=>Self::AntiTowerSocks,3348=>Self::ArcaneSweeperTrinket,3003=>Self::ArchangelsStaff,3504=>Self::ArdentCenser,3174=>Self::ArmoredAdvance,223039=>Self::AtmasReckoning,6696=>Self::AxiomArc,1038=>Self::BFSword,6660=>Self::BamisCinder,4642=>Self::BandleglassMirror,3102=>Self::BansheesVeil,1506=>Self::BaseTurretReinforcedArmorTurretItem,3006=>Self::BerserkersGreaves,3071=>Self::BlackCleaver,447122=>Self::BlackHoleGauntlet,3599=>Self::BlackSpear,2503=>Self::BlackfireTorch,3153=>Self::BladeoftheRuinedKing,1026=>Self::BlastingWand,4630=>Self::BlightingJewel,8010=>Self::BloodlettersCurse,3877=>Self::Bloodsong,3072=>Self::Bloodthirster,1001=>Self::Boots,3009=>Self::BootsofSwiftness,3867=>Self::BountyofWorlds,3076=>Self::BrambleVest,2141=>Self::CappaJuice,3803=>Self::CatalystofAeons,3133=>Self::CaulfieldsWarhammer,3869=>Self::CelestialOpposition,1031=>Self::ChainVest,3173=>Self::ChainlacedCrushers,6609=>Self::ChempunkChainsword,1018=>Self::CloakofAgility,443059=>Self::CloakofStarryNight,1029=>Self::ClothArmor,2055=>Self::ControlWard,4629=>Self::CosmicDrive,3171=>Self::CrimsonLucidity,444644=>Self::CrownoftheShatteredQueen,447109=>Self::Cruelty,3137=>Self::Cryptbloom,3801=>Self::CrystallineBracer,1083=>Self::Cull,1042=>Self::Dagger,1082=>Self::DarkSeal,443054=>Self::DarksteelTalons,6621=>Self::Dawncore,3742=>Self::DeadMansPlate,6333=>Self::DeathsDance,3902=>Self::DeathsDaughter,447107=>Self::Decapitator,443056=>Self::DemonKingsCrown,444637=>Self::DemonicEmbrace,447113=>Self::DetonationOrb,447120=>Self::DiamondTippedSpear,446632=>Self::DivineSunderer,1055=>Self::DoransBlade,1056=>Self::DoransRing,1054=>Self::DoransShield,447106=>Self::Dragonheart,3870=>Self::DreamMaker,446691=>Self::DuskbladeofDraktharr,6620=>Self::EchoesofHelia,6692=>Self::Eclipse,3814=>Self::EdgeofNight,443063=>Self::EleisasMiracle,2151=>Self::ElixirofAvarice,2152=>Self::ElixirofForce,2138=>Self::ElixirofIron,2150=>Self::ElixirofSkill,2139=>Self::ElixirofSorcery,2140=>Self::ElixirofWrath,447105=>Self::EmpyreanPromise,2146=>Self::EnhancedLuckyDice,3508=>Self::EssenceReaver,446656=>Self::Everfrost,3123=>Self::ExecutionersCalling,3073=>Self::ExperimentalHexplate,3513=>Self::EyeoftheHerald,1004=>Self::FaerieCharm,3363=>Self::FarsightAlteration,2508=>Self::FatedAshes,3108=>Self::FiendishCodex,3121=>Self::Fimbulwinter,3901=>Self::FireatWill,447112=>Self::Flesheater,3114=>Self::ForbiddenIdol,443061=>Self::ForceofEntropy,4401=>Self::ForceofNature,3176=>Self::ForeverForward,3110=>Self::FrozenHeart,443055=>Self::Fulmination,446671=>Self::Galeforce,447101=>Self::GamblersBlade,443193=>Self::GargoyleStoneplate,223005=>Self::Ghostcrawlers,1011=>Self::GiantsBelt,3024=>Self::GlacialBuckler,2022=>Self::GlowingMote,226630=>Self::Goredrinker,3026=>Self::GuardianAngel,2049=>Self::GuardiansAmulet,3177=>Self::GuardiansBlade,223185=>Self::GuardiansDirk,3184=>Self::GuardiansHammer,2051=>Self::GuardiansHorn,3112=>Self::GuardiansOrb,2050=>Self::GuardiansShroud,3124=>Self::GuinsoosRageblade,1509=>Self::Gusto,1102=>Self::GustwalkerHatchling,443069=>Self::Hamstringer,3147=>Self::HauntingGuise,2003=>Self::HealthPotion,3051=>Self::HearthboundAxe,3084=>Self::Heartsteel,4017=>Self::HellfireHatchet,447103=>Self::HemomancersHelm,443081=>Self::HexboltCompanion,3155=>Self::Hexdrinker,3145=>Self::HextechAlternator,223146=>Self::HextechGunblade,3152=>Self::HextechRocketbelt,6664=>Self::HollowRadiance,4628=>Self::HorizonFocus,6697=>Self::Hubris,3181=>Self::Hullbreaker,6662=>Self::IcebornGauntlet,6673=>Self::ImmortalShieldbow,4005=>Self::ImperialMandate,3031=>Self::InfinityEdge,447104=>Self::InnervatingLocket,3158=>Self::IonianBootsofLucidity,6665=>Self::JakShoTheProtean,2144=>Self::JuiceofHaste,2142=>Self::JuiceofPower,2143=>Self::JuiceofVitality,2504=>Self::KaenicRookern,3067=>Self::Kindlegem,447116=>Self::KinkouJitte,3109=>Self::KnightsVow,6672=>Self::KrakenSlayer,1501=>Self::LaneSwapDetector,3035=>Self::LastWhisper,220003=>Self::LegendaryAssassinItem,220001=>Self::LegendaryFighterItem,220004=>Self::LegendaryMageItem,220002=>Self::LegendaryMarksmanItem,220006=>Self::LegendarySupportItem,220005=>Self::LegendaryTankItem,6653=>Self::LiandrysTorment,3100=>Self::LichBane,4003=>Self::Lifeline,447119=>Self::LightningRod,3190=>Self::LocketoftheIronSolari,1036=>Self::LongSword,3036=>Self::LordDominiksRegards,3802=>Self::LostChapter,2145=>Self::LuckyDice,6655=>Self::LudensCompanion,3118=>Self::Malignance,3004=>Self::Manamune,3156=>Self::MawofMalmortius,3041=>Self::MejaisSoulstealer,3139=>Self::MercurialScimitar,3111=>Self::MercurysTreads,3222=>Self::MikaelsBlessing,447100=>Self::MirageBlade,447110=>Self::MoonflairSpellblade,6617=>Self::MoonstoneRenewer,3165=>Self::Morellonomicon,3033=>Self::MortalReminder,1103=>Self::MosstomperSeedling,3042=>Self::Muramana,3115=>Self::NashorsTooth,6675=>Self::NavoriFlickerblade,1058=>Self::NeedlesslyLargeRod,1057=>Self::NegatronCloak,444636=>Self::NightHarvester,6670=>Self::Noonquiver,1033=>Self::NullMagicMantle,3916=>Self::OblivionOrb,1500=>Self::OhmwreckerTurretItem,6701=>Self::Opportunity,3364=>Self::OracleLens,1507=>Self::Overcharged,2501=>Self::OverlordsBloodmail,4015=>Self::Perplexity,3044=>Self::Phage,3046=>Self::PhantomDancer,1510=>Self::PhreakishGusto,1037=>Self::Pickaxe,3047=>Self::PlatedSteelcaps,2052=>Self::PoroSnax,220007=>Self::PrismaticItem,6698=>Self::ProfaneHydra,446693=>Self::ProwlersClaw,447123=>Self::Puppeteer,447118=>Self::PyromancersCloak,3140=>Self::QuicksilverSash,3089=>Self::RabadonsDeathcap,446667=>Self::RadiantVirtue,3903=>Self::RaiseMorale,3143=>Self::RanduinsOmen,3094=>Self::RapidFirecannon,3074=>Self::RavenousHydra,447102=>Self::RealityFracture,443090=>Self::ReapersToll,6690=>Self::Rectrix,1043=>Self::RecurveBow,3107=>Self::Redemption,2031=>Self::RefillablePotion,447115=>Self::Regicide,1502=>Self::ReinforcedArmorTurretItem,1006=>Self::RejuvenationBead,447114=>Self::Reverberation,4633=>Self::Riftmaker,3430=>Self::RiteofRuin,6657=>Self::RodofAges,1028=>Self::RubyCrystal,3085=>Self::RunaansHurricane,447108=>Self::Runecarver,3866=>Self::RunicCompass,3116=>Self::RylaisCrystalScepter,443062=>Self::SanguineGift,1027=>Self::SapphireCrystal,3330=>Self::ScarecrowEffigy,1101=>Self::ScorchclawPup,3144=>Self::ScoutsSlingshot,2420=>Self::SeekersArmguard,3040=>Self::SeraphsEmbrace,6695=>Self::SerpentsFang,3134=>Self::SerratedDirk,6694=>Self::SeryldasGrudge,4645=>Self::Shadowflame,2421=>Self::ShatteredArmguard,3057=>Self::Sheen,443058=>Self::ShieldofMoltenStone,2065=>Self::ShurelyasBattlesong,2422=>Self::SlightlyMagicalBoots,3876=>Self::SolsticeSleigh,3020=>Self::SorcerersShoes,3161=>Self::SpearofShojin,224004=>Self::SpectralCutlass,3211=>Self::SpectresCowl,3175=>Self::SpellslingersShoes,3065=>Self::SpiritVisage,6616=>Self::StaffofFlowingWater,220000=>Self::StatBonus,3087=>Self::StatikkShiv,3340=>Self::StealthWard,2019=>Self::SteelSigil,3053=>Self::SteraksGage,223095=>Self::Stormrazor,4646=>Self::Stormsurge,6631=>Self::Stridebreaker,6610=>Self::SunderedSky,3068=>Self::SunfireAegis,1511=>Self::SuperMechArmor,1512=>Self::SuperMechPowerField,3170=>Self::Swiftmarch,4011=>Self::SwordofBlossomingDawn,443060=>Self::SwordoftheDivine,3010=>Self::SymbioticSoles,3013=>Self::SynchronizedSouls,443064=>Self::TalismanofAscension,3070=>Self::TearoftheGoddess,3302=>Self::Terminus,2020=>Self::TheBrutalizer,6676=>Self::TheCollector,224403=>Self::TheGoldenSpatula,3075=>Self::Thornmail,3077=>Self::Tiamat,3748=>Self::TitanicHydra,2010=>Self::TotalBiscuitofEverlastingWill,3002=>Self::Trailblazer,3078=>Self::TrinityForce,2021=>Self::Tunneler,443079=>Self::TurboChemtank,1515=>Self::TurretPlating,447121=>Self::TwilightsEdge,443080=>Self::TwinMask,3179=>Self::UmbralGlaive,2502=>Self::UnendingDespair,1053=>Self::VampiricScepter,4632=>Self::VerdantBarrier,4643=>Self::VigilantWardstone,3135=>Self::VoidStaff,6699=>Self::VoltaicCyclosword,1503=>Self::WardensEye,3082=>Self::WardensMail,3083=>Self::WarmogsArmor,4638=>Self::WatchfulWardstone,3066=>Self::WingedMoonplate,3119=>Self::WintersApproach,3091=>Self::WitsEnd,228002=>Self::WoogletsWitchcap,4016=>Self::WordlessPromise,3865=>Self::WorldAtlas,3142=>Self::YoumuusGhostblade,3400=>Self::YourCut,3032=>Self::YunTalWildarrows,3871=>Self::ZazZaksRealmspike,3086=>Self::Zeal,3050=>Self::ZekesConvergence,3172=>Self::Zephyr,3157=>Self::ZhonyasHourglass,_=>Self::YourCut}}}pub static ITEM_ID_TO_RIOT_ID:[u32;314]=[8020,3105,3113,1052,228001,1508,3348,3003,3504,3174,223039,6696,1038,6660,4642,3102,1506,3006,3071,447122,3599,2503,3153,1026,4630,8010,3877,3072,1001,3009,3867,3076,2141,3803,3133,3869,1031,3173,6609,1018,443059,1029,2055,4629,3171,444644,447109,3137,3801,1083,1042,1082,443054,6621,3742,6333,3902,447107,443056,444637,447113,447120,446632,1055,1056,1054,447106,3870,446691,6620,6692,3814,443063,2151,2152,2138,2150,2139,2140,447105,2146,3508,446656,3123,3073,3513,1004,3363,2508,3108,3121,3901,447112,3114,443061,4401,3176,3110,443055,446671,447101,443193,223005,1011,3024,2022,226630,3026,2049,3177,223185,3184,2051,3112,2050,3124,1509,1102,443069,3147,2003,3051,3084,4017,447103,443081,3155,3145,223146,3152,6664,4628,6697,3181,6662,6673,4005,3031,447104,3158,6665,2144,2142,2143,2504,3067,447116,3109,6672,1501,3035,220003,220001,220004,220002,220006,220005,6653,3100,4003,447119,3190,1036,3036,3802,2145,6655,3118,3004,3156,3041,3139,3111,3222,447100,447110,6617,3165,3033,1103,3042,3115,6675,1058,1057,444636,6670,1033,3916,1500,6701,3364,1507,2501,4015,3044,3046,1510,1037,3047,2052,220007,6698,446693,447123,447118,3140,3089,446667,3903,3143,3094,3074,447102,443090,6690,1043,3107,2031,447115,1502,1006,447114,4633,3430,6657,1028,3085,447108,3866,3116,443062,1027,3330,1101,3144,2420,3040,6695,3134,6694,4645,2421,3057,443058,2065,2422,3876,3020,3161,224004,3211,3175,3065,6616,220000,3087,3340,2019,3053,223095,4646,6631,6610,3068,1511,1512,3170,4011,443060,3010,3013,443064,3070,3302,2020,6676,224403,3075,3077,3748,2010,3002,3078,2021,443079,1515,447121,443080,3179,2502,1053,4632,4643,3135,6699,1503,3082,3083,4638,3066,3119,3091,228002,4016,3865,3142,3400,3032,3871,3086,3050,3172,3157];pub static ABYSSAL_MASK_8020: CachedItem = CachedItem {
                    gold: 2650,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::AbyssalMask, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:350f32,lifesteal:0f32,magic_resist:45f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static AEGIS_OF_THE_LEGION_3105: CachedItem = CachedItem {
                    gold: 1100,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::AegisoftheLegion, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:25f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:25f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static AETHER_WISP_3113: CachedItem = CachedItem {
                    gold: 900,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::AetherWisp, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:30f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static AMPLIFYING_TOME_1052: CachedItem = CachedItem {
                    gold: 400,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::AmplifyingTome, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:20f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static ANATHEMAS_CHAINS_228001: CachedItem = CachedItem {
                    gold: 2500,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::AnathemasChains, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:650f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static ANTI_TOWER_SOCKS_1508: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::AntiTowerSocks, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static ARCANE_SWEEPER_TRINKET_3348: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::ArcaneSweeperTrinket, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static ARCHANGELS_STAFF_3003: CachedItem = CachedItem {
                    gold: 2900,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::ArchangelsStaff, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:70f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:600f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static ARDENT_CENSER_3504: CachedItem = CachedItem {
                    gold: 2200,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::ArdentCenser, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:45f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static ARMORED_ADVANCE_3174: CachedItem = CachedItem {
                    gold: 1700,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::ArmoredAdvance, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:40f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:50f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static ATMAS_RECKONING_223039: CachedItem = CachedItem {
                    gold: 2500,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::AtmasReckoning, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:700f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static AXIOM_ARC_6696: CachedItem = CachedItem {
                    gold: 2750,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::AxiomArc, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:55f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static B_F_SWORD_1038: CachedItem = CachedItem {
                    gold: 1300,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::BFSword, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:40f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static BAMIS_CINDER_6660: CachedItem = CachedItem {
                    gold: 900,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::BamisCinder, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:150f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static BANDLEGLASS_MIRROR_4642: CachedItem = CachedItem {
                    gold: 900,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::BandleglassMirror, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:20f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static BANSHEES_VEIL_3102: CachedItem = CachedItem {
                    gold: 3000,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::BansheesVeil, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:105f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:40f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static BASE_TURRET_REINFORCED_ARMOR_TURRET_ITEM_1506: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::BaseTurretReinforcedArmorTurretItem, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static BERSERKERS_GREAVES_3006: CachedItem = CachedItem {
                    gold: 1100,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::BerserkersGreaves, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:25f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:45f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static BLACK_CLEAVER_3071: CachedItem = CachedItem {
                    gold: 3000,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::BlackCleaver, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:40f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:400f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static BLACK_HOLE_GAUNTLET_447122: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::BlackHoleGauntlet, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:900f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static BLACK_SPEAR_3599: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::BlackSpear, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static BLACKFIRE_TORCH_2503: CachedItem = CachedItem {
                    gold: 2800,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::BlackfireTorch, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:80f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:600f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static BLADE_OF_THE_RUINED_KING_3153: CachedItem = CachedItem {
                    gold: 3200,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::BladeoftheRuinedKing, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:40f32,attack_speed:25f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static BLASTING_WAND_1026: CachedItem = CachedItem {
                    gold: 850,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::BlastingWand, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:45f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static BLIGHTING_JEWEL_4630: CachedItem = CachedItem {
                    gold: 1100,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::BlightingJewel, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:25f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:13f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static BLOODLETTERS_CURSE_8010: CachedItem = CachedItem {
                    gold: 2900,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::BloodlettersCurse, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:65f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:400f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static BLOODSONG_3877: CachedItem = CachedItem {
                    gold: 400,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Bloodsong, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:200f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static BLOODTHIRSTER_3072: CachedItem = CachedItem {
                    gold: 3400,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Bloodthirster, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:80f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static BOOTS_1001: CachedItem = CachedItem {
                    gold: 300,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Boots, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:25f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static BOOTS_OF_SWIFTNESS_3009: CachedItem = CachedItem {
                    gold: 1000,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::BootsofSwiftness, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:55f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static BOUNTY_OF_WORLDS_3867: CachedItem = CachedItem {
                    gold: 400,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::BountyofWorlds, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:200f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static BRAMBLE_VEST_3076: CachedItem = CachedItem {
                    gold: 800,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::BrambleVest, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:30f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static CAPPA_JUICE_2141: CachedItem = CachedItem {
                    gold: 300,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::CappaJuice, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static CATALYST_OF_AEONS_3803: CachedItem = CachedItem {
                    gold: 1300,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::CatalystofAeons, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:350f32,lifesteal:0f32,magic_resist:0f32,mana:375f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static CAULFIELDS_WARHAMMER_3133: CachedItem = CachedItem {
                    gold: 1050,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::CaulfieldsWarhammer, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:20f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static CELESTIAL_OPPOSITION_3869: CachedItem = CachedItem {
                    gold: 400,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::CelestialOpposition, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:200f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static CHAIN_VEST_1031: CachedItem = CachedItem {
                    gold: 800,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::ChainVest, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:40f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static CHAINLACED_CRUSHERS_3173: CachedItem = CachedItem {
                    gold: 1750,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::ChainlacedCrushers, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:35f32,mana:0f32,movespeed:50f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static CHEMPUNK_CHAINSWORD_6609: CachedItem = CachedItem {
                    gold: 3100,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::ChempunkChainsword, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:45f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:450f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static CLOAK_OF_AGILITY_1018: CachedItem = CachedItem {
                    gold: 600,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::CloakofAgility, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static CLOAK_OF_STARRY_NIGHT_443059: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::CloakofStarryNight, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:300f32,lifesteal:0f32,magic_resist:100f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static CLOTH_ARMOR_1029: CachedItem = CachedItem {
                    gold: 300,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::ClothArmor, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:15f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static CONTROL_WARD_2055: CachedItem = CachedItem {
                    gold: 75,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::ControlWard, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static COSMIC_DRIVE_4629: CachedItem = CachedItem {
                    gold: 3000,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::CosmicDrive, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:70f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:350f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static CRIMSON_LUCIDITY_3171: CachedItem = CachedItem {
                    gold: 1400,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::CrimsonLucidity, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:50f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static CROWN_OF_THE_SHATTERED_QUEEN_444644: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::CrownoftheShatteredQueen, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:85f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:300f32,lifesteal:0f32,magic_resist:0f32,mana:600f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static CRUELTY_447109: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Cruelty, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:80f32,armor:30f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:30f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static CRYPTBLOOM_3137: CachedItem = CachedItem {
                    gold: 3000,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Cryptbloom, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:75f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:30f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static CRYSTALLINE_BRACER_3801: CachedItem = CachedItem {
                    gold: 800,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::CrystallineBracer, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:200f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static CULL_1083: CachedItem = CachedItem {
                    gold: 450,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Cull, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:7f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static DAGGER_1042: CachedItem = CachedItem {
                    gold: 250,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Dagger, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:10f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static DARK_SEAL_1082: CachedItem = CachedItem {
                    gold: 350,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::DarkSeal, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:15f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:50f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static DARKSTEEL_TALONS_443054: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::DarksteelTalons, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:55f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:50f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static DAWNCORE_6621: CachedItem = CachedItem {
                    gold: 2500,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Dawncore, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:45f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static DEAD_MANS_PLATE_3742: CachedItem = CachedItem {
                    gold: 2900,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::DeadMansPlate, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:55f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:350f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static DEATHS_DANCE_6333: CachedItem = CachedItem {
                    gold: 3300,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::DeathsDance, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:50f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:60f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static DEATHS_DAUGHTER_3902: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::DeathsDaughter, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static DECAPITATOR_447107: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Decapitator, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:50f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static DEMON_KINGS_CROWN_443056: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::DemonKingsCrown, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static DEMONIC_EMBRACE_444637: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::DemonicEmbrace, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:80f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:700f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static DETONATION_ORB_447113: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::DetonationOrb, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:90f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:12f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:600f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static DIAMOND_TIPPED_SPEAR_447120: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::DiamondTippedSpear, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:30f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static DIVINE_SUNDERER_446632: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::DivineSunderer, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:55f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:350f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static DORANS_BLADE_1055: CachedItem = CachedItem {
                    gold: 450,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::DoransBlade, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:10f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:80f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static DORANS_RING_1056: CachedItem = CachedItem {
                    gold: 400,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::DoransRing, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:18f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:90f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static DORANS_SHIELD_1054: CachedItem = CachedItem {
                    gold: 450,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::DoransShield, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:110f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static DRAGONHEART_447106: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Dragonheart, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static DREAM_MAKER_3870: CachedItem = CachedItem {
                    gold: 400,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::DreamMaker, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:200f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static DUSKBLADE_OF_DRAKTHARR_446691: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::DuskbladeofDraktharr, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:50f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static ECHOES_OF_HELIA_6620: CachedItem = CachedItem {
                    gold: 2200,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::EchoesofHelia, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:35f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:200f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static ECLIPSE_6692: CachedItem = CachedItem {
                    gold: 2900,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Eclipse, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:60f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static EDGE_OF_NIGHT_3814: CachedItem = CachedItem {
                    gold: 3000,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::EdgeofNight, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:50f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:250f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static ELEISAS_MIRACLE_443063: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::EleisasMiracle, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:50f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:50f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static ELIXIR_OF_AVARICE_2151: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::ElixirofAvarice, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static ELIXIR_OF_FORCE_2152: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::ElixirofForce, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static ELIXIR_OF_IRON_2138: CachedItem = CachedItem {
                    gold: 500,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::ElixirofIron, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static ELIXIR_OF_SKILL_2150: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::ElixirofSkill, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static ELIXIR_OF_SORCERY_2139: CachedItem = CachedItem {
                    gold: 500,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::ElixirofSorcery, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static ELIXIR_OF_WRATH_2140: CachedItem = CachedItem {
                    gold: 500,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::ElixirofWrath, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static EMPYREAN_PROMISE_447105: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::EmpyreanPromise, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:70f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static ENHANCED_LUCKY_DICE_2146: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::EnhancedLuckyDice, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static ESSENCE_REAVER_3508: CachedItem = CachedItem {
                    gold: 2900,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::EssenceReaver, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:60f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static EVERFROST_446656: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Everfrost, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:100f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:250f32,lifesteal:0f32,magic_resist:0f32,mana:600f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static EXECUTIONERS_CALLING_3123: CachedItem = CachedItem {
                    gold: 800,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::ExecutionersCalling, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:15f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static EXPERIMENTAL_HEXPLATE_3073: CachedItem = CachedItem {
                    gold: 3000,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::ExperimentalHexplate, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:40f32,attack_speed:20f32,crit_chance:0f32,crit_damage:0f32,health:450f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static EYE_OF_THE_HERALD_3513: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::EyeoftheHerald, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static FAERIE_CHARM_1004: CachedItem = CachedItem {
                    gold: 200,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::FaerieCharm, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static FARSIGHT_ALTERATION_3363: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::FarsightAlteration, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static FATED_ASHES_2508: CachedItem = CachedItem {
                    gold: 900,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::FatedAshes, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:30f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static FIENDISH_CODEX_3108: CachedItem = CachedItem {
                    gold: 850,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::FiendishCodex, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:25f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static FIMBULWINTER_3121: CachedItem = CachedItem {
                    gold: 2400,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Fimbulwinter, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:550f32,lifesteal:0f32,magic_resist:0f32,mana:860f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static FIRE_AT_WILL_3901: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::FireatWill, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static FLESHEATER_447112: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Flesheater, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:500f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static FORBIDDEN_IDOL_3114: CachedItem = CachedItem {
                    gold: 600,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::ForbiddenIdol, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static FORCE_OF_ENTROPY_443061: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::ForceofEntropy, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:900f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static FORCE_OF_NATURE_4401: CachedItem = CachedItem {
                    gold: 2800,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::ForceofNature, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:400f32,lifesteal:0f32,magic_resist:55f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static FOREVER_FORWARD_3176: CachedItem = CachedItem {
                    gold: 1400,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::ForeverForward, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:55f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static FROZEN_HEART_3110: CachedItem = CachedItem {
                    gold: 2500,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::FrozenHeart, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:75f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:400f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static FULMINATION_443055: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Fulmination, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:55f32,attack_speed:45f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static GALEFORCE_446671: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Galeforce, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:65f32,attack_speed:30f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static GAMBLERS_BLADE_447101: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::GamblersBlade, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:70f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static GARGOYLE_STONEPLATE_443193: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::GargoyleStoneplate, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:65f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:65f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static GHOSTCRAWLERS_223005: CachedItem = CachedItem {
                    gold: 500,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Ghostcrawlers, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:70f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static GIANTS_BELT_1011: CachedItem = CachedItem {
                    gold: 900,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::GiantsBelt, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:350f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static GLACIAL_BUCKLER_3024: CachedItem = CachedItem {
                    gold: 900,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::GlacialBuckler, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:25f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:300f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static GLOWING_MOTE_2022: CachedItem = CachedItem {
                    gold: 250,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::GlowingMote, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static GOREDRINKER_226630: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Goredrinker, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:55f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:400f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static GUARDIAN_ANGEL_3026: CachedItem = CachedItem {
                    gold: 3200,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::GuardianAngel, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:45f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:55f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static GUARDIANS_AMULET_2049: CachedItem = CachedItem {
                    gold: 500,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::GuardiansAmulet, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:20f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static GUARDIANS_BLADE_3177: CachedItem = CachedItem {
                    gold: 950,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::GuardiansBlade, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:30f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:150f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static GUARDIANS_DIRK_223185: CachedItem = CachedItem {
                    gold: 500,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::GuardiansDirk, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:25f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static GUARDIANS_HAMMER_3184: CachedItem = CachedItem {
                    gold: 950,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::GuardiansHammer, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:25f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:150f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static GUARDIANS_HORN_2051: CachedItem = CachedItem {
                    gold: 950,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::GuardiansHorn, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:150f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static GUARDIANS_ORB_3112: CachedItem = CachedItem {
                    gold: 950,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::GuardiansOrb, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:50f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:150f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static GUARDIANS_SHROUD_2050: CachedItem = CachedItem {
                    gold: 500,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::GuardiansShroud, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:35f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:300f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static GUINSOOS_RAGEBLADE_3124: CachedItem = CachedItem {
                    gold: 3000,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::GuinsoosRageblade, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:30f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:30f32,attack_speed:25f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static GUSTO_1509: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Gusto, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static GUSTWALKER_HATCHLING_1102: CachedItem = CachedItem {
                    gold: 450,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::GustwalkerHatchling, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static HAMSTRINGER_443069: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Hamstringer, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:45f32,attack_speed:40f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static HAUNTING_GUISE_3147: CachedItem = CachedItem {
                    gold: 1300,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::HauntingGuise, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:30f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:200f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static HEALTH_POTION_2003: CachedItem = CachedItem {
                    gold: 50,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::HealthPotion, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static HEARTHBOUND_AXE_3051: CachedItem = CachedItem {
                    gold: 1200,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::HearthboundAxe, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:20f32,attack_speed:20f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static HEARTSTEEL_3084: CachedItem = CachedItem {
                    gold: 3000,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Heartsteel, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:900f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static HELLFIRE_HATCHET_4017: CachedItem = CachedItem {
                    gold: 2500,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::HellfireHatchet, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:35f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static HEMOMANCERS_HELM_447103: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::HemomancersHelm, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:60f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static HEXBOLT_COMPANION_443081: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::HexboltCompanion, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:75f32,crit_chance:0f32,crit_damage:0f32,health:500f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static HEXDRINKER_3155: CachedItem = CachedItem {
                    gold: 1300,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Hexdrinker, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:25f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:25f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static HEXTECH_ALTERNATOR_3145: CachedItem = CachedItem {
                    gold: 1100,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::HextechAlternator, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:45f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static HEXTECH_GUNBLADE_223146: CachedItem = CachedItem {
                    gold: 2500,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::HextechGunblade, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:90f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:45f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static HEXTECH_ROCKETBELT_3152: CachedItem = CachedItem {
                    gold: 2650,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::HextechRocketbelt, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:70f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:300f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static HOLLOW_RADIANCE_6664: CachedItem = CachedItem {
                    gold: 2800,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::HollowRadiance, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:400f32,lifesteal:0f32,magic_resist:40f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static HORIZON_FOCUS_4628: CachedItem = CachedItem {
                    gold: 2750,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::HorizonFocus, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:125f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static HUBRIS_6697: CachedItem = CachedItem {
                    gold: 3000,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Hubris, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:60f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static HULLBREAKER_3181: CachedItem = CachedItem {
                    gold: 3000,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Hullbreaker, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:40f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:500f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static ICEBORN_GAUNTLET_6662: CachedItem = CachedItem {
                    gold: 2900,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::IcebornGauntlet, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:50f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:300f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static IMMORTAL_SHIELDBOW_6673: CachedItem = CachedItem {
                    gold: 3000,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::ImmortalShieldbow, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:55f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static IMPERIAL_MANDATE_4005: CachedItem = CachedItem {
                    gold: 2250,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::ImperialMandate, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:60f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static INFINITY_EDGE_3031: CachedItem = CachedItem {
                    gold: 3450,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::InfinityEdge, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:65f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static INNERVATING_LOCKET_447104: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::InnervatingLocket, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:70f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:200f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static IONIAN_BOOTS_OF_LUCIDITY_3158: CachedItem = CachedItem {
                    gold: 900,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::IonianBootsofLucidity, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:45f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static JAK_SHO_THE_PROTEAN_6665: CachedItem = CachedItem {
                    gold: 3200,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::JakShoTheProtean, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:45f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:350f32,lifesteal:0f32,magic_resist:45f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static JUICE_OF_HASTE_2144: CachedItem = CachedItem {
                    gold: 500,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::JuiceofHaste, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static JUICE_OF_POWER_2142: CachedItem = CachedItem {
                    gold: 500,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::JuiceofPower, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static JUICE_OF_VITALITY_2143: CachedItem = CachedItem {
                    gold: 500,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::JuiceofVitality, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static KAENIC_ROOKERN_2504: CachedItem = CachedItem {
                    gold: 2900,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::KaenicRookern, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:400f32,lifesteal:0f32,magic_resist:80f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static KINDLEGEM_3067: CachedItem = CachedItem {
                    gold: 800,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Kindlegem, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:200f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static KINKOU_JITTE_447116: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::KinkouJitte, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:400f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static KNIGHTS_VOW_3109: CachedItem = CachedItem {
                    gold: 2300,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::KnightsVow, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:40f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:200f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static KRAKEN_SLAYER_6672: CachedItem = CachedItem {
                    gold: 3000,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::KrakenSlayer, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:45f32,attack_speed:40f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static LANE_SWAP_DETECTOR_1501: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::LaneSwapDetector, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static LAST_WHISPER_3035: CachedItem = CachedItem {
                    gold: 1450,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::LastWhisper, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:18f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:20f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static LEGENDARY_ASSASSIN_ITEM_220003: CachedItem = CachedItem {
                    gold: 2000,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::LegendaryAssassinItem, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static LEGENDARY_FIGHTER_ITEM_220001: CachedItem = CachedItem {
                    gold: 2000,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::LegendaryFighterItem, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static LEGENDARY_MAGE_ITEM_220004: CachedItem = CachedItem {
                    gold: 2000,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::LegendaryMageItem, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static LEGENDARY_MARKSMAN_ITEM_220002: CachedItem = CachedItem {
                    gold: 2000,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::LegendaryMarksmanItem, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static LEGENDARY_SUPPORT_ITEM_220006: CachedItem = CachedItem {
                    gold: 2000,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::LegendarySupportItem, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static LEGENDARY_TANK_ITEM_220005: CachedItem = CachedItem {
                    gold: 2000,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::LegendaryTankItem, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static LIANDRYS_TORMENT_6653: CachedItem = CachedItem {
                    gold: 3000,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::LiandrysTorment, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:60f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:300f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static LICH_BANE_3100: CachedItem = CachedItem {
                    gold: 2900,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::LichBane, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:100f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static LIFELINE_4003: CachedItem = CachedItem {
                    gold: 1600,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Lifeline, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:25f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static LIGHTNING_ROD_447119: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::LightningRod, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:30f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:500f32,lifesteal:0f32,magic_resist:30f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static LOCKET_OF_THE_IRON_SOLARI_3190: CachedItem = CachedItem {
                    gold: 2200,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::LocketoftheIronSolari, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:25f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:200f32,lifesteal:0f32,magic_resist:25f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static LONG_SWORD_1036: CachedItem = CachedItem {
                    gold: 350,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::LongSword, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:10f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static LORD_DOMINIKS_REGARDS_3036: CachedItem = CachedItem {
                    gold: 3100,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::LordDominiksRegards, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:40f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:35f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static LOST_CHAPTER_3802: CachedItem = CachedItem {
                    gold: 1200,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::LostChapter, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:40f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:300f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static LUCKY_DICE_2145: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::LuckyDice, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static LUDENS_COMPANION_6655: CachedItem = CachedItem {
                    gold: 2750,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::LudensCompanion, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:100f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:600f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static MALIGNANCE_3118: CachedItem = CachedItem {
                    gold: 2700,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Malignance, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:90f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:600f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static MANAMUNE_3004: CachedItem = CachedItem {
                    gold: 2900,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Manamune, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:35f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:500f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static MAW_OF_MALMORTIUS_3156: CachedItem = CachedItem {
                    gold: 3100,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::MawofMalmortius, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:60f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:40f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static MEJAIS_SOULSTEALER_3041: CachedItem = CachedItem {
                    gold: 1500,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::MejaisSoulstealer, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:20f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:100f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static MERCURIAL_SCIMITAR_3139: CachedItem = CachedItem {
                    gold: 3200,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::MercurialScimitar, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:50f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:35f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static MERCURYS_TREADS_3111: CachedItem = CachedItem {
                    gold: 1250,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::MercurysTreads, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:20f32,mana:0f32,movespeed:45f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static MIKAELS_BLESSING_3222: CachedItem = CachedItem {
                    gold: 2300,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::MikaelsBlessing, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:250f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static MIRAGE_BLADE_447100: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::MirageBlade, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:60f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static MOONFLAIR_SPELLBLADE_447110: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::MoonflairSpellblade, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:85f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:400f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static MOONSTONE_RENEWER_6617: CachedItem = CachedItem {
                    gold: 2200,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::MoonstoneRenewer, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:25f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:200f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static MORELLONOMICON_3165: CachedItem = CachedItem {
                    gold: 2850,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Morellonomicon, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:75f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:350f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static MORTAL_REMINDER_3033: CachedItem = CachedItem {
                    gold: 3300,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::MortalReminder, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:35f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:35f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static MOSSTOMPER_SEEDLING_1103: CachedItem = CachedItem {
                    gold: 450,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::MosstomperSeedling, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static MURAMANA_3042: CachedItem = CachedItem {
                    gold: 2900,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Muramana, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:35f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:860f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static NASHORS_TOOTH_3115: CachedItem = CachedItem {
                    gold: 2900,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::NashorsTooth, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:80f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:50f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static NAVORI_FLICKERBLADE_6675: CachedItem = CachedItem {
                    gold: 2650,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::NavoriFlickerblade, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:40f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static NEEDLESSLY_LARGE_ROD_1058: CachedItem = CachedItem {
                    gold: 1200,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::NeedlesslyLargeRod, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:65f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static NEGATRON_CLOAK_1057: CachedItem = CachedItem {
                    gold: 850,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::NegatronCloak, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:45f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static NIGHT_HARVESTER_444636: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::NightHarvester, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:90f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:300f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static NOONQUIVER_6670: CachedItem = CachedItem {
                    gold: 1300,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Noonquiver, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:15f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static NULL_MAGIC_MANTLE_1033: CachedItem = CachedItem {
                    gold: 400,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::NullMagicMantle, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:20f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static OBLIVION_ORB_3916: CachedItem = CachedItem {
                    gold: 800,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::OblivionOrb, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:25f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static OHMWRECKER_TURRET_ITEM_1500: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::OhmwreckerTurretItem, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:30f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static OPPORTUNITY_6701: CachedItem = CachedItem {
                    gold: 2700,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Opportunity, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:55f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static ORACLE_LENS_3364: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::OracleLens, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static OVERCHARGED_1507: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Overcharged, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static OVERLORDS_BLOODMAIL_2501: CachedItem = CachedItem {
                    gold: 3300,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::OverlordsBloodmail, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:30f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:550f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static PERPLEXITY_4015: CachedItem = CachedItem {
                    gold: 2500,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Perplexity, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:60f32,armor:0f32,armor_penetration_percent:22f32,armor_penetration_flat:0f32,magic_penetration_percent:30f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static PHAGE_3044: CachedItem = CachedItem {
                    gold: 1100,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Phage, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:15f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:200f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static PHANTOM_DANCER_3046: CachedItem = CachedItem {
                    gold: 2650,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::PhantomDancer, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:65f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static PHREAKISH_GUSTO_1510: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::PhreakishGusto, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static PICKAXE_1037: CachedItem = CachedItem {
                    gold: 875,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Pickaxe, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:25f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static PLATED_STEELCAPS_3047: CachedItem = CachedItem {
                    gold: 1200,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::PlatedSteelcaps, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:25f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:45f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static PORO_SNAX_2052: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::PoroSnax, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static PRISMATIC_ITEM_220007: CachedItem = CachedItem {
                    gold: 4000,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::PrismaticItem, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static PROFANE_HYDRA_6698: CachedItem = CachedItem {
                    gold: 2850,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::ProfaneHydra, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:55f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static PROWLERS_CLAW_446693: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::ProwlersClaw, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:55f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static PUPPETEER_447123: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Puppeteer, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:30f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static PYROMANCERS_CLOAK_447118: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::PyromancersCloak, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:400f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static QUICKSILVER_SASH_3140: CachedItem = CachedItem {
                    gold: 1300,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::QuicksilverSash, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:30f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static RABADONS_DEATHCAP_3089: CachedItem = CachedItem {
                    gold: 3500,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::RabadonsDeathcap, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:130f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static RADIANT_VIRTUE_446667: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::RadiantVirtue, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:35f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:400f32,lifesteal:0f32,magic_resist:35f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static RAISE_MORALE_3903: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::RaiseMorale, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static RANDUINS_OMEN_3143: CachedItem = CachedItem {
                    gold: 2700,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::RanduinsOmen, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:75f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:350f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static RAPID_FIRECANNON_3094: CachedItem = CachedItem {
                    gold: 2650,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::RapidFirecannon, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:35f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static RAVENOUS_HYDRA_3074: CachedItem = CachedItem {
                    gold: 3300,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::RavenousHydra, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:65f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static REALITY_FRACTURE_447102: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::RealityFracture, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:80f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:40f32,crit_chance:0f32,crit_damage:0f32,health:300f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static REAPERS_TOLL_443090: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::ReapersToll, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:50f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static RECTRIX_6690: CachedItem = CachedItem {
                    gold: 775,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Rectrix, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:15f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static RECURVE_BOW_1043: CachedItem = CachedItem {
                    gold: 700,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::RecurveBow, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:15f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static REDEMPTION_3107: CachedItem = CachedItem {
                    gold: 2300,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Redemption, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:200f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static REFILLABLE_POTION_2031: CachedItem = CachedItem {
                    gold: 150,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::RefillablePotion, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static REGICIDE_447115: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Regicide, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:60f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static REINFORCED_ARMOR_TURRET_ITEM_1502: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::ReinforcedArmorTurretItem, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static REJUVENATION_BEAD_1006: CachedItem = CachedItem {
                    gold: 300,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::RejuvenationBead, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static REVERBERATION_447114: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Reverberation, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:35f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:40f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:35f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static RIFTMAKER_4633: CachedItem = CachedItem {
                    gold: 3100,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Riftmaker, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:70f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:350f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static RITE_OF_RUIN_3430: CachedItem = CachedItem {
                    gold: 2500,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::RiteofRuin, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:50f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static ROD_OF_AGES_6657: CachedItem = CachedItem {
                    gold: 2600,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::RodofAges, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:45f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:350f32,lifesteal:0f32,magic_resist:0f32,mana:500f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static RUBY_CRYSTAL_1028: CachedItem = CachedItem {
                    gold: 400,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::RubyCrystal, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:150f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static RUNAANS_HURRICANE_3085: CachedItem = CachedItem {
                    gold: 2650,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::RunaansHurricane, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:40f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static RUNECARVER_447108: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Runecarver, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:80f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static RUNIC_COMPASS_3866: CachedItem = CachedItem {
                    gold: 400,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::RunicCompass, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:100f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static RYLAIS_CRYSTAL_SCEPTER_3116: CachedItem = CachedItem {
                    gold: 2600,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::RylaisCrystalScepter, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:65f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:400f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static SANGUINE_GIFT_443062: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::SanguineGift, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:80f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static SAPPHIRE_CRYSTAL_1027: CachedItem = CachedItem {
                    gold: 300,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::SapphireCrystal, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:300f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static SCARECROW_EFFIGY_3330: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::ScarecrowEffigy, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static SCORCHCLAW_PUP_1101: CachedItem = CachedItem {
                    gold: 450,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::ScorchclawPup, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static SCOUTS_SLINGSHOT_3144: CachedItem = CachedItem {
                    gold: 600,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::ScoutsSlingshot, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:20f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static SEEKERS_ARMGUARD_2420: CachedItem = CachedItem {
                    gold: 1600,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::SeekersArmguard, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:40f32,armor:25f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static SERAPHS_EMBRACE_3040: CachedItem = CachedItem {
                    gold: 2900,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::SeraphsEmbrace, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:70f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:1000f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static SERPENTS_FANG_6695: CachedItem = CachedItem {
                    gold: 2500,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::SerpentsFang, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:55f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static SERRATED_DIRK_3134: CachedItem = CachedItem {
                    gold: 1000,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::SerratedDirk, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:20f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static SERYLDAS_GRUDGE_6694: CachedItem = CachedItem {
                    gold: 3000,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::SeryldasGrudge, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:35f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:45f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static SHADOWFLAME_4645: CachedItem = CachedItem {
                    gold: 3200,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Shadowflame, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:110f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:15f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static SHATTERED_ARMGUARD_2421: CachedItem = CachedItem {
                    gold: 1600,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::ShatteredArmguard, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:40f32,armor:25f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static SHEEN_3057: CachedItem = CachedItem {
                    gold: 900,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Sheen, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static SHIELD_OF_MOLTEN_STONE_443058: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::ShieldofMoltenStone, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:100f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:300f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static SHURELYAS_BATTLESONG_2065: CachedItem = CachedItem {
                    gold: 2200,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::ShurelyasBattlesong, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:50f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static SLIGHTLY_MAGICAL_BOOTS_2422: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::SlightlyMagicalBoots, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:25f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static SOLSTICE_SLEIGH_3876: CachedItem = CachedItem {
                    gold: 400,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::SolsticeSleigh, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:200f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static SORCERERS_SHOES_3020: CachedItem = CachedItem {
                    gold: 1100,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::SorcerersShoes, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:12f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:45f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static SPEAR_OF_SHOJIN_3161: CachedItem = CachedItem {
                    gold: 3100,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::SpearofShojin, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:45f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:450f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static SPECTRAL_CUTLASS_224004: CachedItem = CachedItem {
                    gold: 2800,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::SpectralCutlass, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:50f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static SPECTRES_COWL_3211: CachedItem = CachedItem {
                    gold: 1250,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::SpectresCowl, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:200f32,lifesteal:0f32,magic_resist:35f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static SPELLSLINGERS_SHOES_3175: CachedItem = CachedItem {
                    gold: 1600,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::SpellslingersShoes, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:7f32,magic_penetration_flat:18f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:50f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static SPIRIT_VISAGE_3065: CachedItem = CachedItem {
                    gold: 2700,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::SpiritVisage, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:400f32,lifesteal:0f32,magic_resist:50f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static STAFF_OF_FLOWING_WATER_6616: CachedItem = CachedItem {
                    gold: 2250,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::StaffofFlowingWater, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:35f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static STAT_BONUS_220000: CachedItem = CachedItem {
                    gold: 750,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::StatBonus, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static STATIKK_SHIV_3087: CachedItem = CachedItem {
                    gold: 2700,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::StatikkShiv, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:45f32,attack_speed:30f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static STEALTH_WARD_3340: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::StealthWard, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static STEEL_SIGIL_2019: CachedItem = CachedItem {
                    gold: 1100,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::SteelSigil, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:30f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:15f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static STERAKS_GAGE_3053: CachedItem = CachedItem {
                    gold: 3200,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::SteraksGage, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:400f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static STORMRAZOR_223095: CachedItem = CachedItem {
                    gold: 2500,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Stormrazor, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:45f32,attack_speed:25f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static STORMSURGE_4646: CachedItem = CachedItem {
                    gold: 2800,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Stormsurge, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:90f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:15f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static STRIDEBREAKER_6631: CachedItem = CachedItem {
                    gold: 3300,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Stridebreaker, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:40f32,attack_speed:25f32,crit_chance:0f32,crit_damage:0f32,health:450f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static SUNDERED_SKY_6610: CachedItem = CachedItem {
                    gold: 3100,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::SunderedSky, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:40f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:400f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static SUNFIRE_AEGIS_3068: CachedItem = CachedItem {
                    gold: 2700,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::SunfireAegis, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:50f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:350f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static SUPER_MECH_ARMOR_1511: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::SuperMechArmor, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static SUPER_MECH_POWER_FIELD_1512: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::SuperMechPowerField, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static SWIFTMARCH_3170: CachedItem = CachedItem {
                    gold: 1500,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Swiftmarch, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:65f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static SWORD_OF_BLOSSOMING_DAWN_4011: CachedItem = CachedItem {
                    gold: 2500,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::SwordofBlossomingDawn, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:45f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:200f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static SWORD_OF_THE_DIVINE_443060: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::SwordoftheDivine, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static SYMBIOTIC_SOLES_3010: CachedItem = CachedItem {
                    gold: 900,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::SymbioticSoles, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:40f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static SYNCHRONIZED_SOULS_3013: CachedItem = CachedItem {
                    gold: 900,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::SynchronizedSouls, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:45f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static TALISMAN_OF_ASCENSION_443064: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::TalismanofAscension, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static TEAR_OF_THE_GODDESS_3070: CachedItem = CachedItem {
                    gold: 400,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::TearoftheGoddess, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:240f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static TERMINUS_3302: CachedItem = CachedItem {
                    gold: 3000,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Terminus, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:30f32,attack_speed:35f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static THE_BRUTALIZER_2020: CachedItem = CachedItem {
                    gold: 1337,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::TheBrutalizer, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:25f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static THE_COLLECTOR_6676: CachedItem = CachedItem {
                    gold: 3000,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::TheCollector, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:50f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static THE_GOLDEN_SPATULA_224403: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::TheGoldenSpatula, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:125f32,armor:40f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:90f32,attack_speed:60f32,crit_chance:0f32,crit_damage:0f32,health:350f32,lifesteal:0f32,magic_resist:40f32,mana:350f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static THORNMAIL_3075: CachedItem = CachedItem {
                    gold: 2450,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Thornmail, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:75f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:150f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static TIAMAT_3077: CachedItem = CachedItem {
                    gold: 1200,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Tiamat, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:20f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static TITANIC_HYDRA_3748: CachedItem = CachedItem {
                    gold: 3300,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::TitanicHydra, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:40f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:600f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static TOTAL_BISCUIT_OF_EVERLASTING_WILL_2010: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::TotalBiscuitofEverlastingWill, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static TRAILBLAZER_3002: CachedItem = CachedItem {
                    gold: 2400,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Trailblazer, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:40f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:250f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static TRINITY_FORCE_3078: CachedItem = CachedItem {
                    gold: 3333,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::TrinityForce, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:36f32,attack_speed:30f32,crit_chance:0f32,crit_damage:0f32,health:333f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static TUNNELER_2021: CachedItem = CachedItem {
                    gold: 1150,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Tunneler, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:15f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:250f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static TURBO_CHEMTANK_443079: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::TurboChemtank, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:600f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static TURRET_PLATING_1515: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::TurretPlating, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static TWILIGHTS_EDGE_447121: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::TwilightsEdge, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:100f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:70f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static TWIN_MASK_443080: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::TwinMask, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static UMBRAL_GLAIVE_3179: CachedItem = CachedItem {
                    gold: 2500,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::UmbralGlaive, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:55f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static UNENDING_DESPAIR_2502: CachedItem = CachedItem {
                    gold: 2800,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::UnendingDespair, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:25f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:400f32,lifesteal:0f32,magic_resist:25f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static VAMPIRIC_SCEPTER_1053: CachedItem = CachedItem {
                    gold: 900,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::VampiricScepter, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:15f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static VERDANT_BARRIER_4632: CachedItem = CachedItem {
                    gold: 1600,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::VerdantBarrier, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:40f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:25f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static VIGILANT_WARDSTONE_4643: CachedItem = CachedItem {
                    gold: 2300,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::VigilantWardstone, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:25f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:250f32,lifesteal:0f32,magic_resist:30f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static VOID_STAFF_3135: CachedItem = CachedItem {
                    gold: 3000,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::VoidStaff, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:95f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:40f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static VOLTAIC_CYCLOSWORD_6699: CachedItem = CachedItem {
                    gold: 3000,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::VoltaicCyclosword, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:55f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static WARDENS_EYE_1503: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::WardensEye, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static WARDENS_MAIL_3082: CachedItem = CachedItem {
                    gold: 1000,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::WardensMail, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:40f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static WARMOGS_ARMOR_3083: CachedItem = CachedItem {
                    gold: 3100,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::WarmogsArmor, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:1000f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static WATCHFUL_WARDSTONE_4638: CachedItem = CachedItem {
                    gold: 1100,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::WatchfulWardstone, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:10f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:150f32,lifesteal:0f32,magic_resist:15f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static WINGED_MOONPLATE_3066: CachedItem = CachedItem {
                    gold: 800,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::WingedMoonplate, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:200f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static WINTERS_APPROACH_3119: CachedItem = CachedItem {
                    gold: 2400,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::WintersApproach, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:550f32,lifesteal:0f32,magic_resist:0f32,mana:500f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static WITS_END_3091: CachedItem = CachedItem {
                    gold: 2800,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::WitsEnd, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:50f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:45f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static WOOGLETS_WITCHCAP_228002: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::WoogletsWitchcap, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:300f32,armor:50f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static WORDLESS_PROMISE_4016: CachedItem = CachedItem {
                    gold: 2500,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::WordlessPromise, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:50f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static WORLD_ATLAS_3865: CachedItem = CachedItem {
                    gold: 400,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::WorldAtlas, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:30f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static YOUMUUS_GHOSTBLADE_3142: CachedItem = CachedItem {
                    gold: 2800,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::YoumuusGhostblade, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:55f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static YOUR_CUT_3400: CachedItem = CachedItem {
                    gold: 0,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::YourCut, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static YUN_TAL_WILDARROWS_3032: CachedItem = CachedItem {
                    gold: 3000,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::YunTalWildarrows, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:55f32,attack_speed:35f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static ZAZ_ZAKS_REALMSPIKE_3871: CachedItem = CachedItem {
                    gold: 400,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::ZazZaksRealmspike, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:200f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static ZEAL_3086: CachedItem = CachedItem {
                    gold: 1200,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Zeal, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:15f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static ZEKES_CONVERGENCE_3050: CachedItem = CachedItem {
                    gold: 2200,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::ZekesConvergence, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:25f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:300f32,lifesteal:0f32,magic_resist:25f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static ZEPHYR_3172: CachedItem = CachedItem {
                    gold: 2500,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::Zephyr, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:0f32,armor:0f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:50f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static ZHONYAS_HOURGLASS_3157: CachedItem = CachedItem {
                    gold: 3250,
                    prettified_stats: &[],
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::None,
                    metadata: TypeMetadata { 
            kind: ItemId::ZhonyasHourglass, 
            damage_type: DamageType::Unknown, 
            attributes: Attrs::None 
        },
                    range_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    melee_closure: DamageClosures {
                minimum_damage: zero,
                maximum_damage: zero
            },
                    stats: CachedItemStats {ability_power:105f32,armor:50f32,armor_penetration_percent:0f32,armor_penetration_flat:0f32,magic_penetration_percent:0f32,magic_penetration_flat:0f32,attack_damage:0f32,attack_speed:0f32,crit_chance:0f32,crit_damage:0f32,health:0f32,lifesteal:0f32,magic_resist:0f32,mana:0f32,movespeed:0f32,omnivamp:0f32,},
                    zero_addr: [(true, true), (true, true)]
                };pub static SIMULATED_ITEMS:phf::OrderedSet<u32>=phf::phf_ordered_set!(8020u32,228001u32,3003u32,3504u32,3174u32,223039u32,6696u32,4642u32,3102u32,3071u32,2503u32,3153u32,8010u32,3877u32,3072u32,3869u32,3173u32,6609u32,4629u32,3171u32,3137u32,6621u32,3742u32,6333u32,3870u32,6620u32,6692u32,3814u32,3508u32,3073u32,4401u32,3176u32,3110u32,3026u32,3124u32,3084u32,4017u32,223146u32,3152u32,6664u32,4628u32,6697u32,3181u32,6662u32,6673u32,4005u32,6665u32,2504u32,3109u32,6672u32,6653u32,3100u32,3190u32,3036u32,6655u32,3118u32,3004u32,3156u32,3139u32,3222u32,6617u32,3165u32,3033u32,3115u32,6675u32,6701u32,2501u32,4015u32,3046u32,6698u32,3143u32,3094u32,3074u32,3107u32,4633u32,3430u32,6657u32,3085u32,3116u32,6695u32,6694u32,4645u32,2065u32,3876u32,3161u32,224004u32,3175u32,3065u32,6616u32,3087u32,3053u32,223095u32,4646u32,6631u32,6610u32,3068u32,3170u32,4011u32,3302u32,6676u32,3075u32,3748u32,3002u32,3078u32,3179u32,2502u32,3135u32,6699u32,3083u32,3119u32,3091u32,4016u32,3142u32,3032u32,3871u32,3050u32,3172u32,3157u32,);pub static SIMULATED_ITEMS_ENUM:[u16;118]=[0,4,7,8,9,10,11,14,15,18,21,22,25,26,27,35,37,38,43,44,47,53,54,55,67,69,70,71,81,84,95,96,97,107,115,122,123,128,129,130,131,132,133,134,135,136,140,144,147,148,157,158,161,163,166,167,168,169,171,173,176,177,178,181,182,190,193,194,196,202,210,211,212,217,223,224,225,227,230,238,240,241,245,247,249,250,252,253,254,256,259,260,261,262,263,264,267,268,274,276,278,280,282,283,289,290,294,295,298,301,302,304,306,308,309,311,312,313,];pub static DAMAGING_ITEMS:phf::Set<u32>=phf::phf_set!();