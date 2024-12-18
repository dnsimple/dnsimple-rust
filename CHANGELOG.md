# Changelog

This project uses [Semantic Versioning 2.0.0](http://semver.org/).

## main

## 2.0.0

- CHANGED: Deprecated `from` and `to` fields in `EmailForward`
- CHANGED: Drop support for Rust < 1.83
- CHANGED: Add support for Rust 1.83
- CHANGED: `DomainCollaborators` have been deprecated and will be removed in the next major version. Please use our Domain Access Control feature.

## 1.0.0

- CHANGED: Drop support for Rust < 1.76
- CHANGED: Add support for Rust 1.76

## 0.7.0

FEATURES:

- NEW: Added `Registrar::check_registrant_change` to retrieves the requirements of a registrant change. (#51)
- NEW: Added `Registrar::get_registrant_change` to retrieves the details of an existing registrant change. (#51)
- NEW: Added `Registrar::create_registrant_change` to start registrant change. (#51)
- NEW: Added `Registrar::list_registrant_changes` to lists the registrant changes for a domain. (#51)
- NEW: Added `Registrar::delete_registrant_change` to cancel an ongoing registrant change from the account. (#51)

- NEW: Added `Registrar::enable_domain_transfer_lock` to enable the domain transfer lock for a domain. (#50)
- NEW: Added `Registrar::disable_domain_transfer_lock` to disable the domain transfer lock for a domain. (#50)
- NEW: Added `Registrar::get_domain_transfer_lock` to get the domain transfer lock status for a domain. (#50)

## 0.6.0

ENHANCEMENTS:

- NEW: Added `secondary`, `last_transferred_at`, `active` to `Zone` (dnsimple/dnsimple-rust#47)

## 0.5.0

FEATURES:

- NEW: Added `Zones::activate_dns` to activate DNS services (resolution) for a zone. (dnsimple/dnsimple-rust#44)
- NEW: Added `Zones::deactivate_dns` to deactivate DNS services (resolution) for a zone. (dnsimple/dnsimple-rust#44)

## 0.4.0

- CHANGED: Depedency updates
- NEW: Support the `signature_algorithm` parameter in Let's Encrypt APIs
- NEW: Support the `get_domain_registration` and `get_domain_renewal` Registrar APIs

## 0.3.0

- FIX: Fix serialized name of ZoneRecordPayload::record_type (dnsimple/dnsimple-rust#32)
- FIX: Fix Clippy Lint explicit_auto_deref (dnsimple/dnsimple-rust#33)
- CHANGED: Sets Edition to 2021 (dnsimple/dnsimple-rust#34)
- CHANGED: Update Ureq dependency to 2.6 (dnsimple/dnsimple-rust#34)

## 0.2.1

- CHANGED: Expose specific model errors on validation error (dnsimple/dnsimple-rust#27)

## 0.2.0

- CHANGED: removed unwrap() from the codebase
- CHANGED: functions return Result<T,DNSimpleError> now
- CHANGED: Deprecate Certificate's `contact_id` (dnsimple/dnsimple-rust#23)

## 0.1.3

- FIX: Bug preventing authorized requests (POST, PUT, PATCH)

## 0.1.2

- FIX: Clippy, RustFmt, Cargo cleanup

## 0.1.1

- FIX: Renames the project from `dnsimple_rust` to `dnsimple`

## 0.1.0

- ADDS: Automatic releasing when tagging a release
- First implementation of the dnsimple-rust client (including all the endpoints)
