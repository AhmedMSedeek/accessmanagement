use scrypto::prelude::*;
use crate::access_manager::access_manager::access_manager::AccessManager;
use crate::access_manager::custom_types::*;
pub(crate) fn internal_create_custom_access_key_badge(access_manager: &AccessManager, permissions: Vec<KeyBadgePermission>) -> NonFungibleBucket {
            // if component_owner, accept super permissions as is
            // if key holder, super permissions must be none

    let access_key_badge_data = AccessKeyBadgeData {
        manager_component_address: Runtime::global_address(),
        auth_badge_address: access_manager.auth_badge.resource_address(),
        permissions: permissions
    };
    access_manager.access_key_badge_resource_manager.mint_ruid_non_fungible(access_key_badge_data).as_non_fungible()
}
pub(crate) fn check_caller_permissions(access_manager: &AccessManager, required_permission: KeyBadgePermission, proof: NonFungibleProof) {
    let has_permission: bool;
    if proof.resource_manager().address() == access_manager.owner_badge_resource_manager.address() {
        // if the proof is the owner badge, he has permission, proceed
        has_permission = true;
    }
    else if proof.resource_manager().address() == access_manager.access_key_badge_resource_manager.address() {
        // if the proof is an access key badge, check if it has the right permissions
        let access_key_data: AccessKeyBadgeData = proof.skip_checking().non_fungible::<AccessKeyBadgeData>().data();
        has_permission = access_key_data.permissions.contains(&required_permission);
    }
    else {
        has_permission = false;
    }
    if !has_permission {
        panic!("You do not have permission to perform the required!");
    }
}