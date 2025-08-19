use scrypto::prelude::*;
use crate::access_manager::custom_types::OwnerBadgeData;

#[blueprint]
mod test_helpers {
    pub struct TestHelpers;

    impl TestHelpers {
        // Mint an OwnerBadge RUID non-fungible and return its ResourceAddress and the NonFungibleLocalId
        // This helper is intended for use only from unit/integration tests.
        pub fn mint_owner_badge(dapp_definition: ComponentAddress) -> (ResourceAddress, NonFungibleLocalId) {
            let owner_data = OwnerBadgeData {
                manager_component_address: ComponentAddress::from_u64(0),
                auth_badge_address: ResourceAddress::from_u64(0),
            };
            let resource = ResourceBuilder::new_ruid_non_fungible::<OwnerBadgeData>(OwnerRole::None)
                .mint_roles(mint_roles!{ minter => rule!(allow_all); minter_updater => rule!(deny_all); })
                .create_with_initial_supply(vec![owner_data]);

            let mgr = resource.resource_manager();
            let ids = mgr.non_fungible_local_ids();
            let id = ids.into_iter().next().expect("minted id");
            (resource.resource_address(), id)
        }
    }
}
