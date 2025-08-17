use scrypto::prelude::*;
use crate::validator_extension::custom_types::*;

#[derive(ScryptoSbor, NonFungibleData)]
pub struct OwnerBadgeData {
    pub manager_component_address: ComponentAddress,
    pub auth_badge_address: ResourceAddress
}
#[derive(ScryptoSbor, NonFungibleData)]
pub struct KeyBadgeSuperPermissions {
    pub create_access_key: bool,
    pub recall_access_key: bool
}
#[derive(ScryptoSbor, NonFungibleData)]
pub struct KeyBadgeBasicPermissions {
    pub create_native_proof: bool
}
#[derive(ScryptoSbor, NonFungibleData)]
pub struct AccessKeyBadgeData {
    pub manager_component_address: ComponentAddress,
    pub auth_badge_address: ResourceAddress,

    #[mutable]
    pub super_permissions: KeyBadgeSuperPermissions,
    #[mutable]
    pub basic_permissions: KeyBadgeBasicPermissions,
    #[mutable]
    pub validator_permissions: Option<KeyBadgeValidatorPermissions>
}