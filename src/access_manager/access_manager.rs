use scrypto::prelude::*;
use crate::access_manager::custom_types::*;

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
            // The proof is passed explicitly to PUBLIC methods, so the permission check is done in the code level
            deposit_auth_badge => restrict_to: [component_owner];
            create_super_access_key_badge => restrict_to: [component_owner];
            // `create_access_key_badge` was intentionally removed in v2; use `create_basic_key_badge` instead.
            create_basic_key_badge => PUBLIC;
            create_custom_access_key_badge =>  PUBLIC;
            recall_key_badge =>  PUBLIC;
            burn_key_badge => restrict_to: [component_owner, key_holder];
            create_auth_badge_proof => PUBLIC;
            withdraw_auth_badge => restrict_to: [component_owner];

            // validator extension methods
            validator_register => PUBLIC;
            validator_unregister => PUBLIC;
            validator_stake_as_owner => PUBLIC;
            validator_update_key => PUBLIC;
            validator_update_fee => PUBLIC;
            validator_update_accept_delegated_stake => PUBLIC;
            validator_signal_protocol_update_readiness => PUBLIC;
            validator_lock_owner_stake_units => PUBLIC;
            validator_start_unlock_owner_stake_units => PUBLIC;
            validator_finish_unlock_owner_stake_units => PUBLIC;
        }
    }
    enable_package_royalties! {
        new => Usd(dec!(0.01));
        new_with_address_reservation => Usd(dec!(0.01));
        deposit_auth_badge => Usd(dec!(0.01));
        create_super_access_key_badge => Usd(dec!(0.01));
        create_basic_key_badge => Usd(dec!(0.01));
        create_custom_access_key_badge => Usd(dec!(0.01));
        recall_key_badge => Usd(dec!(0.01));
        burn_key_badge => Usd(dec!(0.01));
        create_auth_badge_proof => Usd(dec!(0.01));
        withdraw_auth_badge => Usd(dec!(0.01));

        // validator extension methods
        validator_register => Usd(dec!(0.01));
        validator_unregister => Usd(dec!(0.01));
        validator_stake_as_owner => Usd(dec!(0.01));
        validator_update_key => Usd(dec!(0.01));
        validator_update_fee => Usd(dec!(0.01));
        validator_update_accept_delegated_stake => Usd(dec!(0.01));
        validator_signal_protocol_update_readiness => Usd(dec!(0.01));
        validator_lock_owner_stake_units => Usd(dec!(0.01));
        validator_start_unlock_owner_stake_units => Usd(dec!(0.01));
        validator_finish_unlock_owner_stake_units => Usd(dec!(0.01));
    }
    pub struct AccessManager {
        pub auth_badge: NonFungibleVault,
        pub owner_badge_resource_manager: ResourceManager,
        pub access_key_badge_resource_manager: ResourceManager,
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

            let access_key_badge_resource_manager = ResourceBuilder::new_ruid_non_fungible::<AccessKeyBadgeData>(OwnerRole::None)
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
                    minter => rule!(require(global_caller(component_address)) || require(owner_badge.resource_address()));
                    minter_updater => rule!(deny_all);
                ))
                .burn_roles(burn_roles! (
                    burner => rule!(allow_all);
                    burner_updater => rule!(deny_all);
                ))
                .withdraw_roles(withdraw_roles! (
                    withdrawer => rule!(require(owner_badge.resource_address()));
                    withdrawer_updater => rule!(deny_all);
                ))
                .deposit_roles(deposit_roles! (
                    depositor => rule!(allow_all);
                    depositor_updater => rule!(deny_all);
                ))
                .recall_roles(recall_roles! (
                    recaller => rule!(require(global_caller(component_address)) || require(owner_badge.resource_address()));
                    recaller_updater => rule!(deny_all);
                ))
                .freeze_roles(freeze_roles! (
                    freezer => rule!(deny_all);
                    freezer_updater => rule!(deny_all);
                ))
                .non_fungible_data_update_roles(non_fungible_data_update_roles! (
                    non_fungible_data_updater => rule!(require(global_caller(component_address)) || require(owner_badge.resource_address()));
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
        pub fn create_super_access_key_badge(&mut self, include_validator_permissions: bool) -> NonFungibleBucket {
            // called by the owner only, no need to re-check the permissions
            let mut permissions = vec![
                KeyBadgePermission::CreateAccessKey,
                KeyBadgePermission::RecallAccessKey,
                KeyBadgePermission::CreateNativeProof
            ];
            if include_validator_permissions {
                permissions.extend(vec![
                    KeyBadgePermission::Validator_StakeAsOwner,
                    KeyBadgePermission::Validator_Register,
                    KeyBadgePermission::Validator_Unregister,
                    KeyBadgePermission::Validator_UpdateKey,
                    KeyBadgePermission::Validator_UpdateFee,
                    KeyBadgePermission::Validator_UpdateAcceptDelegatedStake,
                    KeyBadgePermission::Validator_SignalProtocolUpdateReadiness,
                    KeyBadgePermission::Validator_LockOwnerStakeUnits,
                    KeyBadgePermission::Validator_StartUnlockOwnerStakeUnits,
                    KeyBadgePermission::Validator_FinishUnlockOwnerStakeUnits
                ]);
            }
            self.internal_create_custom_access_key_badge(permissions)
        }
    // `create_access_key_badge` was intentionally removed in v2; use `create_basic_key_badge` (owner or key-holder with proper proof) instead.
        pub fn create_basic_key_badge(&mut self, include_validator_permissions: bool, proof: NonFungibleProof) -> NonFungibleBucket {
            // can be called by either the owner or a key holder
            // need to check if the key holder has the right permissions
            let mut permissions = vec![
                KeyBadgePermission::CreateNativeProof
            ];
            if include_validator_permissions {
                permissions.extend(vec![
                    KeyBadgePermission::Validator_Register,
                    KeyBadgePermission::Validator_Unregister,
                    KeyBadgePermission::Validator_StakeAsOwner,
                    KeyBadgePermission::Validator_UpdateKey,
                    KeyBadgePermission::Validator_UpdateFee,
                    KeyBadgePermission::Validator_UpdateAcceptDelegatedStake,
                    KeyBadgePermission::Validator_SignalProtocolUpdateReadiness,
                    KeyBadgePermission::Validator_LockOwnerStakeUnits,
                    KeyBadgePermission::Validator_StartUnlockOwnerStakeUnits,
                    KeyBadgePermission::Validator_FinishUnlockOwnerStakeUnits
                ]);
            }

            crate::access_manager::access_manager_helper::check_caller_permissions(self, KeyBadgePermission::CreateAccessKey, Some(permissions.clone()), proof);

            self.internal_create_custom_access_key_badge(permissions)
        }
        pub fn create_custom_access_key_badge(&mut self, permissions: Vec<String>, proof: NonFungibleProof) -> NonFungibleBucket {
            let permissions_enum: Result<Vec<_>, _> = permissions
                .into_iter()
                .map(|s| KeyBadgePermission::from_str(&s).map_err(|e| e.to_string()))
                .collect();

           let permissions_vec = match permissions_enum {
                Ok(v) => v,
                Err(e) => {
                    panic!("Invalid permission string: {}", e); 
                    // or handle differently
                }
            };
            // if component_owner, accept super permissions as is
            // if key holder, super permissions must be none
            if proof.resource_manager().address() == self.access_key_badge_resource_manager.address() {
                // if the proof is an access key badge, make sure no super permissions are included
                if permissions_vec.iter().any(|p| matches!(p, KeyBadgePermission::CreateAccessKey | KeyBadgePermission::RecallAccessKey)) {
                    panic!("Key holders cannot create access key badges with super permissions!");
                }
            }

            // can be called by either the owner or a key holder
            // need to check if the key holder has the right permissions
            crate::access_manager::access_manager_helper::check_caller_permissions(self, KeyBadgePermission::CreateAccessKey, Some(permissions_vec.clone()), proof);
            
            self.internal_create_custom_access_key_badge(permissions_vec)
        }
        pub fn recall_key_badge(&mut self, vault_address: InternalAddress, proof: NonFungibleProof) -> NonFungibleBucket {
            crate::access_manager::access_manager_helper::check_caller_permissions(self, KeyBadgePermission::RecallAccessKey, None, proof);
            
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
        pub fn create_auth_badge_proof(&mut self, proof: NonFungibleProof) -> NonFungibleProof {
            crate::access_manager::access_manager_helper::check_caller_permissions(self, KeyBadgePermission::CreateNativeProof, None, proof);
            
            self.auth_badge.as_non_fungible().create_proof_of_non_fungibles(&self.auth_badge.as_non_fungible().non_fungible_local_ids(1))
        }
        pub fn withdraw_auth_badge(&mut self) -> NonFungibleBucket {
            assert!(self.auth_badge.amount() == Decimal::ONE, "Cannot create a proof with an empty vault, deposit the auth badge first!");
            self.auth_badge.take(1)
        }
    
        // private methods
        fn internal_create_custom_access_key_badge(&mut self, permissions: Vec<KeyBadgePermission>) -> NonFungibleBucket {
                    // if component_owner, accept super permissions as is
                    // if key holder, super permissions must be none
            let access_key_badge_data = AccessKeyBadgeData {
                manager_component_address: Runtime::global_address(),
                auth_badge_address: self.auth_badge.resource_address(),
                permissions: permissions
            };
            self.access_key_badge_resource_manager.mint_ruid_non_fungible(access_key_badge_data).as_non_fungible()
        }

        // validator extension methods
        pub fn validator_register(&mut self, proof: NonFungibleProof) {
            crate::validator_extension::validator_extension::register(self, proof);
        }
        pub fn validator_unregister(&mut self, proof: NonFungibleProof) {
            crate::validator_extension::validator_extension::unregister(self, proof);
        }
        pub fn validator_stake_as_owner(&mut self, stake: Bucket, proof: NonFungibleProof) -> Bucket {
            crate::validator_extension::validator_extension::stake_as_owner(self, stake, proof)
        }
        pub fn validator_update_key(&mut self, key: Secp256k1PublicKey, proof: NonFungibleProof) {
            crate::validator_extension::validator_extension::update_key(self, key, proof);
        }
        pub fn validator_update_fee(&mut self, new_fee_factor: Decimal, proof: NonFungibleProof) {
            crate::validator_extension::validator_extension::update_fee(self, new_fee_factor, proof);
        }
        pub fn validator_update_accept_delegated_stake(&mut self, accept_delegated_stake: bool, proof: NonFungibleProof) {
            crate::validator_extension::validator_extension::update_accept_delegated_stake(self, accept_delegated_stake, proof);
        }
        pub fn validator_signal_protocol_update_readiness(&mut self, vote: String, proof: NonFungibleProof) {
            crate::validator_extension::validator_extension::signal_protocol_update_readiness(self, vote, proof);
        }
        pub fn validator_lock_owner_stake_units(&mut self, stake_unit_bucket: Bucket, proof: NonFungibleProof) {
            crate::validator_extension::validator_extension::lock_owner_stake_units(self, stake_unit_bucket, proof);
        }
        pub fn validator_start_unlock_owner_stake_units(&mut self, requested_stake_unit_amount: Decimal, proof: NonFungibleProof) {
            crate::validator_extension::validator_extension::start_unlock_owner_stake_units(self, requested_stake_unit_amount, proof);
        }
        pub fn validator_finish_unlock_owner_stake_units(&mut self, proof: NonFungibleProof) -> Bucket {
            crate::validator_extension::validator_extension::finish_unlock_owner_stake_units(self, proof)
        }
    }
}
