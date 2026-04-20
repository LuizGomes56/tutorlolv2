use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use tutorlolv2_gen::Key;

#[derive(Debug, Deserialize, Serialize)]
pub struct ChampionRaw {
    pub id: f32,
    pub apiname: String,
    pub title: String,

    #[serde(default)]
    pub skill_i: Vec<String>,
    #[serde(default)]
    pub skill_q: Vec<String>,
    #[serde(default)]
    pub skill_w: Vec<String>,
    #[serde(default)]
    pub skill_e: Vec<String>,
    #[serde(default)]
    pub skill_r: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChampionTemplatePageRaw {
    pub champion_name: String,
    pub page_title: String,
    pub pages: Vec<WikiLink>,
    pub categories: Vec<WikiLink>,
    pub ability_subpages: Vec<WikiLink>,
    pub parameters_by_section: BTreeMap<String, BTreeMap<String, ParameterEntry>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChampionTemplatePage {
    pub template_name: String,
    pub champion_name: String,
    pub pages: Vec<String>,
    pub categories: Vec<String>,
    pub ability_subpages: Vec<String>,
    pub general: ChampionGeneral,
    pub abilities: ChampionAbilities,
    pub stats: ChampionStats,
    pub mode_specific: ChampionModeSpecificStats,
    pub deprecated: ChampionDeprecated,
    pub extra: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChampionGeneral {
    pub key_name: Option<String>,
    pub id: Option<u32>,
    pub apiname: Option<String>,
    pub title: Option<String>,
    pub resource: Option<String>,
    pub fullname: Option<String>,
    pub nickname: Option<String>,
    pub date: Option<String>,
    pub patch: Option<String>,
    pub changes: Option<String>,
    pub role: Vec<String>,
    pub client_positions: Vec<String>,
    pub external_positions: Vec<String>,
    pub damage: Option<i32>,
    pub toughness: Option<i32>,
    pub control: Option<i32>,
    pub mobility: Option<i32>,
    pub utility: Option<i32>,
    pub difficulty: Option<i32>,
    pub style: Option<i32>,
    pub adaptivetype: Option<String>,
    pub rangetype: Option<String>,
    pub be: Option<u32>,
    pub rp: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChampionAbilities {
    pub skills: Vec<String>,
    pub skill_i: Vec<String>,
    pub skill_q: Vec<String>,
    pub skill_w: Vec<String>,
    pub skill_e: Vec<String>,
    pub skill_r: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChampionStats {
    pub hp_base: Option<f32>,
    pub hp_lvl: Option<f32>,
    pub hp5_base: Option<f32>,
    pub hp5_lvl: Option<f32>,
    pub mp_base: Option<f32>,
    pub mp_lvl: Option<f32>,
    pub mp5_base: Option<f32>,
    pub mp5_lvl: Option<f32>,
    pub arm_base: Option<f32>,
    pub arm_lvl: Option<f32>,
    pub mr_base: Option<f32>,
    pub mr_lvl: Option<f32>,
    pub dam_base: Option<f32>,
    pub dam_lvl: Option<f32>,
    pub as_base: Option<f32>,
    pub as_ratio: Option<f32>,
    pub as_lvl: Option<f32>,
    pub range: Option<f32>,
    pub range_lvl: Option<f32>,
    pub ms: Option<f32>,
    pub ms_lvl: Option<f32>,
    pub crit_base: Option<f32>,
    pub crit_mod: Option<f32>,
    pub missile_speed: Option<f32>,
    pub attack_cast_time: Option<f32>,
    pub attack_total_time: Option<f32>,
    pub attack_delay_offset: Option<f32>,
    pub windup: Option<f32>,
    pub windup_modifier: Option<f32>,
    pub acquisition_radius: Option<f32>,
    pub selection_radius: Option<f32>,
    pub selection_height: Option<f32>,
    pub gameplay_radius: Option<f32>,
    pub pathing_radius: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChampionModeSpecificStats {
    pub aram_dmg_dealt: Option<f32>,
    pub aram_dmg_taken: Option<f32>,
    pub aram_healing: Option<f32>,
    pub aram_shielding: Option<f32>,
    pub aram_manaregen_mod: Option<f32>,
    pub aram_energyregen_mod: Option<f32>,
    pub aram_total_as: Option<f32>,
    pub aram_ms_mod: Option<f32>,
    pub aram_tenacity: Option<f32>,
    pub aram_ability_haste: Option<f32>,

    pub nb_dmg_dealt: Option<f32>,
    pub nb_dmg_taken: Option<f32>,
    pub nb_healing: Option<f32>,
    pub nb_shielding: Option<f32>,
    pub nb_manaregen_mod: Option<f32>,
    pub nb_energyregen_mod: Option<f32>,
    pub nb_total_as: Option<f32>,
    pub nb_ms_mod: Option<f32>,
    pub nb_tenacity: Option<f32>,
    pub nb_ability_haste: Option<f32>,

    pub ofa_dmg_dealt: Option<f32>,
    pub ofa_dmg_taken: Option<f32>,
    pub ofa_healing: Option<f32>,
    pub ofa_shielding: Option<f32>,
    pub ofa_manaregen_mod: Option<f32>,
    pub ofa_energyregen_mod: Option<f32>,
    pub ofa_total_as: Option<f32>,
    pub ofa_ms_mod: Option<f32>,
    pub ofa_tenacity: Option<f32>,
    pub ofa_ability_haste: Option<f32>,

    pub swift_hp_base: Option<f32>,
    pub swift_hp_lvl: Option<f32>,
    pub swift_mp_base: Option<f32>,
    pub swift_mp_lvl: Option<f32>,
    pub swift_arm_base: Option<f32>,
    pub swift_arm_lvl: Option<f32>,
    pub swift_mr_base: Option<f32>,
    pub swift_mr_lvl: Option<f32>,
    pub swift_hp5_base: Option<f32>,
    pub swift_hp5_lvl: Option<f32>,
    pub swift_mp5_base: Option<f32>,
    pub swift_mp5_lvl: Option<f32>,
    pub swift_dam_base: Option<f32>,
    pub swift_dam_lvl: Option<f32>,
    pub swift_as_base: Option<f32>,
    pub swift_as_ratio: Option<f32>,
    pub swift_as_lvl: Option<f32>,
    pub swift_ms: Option<f32>,
    pub swift_dmg_dealt: Option<f32>,
    pub swift_dmg_taken: Option<f32>,
    pub swift_healing: Option<f32>,
    pub swift_shielding: Option<f32>,
    pub swift_manaregen_mod: Option<f32>,
    pub swift_energyregen_mod: Option<f32>,
    pub swift_total_as: Option<f32>,
    pub swift_ms_mod: Option<f32>,
    pub swift_tenacity: Option<f32>,
    pub swift_ability_haste: Option<f32>,

    pub urf_dmg_dealt: Option<f32>,
    pub urf_dmg_taken: Option<f32>,
    pub urf_healing: Option<f32>,
    pub urf_shielding: Option<f32>,
    pub urf_manaregen_mod: Option<f32>,
    pub urf_energyregen_mod: Option<f32>,
    pub urf_total_as: Option<f32>,
    pub urf_ms_mod: Option<f32>,
    pub urf_tenacity: Option<f32>,
    pub urf_ability_haste: Option<f32>,

    pub usb_dmg_dealt: Option<f32>,
    pub usb_dmg_taken: Option<f32>,
    pub usb_healing: Option<f32>,
    pub usb_shielding: Option<f32>,
    pub usb_manaregen_mod: Option<f32>,
    pub usb_energyregen_mod: Option<f32>,
    pub usb_total_as: Option<f32>,
    pub usb_ms_mod: Option<f32>,
    pub usb_tenacity: Option<f32>,
    pub usb_ability_haste: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChampionDeprecated {
    pub attack: Option<f32>,
    pub defense: Option<f32>,
    pub magic: Option<f32>,
    pub herotype: Option<String>,
    pub alttype: Option<String>,
    pub attack_delay: Option<f32>,
    pub as_lvl1: Option<f32>,
    pub as_lvl1_bonus: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedWikiPage {
    pub page_title: String,
    pub source_module: String,

    pub raw_html: String,

    pub sections: Vec<WikiSection>,
    pub links: Vec<WikiLink>,
    pub images: Vec<WikiImage>,
    pub tables: Vec<WikiTable>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiSection {
    pub heading: String,
    pub heading_level: u8,
    pub text_blocks: Vec<String>,
    pub links: Vec<WikiLink>,
    pub tables: Vec<usize>, // índices em ParsedWikiPage.tables
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiLink {
    pub text: String,
    pub href: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiImage {
    pub alt: Option<String>,
    pub src: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiTable {
    pub headers: Vec<String>,
    pub rows: Vec<WikiRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiRow {
    pub cells: Vec<WikiCell>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiCell {
    pub text: String,
    pub html: String,
    pub links: Vec<WikiLink>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterEntry {
    pub parameter: String,
    pub value_text: String,
    pub value_html: String,
    pub description_text: String,
    pub description_html: String,
    pub links: Vec<WikiLink>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbilityFetchJob {
    pub champion_name: String,
    pub apiname: String,
    pub slot: Key,
    pub variant: usize,
    pub ability_name: String,

    pub module: String,
    pub html_path: String,
    pub json_path: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AbilityTemplatePageRaw {
    pub champion_name: String,
    pub slot: Key,
    pub variant: usize,
    pub ability_name: String,
    pub page_title: String,
    pub parameters: BTreeMap<String, ParameterEntry>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AbilityData {
    pub champion_name: String,
    pub slot: Key,
    pub variant: usize,
    pub ability_name: String,
    pub damage_type: Option<DamageType>,
    pub formulas: Vec<AbilityFormula>,
    pub raw_parameters: std::collections::BTreeMap<String, String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum DamageType {
    Physical,
    Magic,
    True,
    Mixed,
    Other(String),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AbilityFormula {
    pub label: String,
    pub kind: FormulaKind,
    pub per_rank: Vec<Expr>,
    pub source_parameter: String,
    pub raw_text: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum FormulaKind {
    Damage,
    Shield,
    Heal,
    Cost,
    Cooldown,
    Other,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Expr {
    Sum(Vec<Term>),
    Unknown(String),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Term {
    Flat(f32),
    Scale {
        coeff: f32,
        stat: StatRef,
    },
    PercentOfTargetMaxHealth(f32),
    PercentOfTargetCurrentHealth(f32),
    PercentOfMissingHealth(f32),
    Per100Stat {
        base: f32,
        per100: f32,
        stat: StatRef,
    },
    Unknown(String),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum StatRef {
    AbilityPower,
    AttackDamageTotal,
    AttackDamageBonus,
    BonusHealth,
    MaxHealth,
    Armor,
    BonusArmor,
    MagicResist,
    BonusMagicResist,
    CritChance,
    MoveSpeed,
    Level,
    Unknown(String),
}

#[derive(Debug, Clone)]
pub struct AbilityBlockRaw {
    pub suffix: String,
    pub description: Option<String>,
    pub leveling_text: Option<String>,
    pub leveling_html: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ParsedLevelLine {
    pub label: String,
    pub raw_expr: String,
}

#[derive(Debug, Clone)]
pub struct RankVector {
    pub values: Vec<f32>,
}

impl AbilityData {
    pub fn damage_formulas(&self) -> impl Iterator<Item = &AbilityFormula> {
        self.formulas
            .iter()
            .filter(|f| f.kind == FormulaKind::Damage)
    }
}
