use scrypto::prelude::*;
use crate::access_manager::access_manager::access_manager::AccessManager;
use crate::access_manager::custom_types::*;
use crate::validator_extension::custom_types::*; // for ValidatorOwnerBadgeData etc.
pub(crate) fn internal_create_custom_access_key_badge(access_manager: &AccessManager, super_permissions: KeyBadgeSuperPermissions, basic_permissions: KeyBadgeBasicPermissions, validator_permissions: Option<KeyBadgeValidatorPermissions>) -> NonFungibleBucket {
            // if component_owner, accept super permissions as is
            // if key holder, super permissions must be none

    let access_key_badge_data = AccessKeyBadgeData {
        manager_component_address: Runtime::global_address(),
        auth_badge_address: access_manager.auth_badge.resource_address(),
        super_permissions: super_permissions,
        basic_permissions: basic_permissions,
        validator_permissions: validator_permissions
    };
    access_manager.access_key_badge_resource_manager.mint_ruid_non_fungible(access_key_badge_data).as_non_fungible()
}
pub(crate) fn check_caller_permissions(access_manager: &AccessManager, required_permission: &str, proof: NonFungibleProof) {
    let has_permission: bool;
    if proof.resource_manager().address() == access_manager.owner_badge_resource_manager.address() {
        // if the proof is the owner badge, he has permission, proceed
        has_permission = true;
    }
    else if proof.resource_manager().address() == access_manager.access_key_badge_resource_manager.address() {
        // if the proof is an access key badge, check if it has the right permissions
        let access_key_data: AccessKeyBadgeData = proof.skip_checking().non_fungible::<AccessKeyBadgeData>().data();
        if required_permission == "create_access_key" {
            has_permission = access_key_data.super_permissions.create_access_key;
        } else if required_permission == "recall_access_key" {
            has_permission = access_key_data.super_permissions.recall_access_key;
        } else if required_permission == "create_native_proof" {
            has_permission = access_key_data.basic_permissions.create_native_proof;
        } else if required_permission == "stake_as_owner" && access_key_data.validator_permissions.is_some() {
            has_permission = access_key_data.validator_permissions.unwrap().stake_as_owner;
        } else if required_permission == "register" && access_key_data.validator_permissions.is_some() {
            has_permission = access_key_data.validator_permissions.unwrap().register;
        } else if required_permission == "unregister" && access_key_data.validator_permissions.is_some() {
            has_permission = access_key_data.validator_permissions.unwrap().unregister;
        } else if required_permission == "update_key" && access_key_data.validator_permissions.is_some() {
            has_permission = access_key_data.validator_permissions.unwrap().update_key;
        } else if required_permission == "update_fee" && access_key_data.validator_permissions.is_some() {
            has_permission = access_key_data.validator_permissions.unwrap().update_fee;
        } else if required_permission == "update_accept_delegated_stake" && access_key_data.validator_permissions.is_some() {
            has_permission = access_key_data.validator_permissions.unwrap().update_accept_delegated_stake;
        } else if required_permission == "signal_protocol_update_readiness" && access_key_data.validator_permissions.is_some() {
            has_permission = access_key_data.validator_permissions.unwrap().signal_protocol_update_readiness;
        } else if required_permission == "lock_owner_stake_units" && access_key_data.validator_permissions.is_some() {
            has_permission = access_key_data.validator_permissions.unwrap().lock_owner_stake_units;
        } else if required_permission == "start_unlock_owner_stake_units" && access_key_data.validator_permissions.is_some() {
            has_permission = access_key_data.validator_permissions.unwrap().start_unlock_owner_stake_units;
        } else if required_permission == "finish_unlock_owner_stake_units" && access_key_data.validator_permissions.is_some() {
            has_permission = access_key_data.validator_permissions.unwrap().finish_unlock_owner_stake_units;
        } else {
            has_permission = false;
        }
    }
    else {
        has_permission = false;
    }
    if !has_permission {
        panic!("You do not have permission to perform the required!");
    }
}