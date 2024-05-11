<!-- markdownlint-disable MD033 MD041 -->

<img src="https://kura.pro/cmn/images/logos/cmn.svg"
alt="Common (CMN) logo" height="261" width="261" align="right" />

<!-- markdownlint-enable MD033 MD041 -->

# Common (CMN)

A Rust library for accessing a collection of mathematical and cryptographic constants

*Part of the [Mini Functions][00] family of libraries.*

<!-- markdownlint-disable MD033 MD041 -->
<center>
<!-- markdownlint-enable MD033 MD041 -->

![Common (CMN) Banner][banner]

[![Made With Rust][made-with-rust-badge]][13] [![Crates.io][crates-badge]][08] [![Lib.rs][libs-badge]][10] [![Docs.rs][docs-badge]][09] [![License][license-badge]][02] [![Codecov][codecov-badge]][14]

• [Website][01] • [Documentation][09] • [Report Bug][04] • [Request Feature][04] • [Contributing Guidelines][05]

<!-- markdownlint-disable MD033 MD041 -->
</center>
<!-- markdownlint-enable MD033 MD041 -->

![divider][divider]

## Overview 📖

`Common (CMN)` is a modern, fast, and user-friendly library that makes it easy to access a wide range of mathematical and cryptographic constants.

## Features ✨

The `Common (CMN)` uses the `serde` crate to serialize and deserialize the data.

The library has three modules:

- **Macros**: This module contains functions for generating macros that can be used to access the constants and words.
- **Constants**: This module contains the `Constants` structure, which provides a collection of constant values that are used throughout the library.
- **Words**: This module contains the `Words` structure, which provides a collection of words that are used throughout the library.

### Mathematical and Cryptographic Constants

The following table lists the most important mathematical and cryptographic constants available in the `Common (CMN)` library:

