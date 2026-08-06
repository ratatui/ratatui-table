set shell := ["bash", "-uc"]
set default-list := true

fmt:
    RUSTFMT="$(rustup which --toolchain nightly rustfmt)" cargo fmt --all

fmt-check:
    RUSTFMT="$(rustup which --toolchain nightly rustfmt)" cargo fmt --all -- --check

clippy toolchain="stable":
    cargo +{{toolchain}} clippy --all-targets --all-features -- -D warnings

clippy-stable:
    just clippy stable

clippy-beta:
    just clippy beta

clippy-all: clippy-stable clippy-beta

test:
    cargo test --all-features
    cargo test --no-default-features
    cargo test --no-default-features --features serde

docs:
    RUSTC="$(rustup which --toolchain nightly rustc)" \
      RUSTDOC="$(rustup which --toolchain nightly rustdoc)" \
      RUSTDOCFLAGS="--cfg docsrs -D warnings" cargo doc --all-features --no-deps

rdme:
    cargo rdme

rdme-check:
    cargo rdme --check

deny:
    cargo deny check advisories
    cargo deny check bans licenses sources

minimal-versions:
    cargo minimal-versions check --direct --lib

semver-checks:
    cargo semver-checks

package:
    cargo package --locked --allow-dirty

check-all: fmt-check clippy-all test docs rdme-check deny minimal-versions semver-checks package
