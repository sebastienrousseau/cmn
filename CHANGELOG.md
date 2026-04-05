# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

[0.0.5]: https://github.com/sebastienrousseau/cmn/compare/v0.0.4...v0.0.5
[0.0.4]: https://github.com/sebastienrousseau/cmn/releases/tag/v0.0.4