| Constants | Description | Example |
| --- | --- | --- |
| APERY | Apéry's constant, which is the sum of the reciprocals of the positive cubes. `ζ(3) ≈ 1.2020569032` | Used in various mathematical calculations and series approximations. |
| AVOGADRO | Avogadro's constant is the number of atoms or molecules in one mole of a substance. `N_A ≈ 6.02214076 x 10^23 mol^-1` | The number of atoms in 12 grams of carbon-12 is 6.02214076 × 10^23. This can be used to calculate the number of atoms or molecules in a given sample. |
| BOLTZMANN | Boltzmann's constant is the physical constant relating the temperature of a system to the average kinetic energy of its constituent particles. `k_B ≈ 1.380648 x 10^-23 J K^-1` | The kinetic energy of an atom at room temperature is about 2.0 × 10^-21 joules. This can be used to calculate the temperature of a system, or to calculate the average kinetic energy of its particles. |
| CATALAN | Catalan's constant, which is the sum of the alternating harmonic series. `C ≈ 0.915965594177219` | Used in various mathematical calculations and series approximations. |
| COULOMB | Coulomb's constant, which is the proportionality constant in Coulomb's law. `k_e ≈ 8.9875517923` x 10^9 N m^2 C^-2 | Used in calculations related to electrostatic forces and electric fields. |
| EULER | Euler's constant is a mathematical constant approximately equal to 2.71828. `e ≈ 2.7182818284590452353602874713527` | The sum of the infinite series 1 + 1/2 + 1/3 + ... is equal to Euler's constant, e. This can be used to calculate the sum of an infinite series, or to calculate the logarithm of a number. |
| FARADAY | Faraday constant, which represents the amount of electric charge carried by one mole of electrons. `F ≈ 96485.33212 C mol^-1` | Used in calculations related to electrochemistry and electrolysis. |
| GAMMA | The gamma constant is a mathematical constant approximately equal to 0.57721. `γ ≈ 0.5772156649015329` | The gamma function of 2 is equal to 1. This can be used to calculate the gamma function of a number, or to calculate the factorial of a number. |
| GAS_CONSTANT | The gas constant, which relates the energy scale to the temperature scale in the ideal gas law. `R ≈ 8.314462618 J mol^-1 K^-1` | Used in calculations related to the behavior of gases and thermodynamics. |
| GLAISHER_KINKELIN | Glaisher-Kinkelin constant, which arises in the asymptotic expansion of the Barnes G-function. `A ≈ 1.2824271291` | Used in various mathematical calculations and series approximations. |
| GRAVITATIONAL_CONSTANT | The gravitational constant, which is the proportionality constant in Newton's law of universal gravitation. `G ≈ 6.67430 x 10^-11 m^3 kg^-1 s^-2` | Used in calculations related to gravitational forces and celestial mechanics. |
| HASH_ALGORITHM | The hash algorithm used to generate the hash. The default is Blake3. | The hash of the string "Hello, world!" is 5eb63bbbe01eeed093cb22bb8f5acdc32790160b123138d53f2173b8d3dc3eee. This can be used to verify the integrity of data, or to create a unique identifier for a file. |
| HASH_COST | The cost of the hash. | The hash cost of Blake3 is `2^128`. This can be used to determine how secure a hash algorithm is. |
| HASH_LENGTH | The length of the hash. | The hash length of Blake3 is 32 bytes. This can be used to determine how much space is required to store the hash output. |
| KHINCHIN | Khinchin's constant, which appears in the theory of continued fractions. `K ≈ 2.6854520010` | Used in various mathematical calculations and series approximations. |
| PHI | The golden ratio is a number approximately equal to 1.618033988749895. `φ = (1 + √5) / 2 ≈ 1.6180339887498948482045868343656` | The golden ratio can be used to create a symmetrical design, or a design that is pleasing to the eye. |
| Pi (π) | Pi is the ratio of a circle's circumference to its diameter. `π ≈ 3.14159265358979323846264338327950288` | The circumference of a circle with a radius of 1 is equal to pi. This can be used to calculate the circumference, area, and volume of circles, spheres, and other geometric shapes. |
| PLANCK | Planck's constant, which relates the energy of a photon to its frequency. `h ≈ 6.62607015 x 10^-34 J s` | The energy of a photon of light with a wavelength of 500 nanometers is equal to Planck's constant multiplied by the frequency of the light. This can be used to calculate the energy of photons and other elementary particles. |
| PLANCK_REDUCED | Planck's reduced constant, which is Planck's constant divided by 2π. `ħ = h / (2π) ≈ 1.054571817 x 10^-34 J s` | Used in quantum mechanics and related calculations. |
| SILVER_RATIO | The silver ratio is one of the silver means. `δ_s = 1 + √2 ≈ 2.4142135623730950488016887242097` | The silver ratio can be used to create a symmetrical design, or a design that is pleasing to the eye. |
| SPEED_OF_LIGHT | The speed of light in vacuum. `c ≈ 299792458 m s^-1` | Used in calculations related to relativity and electromagnetic phenomena. |
| SPECIAL_CHARS | A set of special characters. | The special characters are: !@#$%^&*()_+-={}[]|\:;"'<>,.?` |
| SQRT2 | The square root of 2. `√2 ≈ 1.4142135623730950488016887242097` | The area of a circle with a radius of 1 is equal to the square root of 2. This can be used to calculate the area and volume of circles, spheres, and other geometric shapes. |
| SQRT3 | The square root of 3. `√3 ≈ 1.7320508075688772935274463415059` | The area of a circle with a radius of 1 is equal to the square root of 3. This can be used to calculate the area and volume of circles. |
| SQRT5 | The square root of 5. `√5 ≈ 2.23606797749979` | The area of a circle with a radius of 1 is equal to the square root of 5. |
| TAU | The circle constant, which is the ratio of a circle's circumference to its radius. `τ = 2π ≈ 6.28318530717958647692528676655900577` | The circumference of a circle with a radius of 1 is equal to tau. |
| VACUUM_PERMEABILITY | The vacuum permeability, which relates magnetic induction to magnetic field strength. `μ_0 ≈ 1.25663706212 x 10^-6 N A^-2` | Used in calculations related to electromagnetism and magnetic fields. |
| VACUUM_PERMITTIVITY | The vacuum permittivity, which relates electric displacement to electric field strength. `ε_0 ≈ 8.8541878128 x 10^-12 F m^-1` | Used in calculations related to electromagnetism and electric fields. |

