# Changelog

This project uses [Semantic Versioning 2.0.0](http://semver.org/), the format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## 5.1.0 - 2026-02-26

### Added

- Added `get_domain_research_status` to research a domain for availability and registration status. (#96)

## Unreleased

### Fixed

- Fixed `deactivate_dns` using incorrect endpoint type

## 5.0.0 - 2026-01-23

### Removed

- Removed deprecated `check_domain_premium_price`. Use `get_domain_prices` instead. (dnsimple/dnsimple-developer#916)
- Removed deprecated `get_whois_privacy` (dnsimple/dnsimple-developer#919)
- Removed deprecated `renew_whois_privacy` (dnsimple/dnsimple-developer#919)

## 4.0.0 - 2025-08-20

### Added

- Added `active` to `EmailForward`.

### Changed

- Removed `from` and `to` fields in `EmailForward`. Please use `alias_email` and `destination_email` instead.

### Fixed

- Skip serializing regions in ZoneRecordPayload if None (dnsimple/dnsimple-rust#67)

## 3.0.0 - 2025-05-14

### Changed

- `DomainCollaborators` have been removed. Please use our Domain Access Control feature.
- Drop support for Rust < 1.86
- Add support for Rust 1.86

## 2.0.0 - 2024-12-12

### Changed

- Deprecated `from` and `to` fields in `EmailForward`
- Drop support for Rust < 1.83
- Add support for Rust 1.83
- `DomainCollaborators` have been deprecated and will be removed in the next major version. Please use our Domain Access Control feature.

## 1.0.0 - 2024-03-12

### Changed

- Drop support for Rust < 1.76
- Add support for Rust 1.76

## 0.7.0 - 2024-01-16

### Added

- Added `Registrar::check_registrant_change` to retrieves the requirements of a registrant change. (#51)
- Added `Registrar::get_registrant_change` to retrieves the details of an existing registrant change. (#51)
- Added `Registrar::create_registrant_change` to start registrant change. (#51)
- Added `Registrar::list_registrant_changes` to lists the registrant changes for a domain. (#51)
- Added `Registrar::delete_registrant_change` to cancel an ongoing registrant change from the account. (#51)
- Added `Registrar::enable_domain_transfer_lock` to enable the domain transfer lock for a domain. (#50)
- Added `Registrar::disable_domain_transfer_lock` to disable the domain transfer lock for a domain. (#50)
- Added `Registrar::get_domain_transfer_lock` to get the domain transfer lock status for a domain. (#50)

## 0.6.0 - 2023-12-13

### Added

- Added `secondary`, `last_transferred_at`, `active` to `Zone` (dnsimple/dnsimple-rust#47)

## 0.5.0 - 2023-08-10

### Added

- Added `Zones::activate_dns` to activate DNS services (resolution) for a zone. (dnsimple/dnsimple-rust#44)
- Added `Zones::deactivate_dns` to deactivate DNS services (resolution) for a zone. (dnsimple/dnsimple-rust#44)

## 0.4.0 - 2023-03-06

### Added

- Support the `signature_algorithm` parameter in Let's Encrypt APIs
- Support the `get_domain_registration` and `get_domain_renewal` Registrar APIs

### Changed

- Dependency updates

## 0.3.0 - 2023-01-20

### Changed

- Sets Edition to 2021 (dnsimple/dnsimple-rust#34)
- Update Ureq dependency to 2.6 (dnsimple/dnsimple-rust#34)

### Fixed

- Fix serialized name of ZoneRecordPayload::record_type (dnsimple/dnsimple-rust#32)
- Fix Clippy Lint explicit_auto_deref (dnsimple/dnsimple-rust#33)

## 0.2.1 - 2022-09-21

### Changed

- Expose specific model errors on validation error (dnsimple/dnsimple-rust#27)

## 0.2.0 - 2022-06-15

### Changed

- Removed unwrap() from the codebase
- Functions return Result<T,DNSimpleError> now

### Deprecated

- Deprecate Certificate's `contact_id` (dnsimple/dnsimple-rust#23)

## 0.1.3 - 2022-01-26

### Fixed

- Bug preventing authorized requests (POST, PUT, PATCH)

## 0.1.2 - 2021-12-16

### Fixed

- Clippy, RustFmt, Cargo cleanup

## 0.1.1 - 2021-12-07

### Fixed

- Renames the project from `dnsimple_rust` to `dnsimple`

## 0.1.0 - 2021-12-07

### Added

- Automatic releasing when tagging a release
- First implementation of the dnsimple-rust client (including all the endpoints)
