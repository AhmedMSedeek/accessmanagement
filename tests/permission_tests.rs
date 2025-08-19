use scrypto_test::prelude::*;
use accessmanagement::access_manager::custom_types::OwnerBadgeData;
use accessmanagement::access_manager::custom_types::KeyBadgePermission;
use accessmanagement::access_manager::access_manager::AccessManager;

#[derive(ScryptoSbor, NonFungibleData)]
struct AuthBadgeData {
    dummy: u8,
}

#[test]
fn access_manager_basic_permission_flow() {
    let mut env = TestEnvironment::new();

    // publish package
    let package = PackageFactory::compile_and_publish(this_package!(), &mut env, CompileProfile::Fast)
        .expect("publish");

    // Create an auth badge resource (simple RUID NFT) to pass into AccessManager::new
    let auth_data = AuthBadgeData { dummy: 1u8 };
    let auth_bucket = ResourceBuilder::new_ruid_non_fungible::<AuthBadgeData>(OwnerRole::None)
        .mint_initial_supply(vec![auth_data], &mut env)
        .expect("mint auth badge");
    let auth_resource_address = auth_bucket.resource_address(&mut env).expect("resource address");

    // Instantiate AccessManager via package function `AccessManager::new`
    let (component, owner_badge): (Global<AccessManager>, NonFungibleBucket) = env
        .call_function_typed(
            package,
            "AccessManager",
            "new",
            &(auth_resource_address, ComponentAddress::from_u64(1u64)),
        )
        .expect("instantiate access manager");

    // Create a proof from owner_badge to use for PUBLIC calls that still require a proof param
    let owner_ids = owner_badge.non_fungible_local_ids(&mut env).expect("ids");
    let owner_first_id = owner_ids.get(0).expect("id").clone();
    let owner_proof: NonFungibleProof = owner_badge.create_proof_of_non_fungibles(vec![owner_first_id.clone()], &mut env).expect("create proof");

    // Using owner_proof, create a custom single-permission badge for UpdateFee
    let permissions = vec![String::from("Validator_UpdateFee")];
    let fee_badge: NonFungibleBucket = env.call_method_typed(&component, "create_custom_access_key_badge", &(permissions.clone(), owner_proof.clone())).expect("create custom badge");

    // Create a proof from the fee_badge to use when calling validator_update_fee
    let fee_ids = fee_badge.non_fungible_local_ids(&mut env).expect("fee ids");
    let fee_id = fee_ids.get(0).expect("fee id").clone();
    let fee_proof: NonFungibleProof = fee_badge.create_proof_of_non_fungibles(vec![fee_id.clone()], &mut env).expect("create fee proof");

    // Call validator_update_fee with fee_proof — should succeed
    let res_ok = env.call_method_typed::<_, _, ()>(&component, "validator_update_fee", &(Decimal::from(2u32), fee_proof.clone()));
    assert!(res_ok.is_ok(), "validator_update_fee should succeed with UpdateFee permission");

    // Now create a custom badge with Validator_UpdateKey permission only and ensure validator_update_key fails
    let key_perms = vec![String::from("Validator_UpdateKey")];
    let key_badge: NonFungibleBucket = env.call_method_typed(&component, "create_custom_access_key_badge", &(key_perms.clone(), owner_proof)).expect("create key badge");
    let key_ids = key_badge.non_fungible_local_ids(&mut env).expect("key ids");
    let key_id = key_ids.get(0).expect("key id").clone();
    let key_proof: NonFungibleProof = key_badge.create_proof_of_non_fungibles(vec![key_id.clone()], &mut env).expect("create key proof");

    // Attempt validator_update_fee with key_proof — should FAIL because permission is Validator_UpdateKey only
    let res_fail = env.call_method_typed::<_, _, ()>(&component, "validator_update_fee", &(Decimal::from(3u32), key_proof));
    assert!(res_fail.is_err(), "validator_update_fee should fail with a badge that only has Validator_UpdateKey permission");

    // Create a super access key badge (owner only) with validator permissions
    let super_bucket: NonFungibleBucket = env.call_method_typed(&component, "create_super_access_key_badge", &(true)).expect("create super badge");
    let super_ids = super_bucket.non_fungible_local_ids(&mut env).expect("super ids");
    let super_id = super_ids.get(0).expect("super id").clone();
    let super_proof: NonFungibleProof = super_bucket.create_proof_of_non_fungibles(vec![super_id.clone()], &mut env).expect("create super proof");

    // Super badge should allow validator_update_fee and validator_update_key
    let super_fee_res = env.call_method_typed::<_, _, ()>(&component, "validator_update_fee", &(Decimal::from(5u32), super_proof.clone()));
    assert!(super_fee_res.is_ok(), "validator_update_fee should succeed with super badge");

    // Create a CreateNativeProof-only badge and test create_auth_badge_proof
    let native_perms = vec![String::from("CreateNativeProof")];
    let native_bucket: NonFungibleBucket = env.call_method_typed(&component, "create_custom_access_key_badge", &(native_perms.clone(), owner_proof.clone())).expect("create native badge");
    let native_ids = native_bucket.non_fungible_local_ids(&mut env).expect("native ids");
    let native_id = native_ids.get(0).expect("native id").clone();
    let native_proof: NonFungibleProof = native_bucket.create_proof_of_non_fungibles(vec![native_id.clone()], &mut env).expect("create native proof");

    let create_proof_res = env.call_method_typed::<_, _, NonFungibleProof>(&component, "create_auth_badge_proof", &(native_proof.clone()));
    assert!(create_proof_res.is_ok(), "create_auth_badge_proof should succeed with CreateNativeProof badge");
}
