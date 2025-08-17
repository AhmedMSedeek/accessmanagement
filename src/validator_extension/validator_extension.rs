use scrypto::prelude::*;
use crate::validator_extension::custom_types::*; // for ValidatorOwnerBadgeData etc.
use crate::access_manager::access_manager::access_manager::AccessManager;

fn validator_authorize<F, O>(access_manager: &AccessManager, f: F) -> O
where
    F: FnOnce(&mut Global<Validator>) -> O,
{
    let nft_data: ValidatorOwnerBadgeData = access_manager.auth_badge.non_fungible().data();
    let validator_address = Some(nft_data.validator);
    let mut validator: Global<Validator> = validator_address
        .expect("The component address of the validator component should be known at this point")
        .into();
    let non_fungible_id = access_manager.auth_badge.non_fungible_local_id();
    access_manager
        .auth_badge
        .authorize_with_non_fungibles(&indexset!(non_fungible_id), || f(&mut validator))
}

pub(crate) fn stake_as_owner(access_manager: &AccessManager, stake: Bucket, _proof: Proof) -> Bucket {
    validator_authorize(access_manager, |validator| {
        validator.stake_as_owner(stake)
    })
}