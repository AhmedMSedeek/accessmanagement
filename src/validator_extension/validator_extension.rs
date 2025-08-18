use scrypto::prelude::*;
use crate::validator_extension::custom_types::*; // for ValidatorOwnerBadgeData etc.
use crate::access_manager::access_manager::access_manager::AccessManager;
use crate::access_manager::access_manager_helper::*;
use crate::access_manager::custom_types::*;

fn validator_authorize<F, O>(access_manager: &AccessManager, f: F) -> O where F: FnOnce(&mut Global<Validator>) -> O,
{
    let nft_data: ValidatorOwnerBadgeData = access_manager.auth_badge.non_fungible().data();
    let validator_address = Some(nft_data.validator);
    let mut validator: Global<Validator> = validator_address.unwrap().into();
    let non_fungible_id = access_manager.auth_badge.non_fungible_local_id();
    access_manager
        .auth_badge
        .authorize_with_non_fungibles(&indexset!(non_fungible_id), || f(&mut validator))
}
pub(crate) fn register(access_manager: &AccessManager, proof: NonFungibleProof) {
    check_caller_permissions(access_manager, KeyBadgePermission::Register, proof);
    validator_authorize(access_manager, |validator| {
        validator.register()
    })
}
pub(crate) fn unregister(access_manager: &AccessManager, proof: NonFungibleProof) {
    check_caller_permissions(access_manager, KeyBadgePermission::Unregister, proof);
    validator_authorize(access_manager, |validator| {
        validator.unregister()
    })
}
pub(crate) fn stake_as_owner(access_manager: &AccessManager, stake: Bucket, proof: NonFungibleProof) -> Bucket {
    check_caller_permissions(access_manager, KeyBadgePermission::StakeAsOwner, proof);
    validator_authorize(access_manager, |validator| {
        validator.stake_as_owner(stake)
    })
}
pub(crate) fn update_key(access_manager: &AccessManager, key: Secp256k1PublicKey, proof: NonFungibleProof) {
    check_caller_permissions(access_manager, KeyBadgePermission::UpdateKey, proof);
    validator_authorize(access_manager, |validator| {
        validator.update_key(key)
    })
}
pub(crate) fn update_fee(access_manager: &AccessManager, new_fee_factor: Decimal, proof: NonFungibleProof) {
    check_caller_permissions(access_manager, KeyBadgePermission::UpdateFee, proof);
    validator_authorize(access_manager, |validator| {
        validator.update_fee(new_fee_factor)
    })
}
pub(crate) fn update_accept_delegated_stake(access_manager: &AccessManager, accept_delegated_stake: bool, proof: NonFungibleProof) {
    check_caller_permissions(access_manager, KeyBadgePermission::UpdateAcceptDelegatedStake, proof);
    validator_authorize(access_manager, |validator| {
        validator.update_accept_delegated_stake(accept_delegated_stake)
    })
}
pub(crate) fn signal_protocol_update_readiness(access_manager: &AccessManager, vote: String, proof: NonFungibleProof) {
    check_caller_permissions(access_manager, KeyBadgePermission::SignalProtocolUpdateReadiness, proof);
    validator_authorize(access_manager, |validator| {
        validator.signal_protocol_update_readiness(vote)
    })
}
pub(crate) fn lock_owner_stake_units(access_manager: &AccessManager, stake_unit_bucket: Bucket, proof: NonFungibleProof) {
    check_caller_permissions(access_manager, KeyBadgePermission::LockOwnerStakeUnits, proof);
    validator_authorize(access_manager, |validator| {
        validator.lock_owner_stake_units(stake_unit_bucket)
    })
}
pub(crate) fn start_unlock_owner_stake_units(access_manager: &AccessManager, requested_stake_unit_amount: Decimal, proof: NonFungibleProof) {
    check_caller_permissions(access_manager, KeyBadgePermission::StartUnlockOwnerStakeUnits, proof);
    validator_authorize(access_manager, |validator| {
        validator.start_unlock_owner_stake_units(requested_stake_unit_amount)
    })
}
pub(crate) fn finish_unlock_owner_stake_units(access_manager: &AccessManager, proof: NonFungibleProof) -> Bucket {
    check_caller_permissions(access_manager, KeyBadgePermission::FinishUnlockOwnerStakeUnits, proof);
    validator_authorize(access_manager, |validator| {
        validator.finish_unlock_owner_stake_units()
    })
}
