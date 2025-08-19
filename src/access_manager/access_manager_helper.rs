use scrypto::prelude::*;
use crate::access_manager::access_manager::access_manager::AccessManager;
use crate::access_manager::custom_types::*;
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
        panic!("You do not have permission to perform the required action!");
    }
}