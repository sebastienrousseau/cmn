# Contributing to Common (CMN)

Thank you for your interest in contributing. This guide covers setup, commit policy, and PR hygiene.

## Prerequisites

| Platform | Toolchain |
|:---|:---|
| **macOS** | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |
| **Linux / WSL** | Same as above, plus `sudo apt-get install -y build-essential` (Debian/Ubuntu) |
| **Windows** | Download [rustup-init.exe](https://rustup.rs/) and install the MSVC toolchain |

Minimum Rust version: **1.72**

## Getting Started

```bash
git clone https://github.com/sebastienrousseau/cmn.git
cd cmn
cargo build
cargo test
```

All 198+ tests should pass. If they don't, open an issue before proceeding.

## Signed Commits (Required)

All commits **must** be signed. Unsigned commits will be rejected by CI.

### Setup GPG signing (one-time)

```bash
# Generate a key (if you don't have one)
gpg --full-generate-key

# List your key ID
gpg --list-secret-keys --keyid-format=long

# Tell Git to use it
git config --global user.signingkey <YOUR_KEY_ID>
git config --global commit.gpgsign true

# (macOS only) If using GPG Suite:
echo "pinentry-program /usr/local/bin/pinentry-mac" >> ~/.gnupg/gpg-agent.conf
gpgconf --kill gpg-agent
```

### Setup SSH signing (alternative)

```bash
git config --global gpg.format ssh
git config --global user.signingkey ~/.ssh/id_ed25519.pub
git config --global commit.gpgsign true
```

### Verify your setup

```bash
git commit --allow-empty -m "test: verify signed commit" -S
git log --show-signature -1
```

## Branch Naming

| Type | Pattern | Example |
|:---|:---|:---|
| Feature | `feat/<description>` | `feat/add-ln2-constant` |
| Fix | `fix/<description>` | `fix/phi-precision` |
| Docs | `docs/<description>` | `docs/update-readme` |
| Release | `feat/vX.Y.Z` | `feat/v0.0.5` |

## Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <description>

[optional body]
```

Types: `feat`, `fix`, `docs`, `test`, `refactor`, `build`, `ci`, `chore`

## Pull Request Checklist

Before opening a PR, verify:

- [ ] `cargo fmt --check` passes
- [ ] `cargo clippy --all-targets` has zero warnings
- [ ] `cargo test` passes (all 198+ tests)
- [ ] `cargo doc` builds without warnings
- [ ] New public API items have `///` doc comments
- [ ] All commits are signed (`git log --show-signature`)

## Code Style

- Edition 2021, formatted by `rustfmt` (see `rustfmt.toml`)
- Max line width: 72 characters
- `#![forbid(unsafe_code)]` is enforced crate-wide
- All public items require documentation (`missing_docs = "warn"`)

## Running the Full CI Check Locally

```bash
cargo fmt --check && cargo clippy --all-targets && cargo test && cargo doc --no-deps
```

## License

By contributing, you agree that your contributions will be dual-licensed under [Apache 2.0](LICENSE-APACHE) and [MIT](LICENSE-MIT), without additional terms.
