# Security Policy

## Reporting Security Issues

**Do not open a public GitHub issue for security vulnerabilities.**

Instead, please report them via
[GitHub Security Advisories](https://github.com/sebastienrousseau/cmn/security/advisories/new)
or email **security@cmnlib.com**.

Please include:

- Type of issue (e.g. panic on untrusted input, logic error, etc.)
- Full paths of source file(s) related to the issue
- The location of the affected source code (tag/branch/commit or URL)
- Any special configuration required to reproduce the issue
- Step-by-step instructions to reproduce the issue
- Proof-of-concept or exploit code (if possible)
- Impact of the issue, including how an attacker might exploit it

We will acknowledge receipt within **48 hours** and aim to provide a
fix or mitigation within **7 days**.

## Supported Versions

| Version | Supported |
|:--------|:----------|
| 0.0.5   | Yes       |
| < 0.0.5 | No        |

## Security Posture

- `#![forbid(unsafe_code)]` enforced crate-wide
- Zero `unsafe` blocks in all source code
- `cargo deny` configured for license audit, advisory checks,
  and duplicate dependency detection
- All dependencies are optional (`serde`, `serde_json`)
- 100% test coverage with mathematical identity verification
