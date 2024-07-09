use scrypto::prelude::*;

#[derive(ScryptoSbor, NonFungibleData)]
pub struct OwnerBadgeData {
    pub manager_component_address: ComponentAddress,
    pub auth_badge_address: ResourceAddress,
    // #[mutable]
    
}
#[derive(ScryptoSbor, NonFungibleData)]
pub struct AccessKeyBadgeData {
    pub manager_component_address: ComponentAddress,
    pub auth_badge_address: ResourceAddress,
    // #[mutable]
    
}

#[blueprint]
mod access_manager {
    enable_function_auth! {
        new => rule!(allow_all);
        new_with_address_reservation => rule!(allow_all);
    }
    enable_method_auth! { 
        roles { 
            component_owner => updatable_by: [];
            key_holder => updatable_by: [];
        },
        methods { 
            deposit_auth_badge => restrict_to: [component_owner];
            create_access_key_badge => restrict_to: [component_owner];
            recall_key_badge => restrict_to: [component_owner];
            burn_key_badge => restrict_to: [component_owner];
            create_auth_badge_proof => restrict_to: [component_owner, key_holder];
            withdraw_auth_badge => restrict_to: [component_owner];
        }
    }
    enable_package_royalties! {
        new => Usd(dec!(0.01));
        new_with_address_reservation => Usd(dec!(0.01));
        deposit_auth_badge => Usd(dec!(0.01));
        create_access_key_badge => Usd(dec!(0.01));
        recall_key_badge => Usd(dec!(0.01));
        burn_key_badge => Usd(dec!(0.01));
        create_auth_badge_proof => Usd(dec!(0.01));
        withdraw_auth_badge => Usd(dec!(0.01));
    }
    struct AccessManager {
        auth_badge: NonFungibleVault,
        owner_badge_resource_manager: ResourceManager,
        access_key_badge_resource_manager: ResourceManager
    }
    impl AccessManager {
        pub fn new(auth_badge_address: ResourceAddress, dapp_definition_address: ComponentAddress) -> (Global<AccessManager>,  NonFungibleBucket) {
            let (address_reservation, _component_address) = Runtime::allocate_component_address(AccessManager::blueprint_id());
            Self::new_with_address_reservation(auth_badge_address, dapp_definition_address, address_reservation)
        }
        pub fn new_with_address_reservation(auth_badge_address: ResourceAddress, dapp_definition_address: ComponentAddress, address_reservation: GlobalAddressReservation) -> (Global<AccessManager>,  NonFungibleBucket) {
            let global_address = Runtime::get_reservation_address(&address_reservation);
            let component_address = ComponentAddress::try_from_hex(global_address.to_hex().as_str()).unwrap();
            let owner_badge_data = OwnerBadgeData {
                manager_component_address: component_address,
                auth_badge_address: auth_badge_address
            };
            let owner_badge = ResourceBuilder::new_ruid_non_fungible::<OwnerBadgeData>(OwnerRole::None)
                .metadata(metadata! (
                    roles {
                        metadata_locker => OWNER;
                        metadata_locker_updater => OWNER;
                        metadata_setter => OWNER;
                        metadata_setter_updater => OWNER;
                    },
                    init {
                        "manager_component_address" => component_address, locked;
                        "name" => "Access Manager Owner Badge", locked;
                        "description" => "Access Manager Owner badge belongs to the owner of the auth badge which this access manager component is managing", locked;
                        "tags" => vec!["Badge", "Access Control", "Owner Badge"], locked;
                        "dapp_definitions" => vec![dapp_definition_address], locked;
                    }
                ))
                .mint_roles(mint_roles! (
                    minter => rule!(require(global_caller(component_address)));
                    minter_updater => rule!(deny_all);
                ))
                .burn_roles(burn_roles! (
                    burner => rule!(require(global_caller(component_address)));
                    burner_updater => rule!(deny_all);
                ))
                .withdraw_roles(withdraw_roles! (
                    withdrawer => rule!(allow_all);
                    withdrawer_updater => rule!(deny_all);
                ))
                .deposit_roles(deposit_roles! (
                    depositor => rule!(allow_all);
                    depositor_updater => rule!(deny_all);
                ))
                .recall_roles(recall_roles! (
                    recaller => rule!(deny_all);
                    recaller_updater => rule!(deny_all);
                ))
                .freeze_roles(freeze_roles! (
                    freezer => rule!(deny_all);
                    freezer_updater => rule!(deny_all);
                ))
                .non_fungible_data_update_roles(non_fungible_data_update_roles! (
                    non_fungible_data_updater => rule!(require(global_caller(component_address)));
                    non_fungible_data_updater_updater => rule!(deny_all);
                ))
                .mint_initial_supply(vec![owner_badge_data]);

            let access_key_badge_resource_manager = ResourceBuilder::new_ruid_non_fungible::<OwnerBadgeData>(OwnerRole::None)
                .metadata(metadata! (
                    roles {
                        metadata_locker => OWNER;
                        metadata_locker_updater => OWNER;
                        metadata_setter => OWNER;
                        metadata_setter_updater => OWNER;
                    },
                    init {
                        "manager_component_address" => component_address, locked;
                        "name" => "Access Manager Access Key Badge", locked;
                        "description" => "Access Manager Access Key badge is the badge used to create proof for the auth badge that the access manager component is managing", locked;
                        "tags" => vec!["Badge", "Access Control", "Key Badge"], locked;
                        "dapp_definitions" => vec![dapp_definition_address], locked;
                    }
                ))
                .mint_roles(mint_roles! (
                    minter => rule!(require(global_caller(component_address)));
                    minter_updater => rule!(deny_all);
                ))
                .burn_roles(burn_roles! (
                    burner => rule!(allow_all);
                    burner_updater => rule!(deny_all);
                ))
                .withdraw_roles(withdraw_roles! (
                    withdrawer => rule!(deny_all);
                    withdrawer_updater => rule!(deny_all);
                ))
                .deposit_roles(deposit_roles! (
                    depositor => rule!(allow_all);
                    depositor_updater => rule!(deny_all);
                ))
                .recall_roles(recall_roles! (
                    recaller => rule!(require(global_caller(component_address)));
                    recaller_updater => rule!(deny_all);
                ))
                .freeze_roles(freeze_roles! (
                    freezer => rule!(deny_all);
                    freezer_updater => rule!(deny_all);
                ))
                .non_fungible_data_update_roles(non_fungible_data_update_roles! (
                    non_fungible_data_updater => rule!(require(global_caller(component_address)));
                    non_fungible_data_updater_updater => rule!(deny_all);
                ))
                .create_with_no_initial_supply();

            let component = Self {
                                auth_badge: NonFungibleVault::new(auth_badge_address),
                                owner_badge_resource_manager: owner_badge.resource_manager(),
                                access_key_badge_resource_manager: access_key_badge_resource_manager
                            }
                .instantiate()
                .prepare_to_globalize(OwnerRole::Updatable(rule!(require(global_caller(component_address)))))
                .metadata(metadata! (
                    roles {
                        metadata_locker => OWNER;
                        metadata_locker_updater => OWNER;
                        metadata_setter => OWNER;
                        metadata_setter_updater => OWNER;
                    },
                    init {
                        "dapp_definition" => dapp_definition_address, locked;
                    }
                ))
                .roles(roles! (
                    component_owner => rule!(require(owner_badge.resource_address()));
                    key_holder => rule!(require(access_key_badge_resource_manager.address()));
                ))
                .with_address(address_reservation)
                .globalize();

            (component, owner_badge)
        }
            pub fn deposit_auth_badge(&mut self, auth_badge: NonFungibleBucket) {
            assert!(self.auth_badge.is_empty(), "Access Manager component already has an auth badge!");
            assert!(auth_badge.amount() == Decimal::ONE, "Cannot deposit any amount other than exactly one!");
            self.auth_badge.put(auth_badge);
        }
        pub fn create_access_key_badge(&mut self) -> NonFungibleBucket {
            let access_key_badge_data = AccessKeyBadgeData {
                manager_component_address: Runtime::global_address(),
                auth_badge_address: self.auth_badge.resource_address()
            };
            self.access_key_badge_resource_manager.mint_ruid_non_fungible(access_key_badge_data).as_non_fungible()
        }
        pub fn recall_key_badge(&mut self, vault_address: InternalAddress) -> NonFungibleBucket {
            let recalled_bucket: Bucket = scrypto_decode(&ScryptoVmV1Api::object_call_direct(
                vault_address.as_node_id(),
                VAULT_RECALL_IDENT,
                scrypto_args!(Decimal::ONE),
              )).unwrap();
          
              recalled_bucket.as_non_fungible()
        }
        pub fn burn_key_badge(&mut self, key_badge: NonFungibleBucket) {
            key_badge.burn();
        }
        pub fn create_auth_badge_proof(&mut self) -> NonFungibleProof {
            self.auth_badge.as_non_fungible().create_proof_of_non_fungibles(&self.auth_badge.as_non_fungible().non_fungible_local_ids(1))
        }
        pub fn withdraw_auth_badge(&mut self) -> NonFungibleBucket {
            self.auth_badge.take(1)
        }
    }
}
