<!-- markdownlint-disable MD033 MD041 -->

<img src="https://kura.pro/cmn/images/logos/cmn.svg"
alt="Common (CMN) logo" height="261" width="261" align="right" />

<!-- markdownlint-enable MD033 MD041 -->

# Common (CMN)

A Rust library for accessing a collection of mathematical and cryptographic constants

*Part of the [Mini Functions][0] family of libraries.*

<!-- markdownlint-disable MD033 MD041 -->
<center>
<!-- markdownlint-enable MD033 MD041 -->

![Common (CMN) Banner][banner]

[![Made With Rust][made-with-rust-badge]][14] [![Crates.io][crates-badge]][8] [![Lib.rs][libs-badge]][10] [![Docs.rs][docs-badge]][9] [![License][license-badge]][2] [![Codecov][codecov-badge]][15]

• [Website][1] • [Documentation][9] • [Report Bug][4] • [Request Feature][4] • [Contributing Guidelines][5]

<!-- markdownlint-disable MD033 MD041 -->
</center>
<!-- markdownlint-enable MD033 MD041 -->

![divider][divider]

## Overview 📖

`Common (CMN)`, a Rust library designed for developers who are looking for a comprehensive collection of mathematical and cryptographic constants.

`Common (CMN)` is a modern, fast, and user-friendly library that makes it easy to access a wide range of mathematical and cryptographic constants, including the mathematical constant `Euler`, the `hash` algorithm used, the `cost` of the hash algorithm, the `length` of the hash, the mathematical constant `Phi`, the mathematical constant `Pi`, the `Planck` constant, a set of `special` characters, and much more.

## Features ✨

The library includes two main structures: `Constant` and `Constants`.

- The `Constant` structure holds the name and value of each constant as
a `&'static str` and a `String`, respectively.
- The Constants structure implements a method constants that returns a
`Vec<Constant>` containing all the available constants.
- The available constants include the mathematical constants `EULER`,
`PHI`, `PI`, `PLANCK`, and `SQRT5`, and the cryptographic constants
`HASH_ALGORITHM`, `HASH_COST`, `HASH_LENGTH`, and `SPECIAL_CHARS`.
- The library also includes an enumeration `ConstantValue` that
represents the different constant values. The values can be an
`f64 float`, a `String`, a `u32`, a `usize`, or a `&'static [char]`
array of characters.

## Getting Started 🚀

It takes just a few minutes to get up and running with `Common (CMN)`.

### Installation

To install `Common (CMN)`, you need to have the Rust toolchain installed on your machine. You can install the Rust toolchain by following the instructions on the [Rust website][14].

Once you have the Rust toolchain installed, you can install `Common (CMN)` using the following command:

```shell
cargo install cmn
```

You can then run the help command to see the available options:

```shell
cmn --help
```

### Requirements

The minimum supported Rust toolchain version is currently Rust **1.69.0** or later (stable). It is recommended that you install the latest stable version of Rust.

### Platform support

`Common (CMN)` is supported and tested on the following platforms:

#### Tier 1 platforms 🏆

| Operating System | Target | Description |
| --- | --- | --- |
| Linux   | aarch64-unknown-linux-gnu | 64-bit Linux systems on ARM architecture |
| Linux   | i686-unknown-linux-gnu | 32-bit Linux (kernel 3.2+, glibc 2.17+) |
| Linux   | x86_64-unknown-linux-gnu | 64-bit Linux (kernel 2.6.32+, glibc 2.11+) |
| macOS   | x86_64-apple-darwin | 64-bit macOS (10.7 Lion or later) |
| Windows | i686-pc-windows-gnu | 32-bit Windows (7 or later) |
| Windows | i686-pc-windows-msvc | 32-bit Windows (7 or later) |
| Windows | x86_64-pc-windows-gnu | 64-bit Windows (7 or later) |
| Windows | x86_64-pc-windows-msvc | 64-bit Windows (7 or later) |

#### Tier 2 platforms 🥈

