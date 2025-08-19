use scrypto::prelude::*;
use strum_macros::EnumString;

#[derive(ScryptoSbor, NonFungibleData)]
pub struct OwnerBadgeData {
    pub manager_component_address: ComponentAddress,
    pub auth_badge_address: ResourceAddress
}
#[derive(ScryptoSbor, NonFungibleData)]
pub struct AccessKeyBadgeData {
    pub manager_component_address: ComponentAddress,
    pub auth_badge_address: ResourceAddress,

    #[mutable]
    pub permissions: Vec<KeyBadgePermission>
}
#[derive(ScryptoSbor, PartialEq, EnumString, Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum KeyBadgePermission {
    // Super permissions
    CreateAccessKey,
    RecallAccessKey,

    // Basic permissions
    CreateNativeProof,

    // Validator permissions
    Validator_Register,
    Validator_Unregister,
    Validator_StakeAsOwner,
    Validator_UpdateKey,
    Validator_UpdateFee,
    Validator_UpdateAcceptDelegatedStake,
    Validator_SignalProtocolUpdateReadiness,
    Validator_LockOwnerStakeUnits,
    Validator_StartUnlockOwnerStakeUnits,
    Validator_FinishUnlockOwnerStakeUnits
}