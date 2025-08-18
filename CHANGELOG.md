# Changelog

All notable changes to this project are documented here.

## [v2] - Changes introduced in v2
- Introduced three access key factory methods:
  - `create_basic_key_badge` (basic key; callable by owner or key-holder with proof)
  - `create_super_access_key_badge` (super key: can mint/recall keys; optionally includes validator permissions)
  - `create_custom_access_key_badge` (fully customizable set of permissions)
- Key-holder strict permission model: key holders may only perform actions explicitly allowed by their `AccessKeyBadgeData.permissions`. Super permissions cannot be minted by key-holders (only owners).
- Validator extension integration: selected validator operations now can be called via the access manager if the caller's key has validator permissions.
- Added/updated manifests and scripting samples (see `manifests/` and `scripts/all.ps1`).
- Minor metadata/roles updates for resource managers to reflect new rules.

## [v1] - Original behavior (for reference)
-- Single factory method (v1): a single factory which minted a basic access key allowing the holder to create native proofs.
-- Roles:
  - Owner: deposit/withdraw auth badge, create/recall keys, create auth proofs.
  - Key holder: create auth badge proofs via the issued key.

See source for details:
- `src/access_manager/access_manager.rs`
- `src/access_manager/custom_types.rs`
- `src/access_manager/access_manager_helper.rs`
- `src/validator_extension/validator_extension.rs`