| Operating System | Target | Description |
| --- | --- | --- |
| 64-bit Linux | x86_64-unknown-linux-musl | 64-bit Linux (kernel 2.6.32+, musl libc) |
| ARM64 Linux | aarch64-unknown-linux-musl | 64-bit Linux systems on ARM architecture |
| ARM64 macOS | aarch64-apple-darwin | 64-bit macOS on Apple Silicon |
| ARM64 Windows | aarch64-pc-windows-msvc | 64-bit Windows (aarch64-pc-windows-msvc) |
| ARMv6 Linux | arm-unknown-linux-gnueabi | ARMv6 Linux (kernel 3.2, glibc 2.17) |
| ARMv6 Linux, hardfloat | arm-unknown-linux-gnueabihf | ARMv7 Linux, hardfloat (kernel 3.2, glibc 2.17) |
| ARMv7 Linux, hardfloat | armv7-unknown-linux-gnueabihf | ARMv7 Linux, hardfloat (kernel 3.2, glibc 2.17) |
| FreeBSD  | x86_64-unknown-freebsd | 64-bit FreeBSD on x86-64 |
| MIPS (LE) Linux | mipsel-unknown-linux-gnu | MIPSel Linux (kernel 2.6.32+, glibc 2.11+) |
| MIPS Linux | mips-unknown-linux-gnu | MIPS Linux (kernel 2.6.32+, glibc 2.11+) |
| MIPS64 (LE) Linux | mips64el-unknown-linux-gnuabi64 | MIPS64el Linux (kernel 2.6.32+, glibc 2.11+) |
| MIPS64 Linux | mips64-unknown-linux-gnuabi64 | MIPS64 Linux (kernel 2.6.32+, glibc 2.11+) |
| NetBSD  | x86_64-unknown-netbsd | 64-bit NetBSD on x86-64 |
| PowerPC Linux | powerpc-unknown-linux-gnu | PowerPC Linux (kernel 3.2, glibc 2.17) |
| PPC64 Linux | powerpc64-unknown-linux-gnu | PowerPC64 Linux (kernel 3.2, glibc 2.17) |
| PPC64LE Linux | powerpc64le-unknown-linux-gnu | PowerPC64le Linux (kernel 3.2, glibc 2.17) |
| RISC-V Linux | riscv64gc-unknown-linux-gnu | RISC-V Linux (kernel 3.2, glibc 2.17) |
| S390x Linux | s390x-unknown-linux-gnu | s390x Linux (kernel 3.2, glibc 2.17) |

The [GitHub Actions][11] shows the platforms in which the `Common (CMN)` library tests are run.

### Documentation

> ℹ️ **Info:** Please check out our [website][1] for more information.
You can find our documentation on [docs.rs][9], [lib.rs][10] and [crates.io][8].

## Usage 📖

To use the `Common (CMN)` library in your project, add the following to your `Cargo.toml` file:

```toml
[dependencies]
cmn = "0.0.3"
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

For transparency into our release cycle and in striving to maintain backward compatibility, `Common (CMN)` follows [semantic versioning][7].

## License 📝

The project is licensed under the terms of both the MIT license and the Apache License (Version 2.0).

- [Apache License, Version 2.0][2]
- [MIT license][3]

## Contribution 🤝

We welcome all people who want to contribute. Please see the [contributing instructions][5] for more information.

Contributions in any form (issues, pull requests, etc.) to this project must adhere to the [Rust's Code of Conduct][12].

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Acknowledgements 💙

A big thank you to all the awesome contributors of the [Common (CMN) Library][6] for their help and support.

A special thank you goes to the [Rust Reddit][13] community for providing a lot of useful suggestions on how to improve this project.

[0]: https://minifunctions.com/ "MiniFunctions"
[1]: https://cmnlib.one "Common (CMN) Library Website"
[2]: https://opensource.org/license/apache-2-0/ "Apache License, Version 2.0"
[3]: https://opensource.org/licenses/MIT "MIT license"
[4]: https://github.com/sebastienrousseau/cmn/issues "Issues"
[5]: https://github.com/sebastienrousseau/cmn/blob/main/CONTRIBUTING.md "Contributing Instructions"
[6]: https://github.com/sebastienrousseau/cmn/graphs/contributors "Contributors"
[7]: http://semver.org/ "Semantic Versioning"
[8]: https://crates.io/crates/cmn "Crates.io"
[9]: https://docs.rs/cmn "Docs.rs"
[10]: https://lib.rs/crates/cmn "Lib.rs"
[11]: https://github.com/sebastienrousseau/cmn/actions "GitHub Actions"
[12]: https://www.rust-lang.org/policies/code-of-conduct "Rust's Code of Conduct"
[13]: https://reddit.com/r/rust "Rust Reddit"
[14]: https://www.rust-lang.org "The Rust Programming Language"
[15]: https://codecov.io/gh/sebastienrousseau/cmn "Codecov"

[banner]: https://kura.pro/cmn/images/titles/title-cmn.svg 'Common (CMN) banner'
[codecov-badge]: https://img.shields.io/codecov/c/github/sebastienrousseau/cmn?style=for-the-badge&token=0FZQGHLMOP 'Codecov'
[crates-badge]: https://img.shields.io/crates/v/cmn.svg?style=for-the-badge 'Crates.io badge'
[divider]: https://kura.pro/common/images/elements/divider.svg "divider"
[docs-badge]: https://img.shields.io/docsrs/cmn.svg?style=for-the-badge 'Docs.rs badge'
[libs-badge]: https://img.shields.io/badge/lib.rs-v0.0.4-orange.svg?style=for-the-badge 'Lib.rs badge'
[license-badge]: https://img.shields.io/crates/l/cmn.svg?style=for-the-badge 'License badge'
[made-with-rust-badge]: https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust 'Made With Rust badge'
