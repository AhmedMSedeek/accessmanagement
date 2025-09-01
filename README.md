# Access Management

Access Management is a Scrypto package that provides an Access Manager blueprint for delegating an "Auth Badge" NFT's authority via issued Access Key NFTs.

This README documents the v2 behavior: three new access key factory methods and a stricter permission model. A short section documenting the original v1 behavior is included for backward-compatibility reference.

## Terminology

- Auth Badge: the original NFT that holds authority to perform privileged actions.
- Access Manager Owner Badge: a badge issued to the component instantiator with owner-level privileges.
- Access Key Badge: a badge minted by the owner (or by a super key-holder, where allowed) to delegate a subset of permissions.
- Delegate: the recipient of an Access Key Badge.

## Roles & high-level changes (v2)

Owner (Access Manager Owner Badge) abilities:
- deposit_auth_badge and withdraw_auth_badge
- create access key badges (basic, super, custom)
- recall and burn access key badges
- create_auth_badge_proof
- call validator-extension methods

Key holder (Access Key Badge) abilities:
- Only the set of permissions encoded in the Access Key Badge (KeyBadgePermission enum). Key-holders cannot deposit/withdraw the original Auth Badge or change owner-level settings.

## New badge factory methods

`create_basic_key_badge(include_validator_permissions: bool, proof: NonFungibleProof)`: Create a basic access key. Callable by the component owner or by a key-holder who provides a valid proof; when called by a key-holder the method enforces that the caller's key has the `CreateAccessKey` permission.
`create_super_access_key_badge(include_validator_permissions: bool)`: Callable only by the owner, Grants the key-holder authority to mint and recall other keys and create proofs — owner-like for key operations but cannot deposit/withdraw the original Auth Badge, also allows creating a native proof of the Auth Badge. Optionally includes validator-related permissions to allow calling validator extension methods.
- create_super_access_key_badge(include_validator_permissions: bool): Callable only by the owner, Grants the key-holder authority to mint and recall other keys and create proofs — owner-like for key operations but cannot deposit/withdraw the original Auth Badge, also allows creating a native proof of the Auth Badge. Optionally includes validator-related permissions to allow calling validator extension methods.
- create_custom_access_key_badge(permissions: Vec<String>, proof: Proof): Create a key with an explicit set of permissions. If called by a non-owner, minting of super permissions is rejected.

See the code for exact method names and signatures in `src/access_manager/access_manager.rs` and permission definitions in `src/access_manager/custom_types.rs`.

---

## v1 — prior behavior (for reference)

The v1 implementation supported a simpler model. For backward reference only:

- Roles:
    - Owner: could deposit/withdraw the Auth Badge, create access key badges (single factory method), recall access keys, and create auth badge proofs.
    - Key holder: could create an auth badge proof via the issued access key.

- Badge factory (v1):
    - A single factory method minted a basic access key that allowed the holder to create a native proof of the deposited Auth Badge.

This README's manifest examples labeled "basic" reproduce the v1 behavior and are provided for compatibility.


## ⚠️ Important Security Warning: CreateNativeProof Permission

**WARNING:** If a badge is created with the permission to create native proof (`CreateNativeProof`), the badge holder can do whatever they want with that proof outside of the component, regardless of whether or not they have permission to access other methods. Grant this permission only to trusted parties and with full awareness of its implications.

## Permission enum

Permissions are defined in `KeyBadgePermission` (see `src/access_manager/custom_types.rs`). Valid values listed below:
- Super permissions: CreateAccessKey, RecallAccessKey
- Basic permissions: CreateNativeProof
- Validator-related: Validator_Register, Validator_Unregister, Validator_StakeAsOwner, Validator_UpdateKey, Validator_UpdateFee, Validator_UpdateAcceptDelegatedStake, Validator_SignalProtocolUpdateReadiness, Validator_LockOwnerStakeUnits, Validator_StartUnlockOwnerStakeUnits, Validator_FinishUnlockOwnerStakeUnits

## Quick usage examples (transaction manifest snippets)

Create Access Manager component

```
CALL_FUNCTION
    Address("${package}")
    "AccessManager"
    "new"
    Address("${auth_badge}")
    Address("${dApp_account_address}");

CALL_METHOD Address("${account}") "deposit_batch" Expression("ENTIRE_WORKTOP");
```

Deposit the Auth Badge into the component

```
CALL_METHOD Address("${account}") "withdraw_non_fungibles" Address("${auth_badge}") Array<NonFungibleLocalId>(NonFungibleLocalId("${auth_badge_id}"));
TAKE_NON_FUNGIBLES_FROM_WORKTOP Address("${auth_badge}") Array<NonFungibleLocalId>(NonFungibleLocalId("${auth_badge_id}")) Bucket("auth_badge_bucket");

CALL_METHOD Address("${account}") "create_proof_of_non_fungibles" Address("${access_manager_owner_badge}") Array<NonFungibleLocalId>(NonFungibleLocalId("${access_manager_owner_badge_id}"));

CALL_METHOD
    Address("${component}")
    "deposit_auth_badge"
    Bucket("auth_badge_bucket");
```

Create an access key badge (backwards compatible, owner-only)

