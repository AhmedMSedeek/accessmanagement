# Release Notes — v2 Update

This release (v2) contains the Access Key redesign that separates roles and enables fine-grained delegation.

Highlights:
- New badge factory methods:
  - Basic key (backwards-compatible)
  - Basic key (`create_basic_key_badge`) — callable by owner or key-holder with proof (v2 behavior). Backwards-compatible manifests labeled "basic" reproduce the original v1 flow.
  - Super key (delegated owner-like powers for key operations)
  - Custom key (fully configurable permissions)
- Permission system extended with validator permissions (stake, register, update key/fee, lock/unlock owner stake units).
- Owner retains exclusive rights to deposit/withdraw the original Auth Badge.
- Key-holder cannot mint keys with super permissions; owner-only.
- Manifests and helper scripts updated to demonstrate new flows.

Developer notes:
- When constructing manifests for `create_custom_access_key_badge` you must include the permissions values correctly in the manifest arguments. below are all possible permissions:
  - Super permissions: CreateAccessKey, RecallAccessKey
  - Basic permissions: CreateNativeProof
  - Validator-related: Register, Unregister, StakeAsOwner, UpdateKey,   UpdateFee, UpdateAcceptDelegatedStake, SignalProtocolUpdateReadiness,   LockOwnerStakeUnits, StartUnlockOwnerStakeUnits, FinishUnlockOwnerStakeUnits
- Validator operations are gated by permission checks performed in `src/access_manager/access_manager_helper.rs` and handled by `src/validator_extension/validator_extension.rs`.

Files changed/added:
- Modified: `src/access_manager/access_manager.rs`
- Modified: `src/access_manager/custom_types.rs`
- Modified: `src/access_manager/access_manager_helper.rs`
- Modified: `src/validator_extension/validator_extension.rs`
- Added samples: manifests/create_basic_key_badge.rtm, manifests/create_super_key_badge.rtm, manifests/create_custom_key_badge.rtm
- Updated scripts: `scripts/all.ps1`

---

v1 — prior behavior (short summary)

The original v1 behavior provided a single key factory:
- v1 factory: a single factory which minted a basic access key that allowed the holder to create a native proof of the deposited Auth Badge.

Roles in v1:
- Owner: deposit/withdraw auth badge, create/recall keys, create auth proofs.
- Key holder: create auth badge proofs via issued keys.

This document includes v1 compatibility manifests labeled "basic" that reproduce the original behaviour.
