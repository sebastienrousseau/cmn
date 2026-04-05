# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.0.6] - 2026-04-05

### Added

- 66 new constants (121 total): particle masses (muon, tau, deuteron,
  triton, helion, alpha), mass ratios, magnetic moments and g-factors,
  eV equivalents, Compton wavelengths, radiation constants, Josephson
  and von Klitzing constants, Hartree energy, Planck units (mass,
  length, time, temperature, charge), molar/thermodynamic constants,
  electromagnetic constants, atomic unit conversions, W/Z/Higgs boson
  masses (PDG 2022), particle masses in MeV/c², reduced Compton
  wavelengths, gas constant in L·atm/(mol·K)
- `Category` enum on `CONSTANTS_TABLE` for filtering
  (Mathematical/Physical/Cryptographic)
- `DateTime::now()`, `from_unix_timestamp()`, `add_seconds()`,
  `add_hours()`, `add_days()`
- `impl TryFrom<&str>` and `impl From<SystemTime>` for `DateTime`
- `impl Display` for `ConstantValue` (all 5 variants)
- `impl FromIterator<String>` for `Words`
- 6 example files: constants_math, constants_physical,
  constants_lookup, datetime_demo, words_demo, macros_demo
- `#[cfg_attr(docsrs, doc(cfg(feature = "std")))]` on all
  std-gated items for docs.rs feature badges

### Changed

- `CONSTANTS_TABLE` now includes `Category` metadata (was flat tuples)
- 8 std-dependent macros gated behind `cfg(feature = "std")`; 7 remain
  `no_std`-compatible
- `Common::words()` gracefully handles malformed JSON instead of
  panicking
- `Common::parse()` uses `is_some_and()` instead of `.unwrap()`
- Removed `eprintln` from library code
- `unsafe_code` lint changed from `"allow"` to `"forbid"` in Cargo.toml
- Removed conflicting `exclude`/`include` in Cargo.toml
- Removed unused `commons` git dependency
- Updated `deny.toml` for cargo-deny 0.19 schema
- Rewrote SECURITY.md with contact, versions table, security posture

### Fixed

- CI: removed invalid `coverage-threshold` input (startup_failure)
- CI: applied `cargo fmt` for max_width = 72
- CI: added `#![allow(missing_docs)]` to test crates
- CI: bumped `actions/checkout` from v4 to v6
- CI: fixed `severity-threshold` from `medium` to `moderate`
- CI: added `main` to GitHub Pages deployment branch policy

## [0.0.5] - 2026-04-05

### Added

- 40 new constants (total: 55):
  - Mathematical: APERY, CATALAN, GLAISHER_KINKELIN, KHINCHIN,
    SILVER_RATIO, LN_2, LN_10, LOG2_E, LOG10_E, FRAC_1_SQRT_2,
    FRAC_1_PI, FRAC_2_PI, FRAC_2_SQRT_PI, FRAC_PI_2, FRAC_PI_3,
    FRAC_PI_4, FRAC_PI_6, FRAC_PI_8
  - Physical: COULOMB, FARADAY, GAS_CONSTANT, GRAVITATIONAL_CONSTANT,
    PLANCK_REDUCED, SPEED_OF_LIGHT, VACUUM_PERMEABILITY,
    VACUUM_PERMITTIVITY, ELEMENTARY_CHARGE, ELECTRON_MASS,
    PROTON_MASS, NEUTRON_MASS, STEFAN_BOLTZMANN, WIEN_DISPLACEMENT,
    STANDARD_GRAVITY, STANDARD_ATMOSPHERE, ATOMIC_MASS_UNIT,
    BOHR_RADIUS, FINE_STRUCTURE, RYDBERG, MAGNETIC_FLUX_QUANTUM,
    CONDUCTANCE_QUANTUM
- `ConstantValue` enum for typed constant access (`Float`, `String`,
  `U32`, `Usize`, `CharArray`)
- `Constants::get_value()` method for typed retrieval
- `Constants::is_valid()` validation method
- `Words` module with `HashSet`-backed word management
  (`add_word`, `remove_word`, `contains`, `clear`, `count`)
- Built-in `WORD_LIST` constant with curated word dictionary
- 14 utility macros (`cmn_max!`, `cmn_min!`, `cmn_vec!`, `cmn_map!`,
  `cmn_in_range!`, `cmn_join!`, `cmn_split!`, `cmn_to_num!`,
  `cmn_parse!`, `cmn_print!`, `cmn_print_vec!`, `cmn_contains!`,
  `cmn_assert!`, `cmn_constants!`)
- `datetime` module: ISO 8601 parsing, relative time formatting
  ("3 hours ago"), duration calculations, timezone offset support
  (closes #64)
- `CONTRIBUTING.md` with signed-commit policy and PR guidelines
- `CHANGELOG.md`
- 50-test regression suite (`test_regression.rs`)
- Mermaid architecture diagram in README

### Changed

- Bumped MSRV from 1.60 to 1.72
- Updated dependencies: serde 1.0.228, serde_json 1.0.149,
  assert_cmd 2.2.0, criterion 0.8.2
- Migrated `criterion::black_box` to `std::hint::black_box`
- Changed `Constants::constants()` return type from `&Vec<Constant>`
  to `&[Constant]`
- Pre-allocated `Constants` vector with `Vec::with_capacity(28)`
- Replaced `HashSet::from_iter` with idiomatic `.collect()`
- Simplified `cmn_in_range!` macro (removed needless `if/else`)
- Consolidated CI into single reusable workflow pipeline
- Rewrote README with cross-platform setup, constants table,
  architecture diagram
- Rewrote crate-level documentation for clarity and accuracy

### Removed

- Removed obsolete lints (`pointer_structural_match`,
  `missing_fragment_specifier`) — now hard errors in rustc
- Removed dead `#![cfg_attr(feature = "bench", feature(test))]`
- Removed unreachable `Category::Io` error branch in `Common::parse()`
- Removed legacy per-workflow CI files (audit, check, coverage,
  document, lint, release, test)

### Fixed

- Fixed `keyword_idents` lint group priority conflict in Cargo.toml
- Fixed stale `v0.0.4` version badges in documentation
- Fixed `///` doc-comment on local variable in example
- Removed stale review comments (`// Add this line`, `// Remove this line`)
- Updated copyright years to 2023-2026

## [0.0.4] - 2023-12-01

### Added

- Initial public release with 15 constants (EULER, PHI, PI, PLANCK,
  SQRT2, SQRT3, SQRT5, TAU, AVOGADRO, BOLTZMANN, GAMMA,
  HASH_ALGORITHM, HASH_COST, HASH_LENGTH, SPECIAL_CHARS)
- `Common` struct with JSON-backed `serde` layer
- `Constants` struct with `constant()` and `constants()` methods
- Basic macro set (`cmn!`, `cmn_parse!`)

[0.0.6]: https://github.com/sebastienrousseau/cmn/compare/v0.0.5...v0.0.6
[0.0.5]: https://github.com/sebastienrousseau/cmn/compare/v0.0.4...v0.0.5
[0.0.4]: https://github.com/sebastienrousseau/cmn/releases/tag/v0.0.4
