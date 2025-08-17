use scrypto::prelude::*;
#[derive(ScryptoSbor, NonFungibleData)]
pub struct ValidatorOwnerBadgeData {
    pub name: String,
    pub validator: ComponentAddress,
}
#[derive(ScryptoSbor, NonFungibleData)]
pub struct KeyBadgeValidatorPermissions {
    pub stake_as_owner: bool,
    pub register: bool,
    pub unregister: bool,
    pub update_key: bool,
    pub update_fee: bool,
    pub update_accept_delegated_stake: bool,
    pub signal_protocol_update_readiness: bool,
    pub lock_owner_stake_units: bool,
    pub start_unlock_owner_stake_units: bool,
    pub finish_unlock_owner_stake_units: bool
}