## Getting Started 🚀

It takes just a few minutes to get up and running with `Common (CMN)`.

### Installation

To install `Common (CMN)`, you need to have the Rust toolchain installed on your machine. You can install the Rust toolchain by following the instructions on the [Rust website][13].

Once you have the Rust toolchain installed, you can install `Common (CMN)` using the following command:

```shell
cargo install cmn
```

### Requirements

The minimum supported Rust toolchain version is currently Rust **1.60** or later (stable). It is recommended that you install the latest stable version of Rust.

### Platform support

`cmn` supports a variety of CPU architectures. It is supported and tested on MacOS, Linux, and Windows.


### Documentation

> ℹ️ **Info:** Please check out our [website][01] for more information.
You can find our documentation on [docs.rs][09], [lib.rs][10] and [crates.io][08].

## Usage 📖

To use the `Common (CMN)` library in your project, add the following to your `Cargo.toml` file:

```toml
[dependencies]
cmn = "0.0.4"
```

Add the following to your `main.rs` file:

```rust
extern crate cmn;
use cmn::*;
```

then you can use the functions in your application code.

### Examples

`Common (CMN)` comes with a set of examples that you can use to get started. The examples are located in the `examples` directory of the project. To run the examples, clone the repository and run the following command in your terminal from the project root directory.

```shell
cargo run --example cmn
```

## Semantic Versioning Policy 🚥

For transparency into our release cycle and in striving to maintain backward compatibility, `Common (CMN)` follows [semantic versioning][07].

## License 📝

`Common (CMN)`is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE][02] and [LICENSE-MIT][03] for details.

## Contribution 🤝

We welcome all people who want to contribute. Please see the [contributing instructions][05] for more information.

Contributions in any form (issues, pull requests, etc.) to this project must adhere to the [Rust's Code of Conduct][11].

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Acknowledgements 💙

A big thank you to all the awesome contributors of the [Common (CMN) Library][06] for their help and support.

A special thank you goes to the [Rust Reddit][12] community for providing a lot of useful suggestions on how to improve this project.

[00]: https://cmnlib.com/ "MiniFunctions"
[01]: https://cmnlib.one "Common (CMN) Library Website"
[02]: https://opensource.org/license/apache-2-0/ "Apache License, Version 2.0"
[03]: https://opensource.org/licenses/MIT "MIT license"
[04]: https://github.com/sebastienrousseau/cmn/issues "Issues"
[05]: https://github.com/sebastienrousseau/cmn/blob/main/CONTRIBUTING.md "Contributing Instructions"
[06]: https://github.com/sebastienrousseau/cmn/graphs/contributors "Contributors"
[07]: http://semver.org/ "Semantic Versioning"
[08]: https://crates.io/crates/cmn "Crates.io"
[09]: https://docs.rs/cmn "Docs.rs"
[10]: https://lib.rs/crates/cmn "Lib.rs"
[11]: https://www.rust-lang.org/policies/code-of-conduct "Rust's Code of Conduct"
[12]: https://reddit.com/r/rust "Rust Reddit"
[13]: https://www.rust-lang.org "The Rust Programming Language"
[14]: https://codecov.io/gh/sebastienrousseau/cmn "Codecov"

[banner]: https://kura.pro/cmn/images/titles/title-cmn.svg 'Common (CMN) banner'
[codecov-badge]: https://img.shields.io/codecov/c/github/sebastienrousseau/cmn?style=for-the-badge&token=0FZQGHLMOP 'Codecov'
[crates-badge]: https://img.shields.io/crates/v/cmn.svg?style=for-the-badge 'Crates.io badge'
[divider]: https://kura.pro/common/images/elements/divider.svg "divider"
[docs-badge]: https://img.shields.io/docsrs/cmn.svg?style=for-the-badge 'Docs.rs badge'
[libs-badge]: https://img.shields.io/badge/lib.rs-v0.0.4-orange.svg?style=for-the-badge 'Lib.rs badge'
[license-badge]: https://img.shields.io/crates/l/cmn.svg?style=for-the-badge 'License badge'
[made-with-rust-badge]: https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust 'Made With Rust badge'