```
CALL_METHOD Address("${account}") "create_proof_of_non_fungibles" Address("${access_manager_owner_badge}") Array<NonFungibleLocalId>(NonFungibleLocalId("${access_manager_owner_badge_id}"));

CALL_METHOD
    Address("${component}")
    "create_basic_key_badge";

CALL_METHOD Address("${delegate_account}") "try_deposit_batch_or_abort" Expression("ENTIRE_WORKTOP") None;
```

Create a super access key badge (owner-only)

```
CALL_METHOD Address("${account}") "create_proof_of_non_fungibles" Address("${access_manager_owner_badge}") Array<NonFungibleLocalId>(NonFungibleLocalId("${access_manager_owner_badge_id}"));

CALL_METHOD
    Address("${component}")
    "create_super_access_key_badge"
    Bool(true); # include_validator_permissions

CALL_METHOD Address("${delegate_account}") "try_deposit_batch_or_abort" Expression("ENTIRE_WORKTOP") None;
```

Create a custom access key badge (owner, key holder with permission "CreateAccessKey")

```
CALL_METHOD Address("${account}") "create_proof_of_non_fungibles" Address("${access_manager_owner_badge}") Array<NonFungibleLocalId>(NonFungibleLocalId("${access_manager_owner_badge_id}"));

CALL_METHOD
    Address("${component}")
    "create_custom_access_key_badge"
    Array<String>("CreateNativeProof", "UpdateFee");

CALL_METHOD Address("${delegate_account}") "try_deposit_batch_or_abort" Expression("ENTIRE_WORKTOP") None;
```

Recall & burn an access key badge

```
CALL_METHOD Address("${account}") "create_proof_of_non_fungibles" Address("${access_manager_owner_badge}") Array<NonFungibleLocalId>(NonFungibleLocalId("${access_manager_owner_badge_id}"));

CALL_METHOD
    Address("${component}")
    "recall_key_badge"
    Address("${access_key_badge_vault_address}");

TAKE_NON_FUNGIBLES_FROM_WORKTOP Address("${access_key_badge}") Array<NonFungibleLocalId>(NonFungibleLocalId("${access_key_badge_id}")) Bucket("access_key_badge_bucket");

CALL_METHOD
    Address("${component}")
    "burn_key_badge"
    Bucket("access_key_badge_bucket");
```

Create Auth Badge proof (for owner or keys that include CreateNativeProof)

```
CALL_METHOD Address("${account}") "create_proof_of_non_fungibles" Address("${access_key_badge}") Array<NonFungibleLocalId>(NonFungibleLocalId("${access_key_badge_id}"));

CALL_METHOD
    Address("${component}")
    "create_auth_badge_proof";
```

## Manifests & samples

Added manifest samples under `manifests/`:
- `create_basic_key_badge.rtm` — create a basic access key badge (callable by owner or key-holders with permission)
- `create_super_key_badge.rtm` — create a super access key badge (validator perms optional)
- `create_custom_key_badge.rtm` — create a custom access key badge

## Files of interest

- `src/access_manager/access_manager.rs` — main blueprint implementation
- `src/access_manager/custom_types.rs` — `KeyBadgePermission` enum and related types
- `src/access_manager/access_manager_helper.rs` — helper checks and minting
- `src/validator_extension/validator_extension.rs` — validator proxy methods
- `manifests/` — transaction manifest samples

For more details and concrete encodings, see the `CHANGELOG.md` and `RELEASE_NOTES.md` files in the repository.

## Deployed packages & recommended dApp accounts

Deployed package addresses (for reference):

- v1.0.0:
    Stokenet: `package_tdx_2_1p54xl6f3d7leetxpp85j0ua3ll2qfx4xxjcrdvsdgchr00t8qspmnq`
    Mainnet: `package_rdx1p4m04kkm8tw3fefwrf7zvgxjw8k0n9t30vawgq2kl90q3r77nf59w8`
- v2.0.0
    Stokenet: `package_tdx_2_1pkn7f6jqgu4dnf02qnj93t64hhsn95vch5dksh2j3zs5jy0hj55vv3`
    Mainnet: `package_rdx1pk99tw5p7djduc3je8u3m5p2twkkzxexlc40sw65wnns280h2f5237`

Use your own dApp account address when constructing manifests. If you don't have one, you can use the RadixPlanet dApp account addresses below:

- Stokenet dApp account: `account_tdx_2_128ly7s6494uasmggf9rxy6th2e6zu53hj7p0uxgq2ucdmzf43gqkus`
- Mainnet dApp account: `account_rdx12xjdx9ntkjl60r7fuv9az8uzmad0d05mqmjstrpkpvtcew87crahw6`

Note: the deployed package addresses above are provided for convenience and reference only. When testing or deploying in your own environment, publish the package locally and use the generated package address.

### v2 permission behaviour (important)

For version 2: a key holder with the permission `CreateAccessKey` cannot create access keys that include either `CreateAccessKey` or `RecallAccessKey` , or other permissions the key holder themselves do not have. In other words, non-owner callers are blocked from minting keys with any permissions they don't hold (for example, `CreateNativeProof` or any validator permissions not present on their own key). This restriction enforces stricter delegation boundaries compared to v1.