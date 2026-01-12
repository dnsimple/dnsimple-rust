# Releasing

This document describes the steps to release a new version of DNSimple/Rust.

## Prerequisites

- You have commit access to the repository
- You have push access to the repository
- You have a GPG key configured for signing tags

## Release process

1. **Determine the new version** using [Semantic Versioning](https://semver.org/)

   ```shell
   VERSION=X.Y.Z
   ```

   - **MAJOR** version for incompatible API changes
   - **MINOR** version for backwards-compatible functionality additions
   - **PATCH** version for backwards-compatible bug fixes

2. **Update the version file** with the new version

   Edit `Cargo.toml`:

   ```toml
   version = "$VERSION"
   ```

3. **Run tests** and confirm they pass

   ```shell
   cargo test
   ```

4. **Update the changelog** with the new version

   Finalize the `## main` section in `CHANGELOG.md` assigning the version.

5. **Commit the new version**

   ```shell
   git commit -a -m "Release $VERSION"
   ```

6. **Push the changes**

   ```shell
   git push origin main
   ```

7. **Wait for CI to complete**

8. **Create a signed tag**

   ```shell
   git tag -a v$VERSION -s -m "Release $VERSION"
   git push origin --tags
   ```

   The `release` GitHub action should take it from here and release the package to crates.io.

## Post-release

- Verify the new version appears on [crates.io](https://crates.io/crates/dnsimple)
- Verify the GitHub release was created
- Announce the release if necessary
