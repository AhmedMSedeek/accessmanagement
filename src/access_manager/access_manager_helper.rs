use scrypto::prelude::*;
use crate::access_manager::access_manager::access_manager::AccessManager;
use crate::access_manager::custom_types::*;
///
/// This function checks if the caller has the required permissions to perform an action.
/// It checks if the caller has the owner badge or an access key badge with the required permissions.
/// If the caller does not have the required permissions, it panics with an error message.
/// If the caller is a key holder and the required permission is `CreateAccessKey`, create_badge_permissions must be present to also checks if the key holder has the same permission they are granting.
/// 
pub(crate) fn check_caller_permissions(access_manager: &AccessManager, required_permission: KeyBadgePermission, create_badge_permissions: Option<Vec<KeyBadgePermission>>, proof: NonFungibleProof) {
    let has_permission: bool;
    if proof.resource_manager().address() == access_manager.owner_badge_resource_manager.address() {
        let _owner_badge_data = proof.check(access_manager.owner_badge_resource_manager.address()).non_fungible::<OwnerBadgeData>().data();
        // if the proof is the owner badge, he has permission, proceed
        has_permission = true;
    }
    else if proof.resource_manager().address() == access_manager.access_key_badge_resource_manager.address() {
        // if the proof is an access key badge, check if it has the right permissions
        let access_key_data = proof.check(access_manager.access_key_badge_resource_manager.address()).non_fungible::<AccessKeyBadgeData>().data();
        has_permission = access_key_data.permissions.contains(&required_permission);
        if required_permission == KeyBadgePermission::CreateAccessKey {
            // the key holder must have the same permission he is giving as well
            if create_badge_permissions.is_none() {
                panic!("Key holder must provide the permissions they are granting when creating a new access key badge!");
            }
            let create_badge_permissions = create_badge_permissions.unwrap();
            create_badge_permissions.iter().for_each(|permission| {
                if !access_key_data.permissions.contains(permission) {
                    panic!("Key holder must have the permissions he is granting when creating a new access key badge!, missing permission: {:?}", permission );
                }
            });
        }
    }
    else {
        has_permission = false;
    }
    if !has_permission {
        panic!("You do not have permission to perform the required action!");
    }
}