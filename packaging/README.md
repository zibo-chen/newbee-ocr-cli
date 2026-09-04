# Package publishing

Stable version tags are built by `cargo-dist` and published to these channels:

- Homebrew: `zibo-chen/homebrew-tap`
- APT/RPM: `zibo-chen/nbocr-packages`
- WinGet: `ChenZibo.NewbeeOCRCLI` in `microsoft/winget-pkgs`

The Homebrew and Linux repositories use separate write-enabled deploy keys. Their
private halves are stored in this repository as Actions secrets:

- `HOMEBREW_TAP_DEPLOY_KEY`
- `LINUX_PACKAGES_DEPLOY_KEY`

Linux packages and repository metadata are signed by the key stored in
`LINUX_SIGNING_KEY`. Its public fingerprint is:

```text
F4FA A579 A551 1320 6B32  634A D785 4FC0 2114 4E05
```

The signing key expires on 2029-09-03 and must be rotated before that date.

Run `dist generate --check`, `cargo fmt --check`, and `cargo test --locked`
before tagging a release. The release workflow builds and uploads the platform
archives and MSI, tests the Homebrew formula, builds and tests DEB/RPM packages,
and updates both package repositories.
