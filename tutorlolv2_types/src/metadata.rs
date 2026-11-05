//! Metadata attached to generated entities and their merged wiki data.
use super::*;

/// A generic metadata holder for [`AbilityId`], [`ItemId`], or [`RuneId`].
/// Contains its damage type, attributes, and which instance of the enum the value is.
#[derive(
    Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq, Encode, Decode, Serialize, Deserialize,
)]
#[cfg_attr(feature = "bake", derive(databake::Bake))]
#[cfg_attr(feature = "bake", databake(path = tutorlolv2_types))]
pub struct TypeMetadata<T> {
    pub kind: T,
    pub damage_type: DamageType,
    pub attributes: Attrs,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Encode, Decode, Serialize, Deserialize)]
#[cfg_attr(feature = "bake", derive(databake::Bake))]
#[cfg_attr(feature = "bake", databake(path = tutorlolv2_types))]
pub struct MergeData {
    pub min: u8,
    pub max: u8,
    pub alias: AbilityId,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct DevMergeData {
    pub min: AbilityId,
    pub max: AbilityId,
    pub alias: AbilityId,
}
