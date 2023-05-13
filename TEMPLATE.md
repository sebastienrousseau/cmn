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

[0]: https://minifunctions.com/ "MiniFunctions"
[1]: https://cmnlib.one "Common (CMN) Library Website"
[2]: https://opensource.org/license/apache-2-0/ "Apache License, Version 2.0"
[4]: https://github.com/sebastienrousseau/cmn/issues "Issues"
[5]: https://github.com/sebastienrousseau/cmn/blob/main/CONTRIBUTING.md "Contributing Instructions"
[8]: https://crates.io/crates/cmn "Crates.io"
[9]: https://docs.rs/cmn "Docs.rs"
[10]: https://lib.rs/crates/cmn "Lib.rs"
[14]: https://www.rust-lang.org "The Rust Programming Language"
[15]: https://codecov.io/gh/sebastienrousseau/cmn "Codecov"

[banner]: https://kura.pro/cmn/images/titles/title-cmn.svg 'Common (CMN) banner'
[codecov-badge]: https://img.shields.io/codecov/c/github/sebastienrousseau/cmn?style=for-the-badge&token=0FZQGHLMOP 'Codecov'
[crates-badge]: https://img.shields.io/crates/v/cmn.svg?style=for-the-badge 'Crates.io badge'
[divider]: https://kura.pro/common/images/elements/divider.svg "divider"
[docs-badge]: https://img.shields.io/docsrs/cmn.svg?style=for-the-badge 'Docs.rs badge'
[libs-badge]: https://img.shields.io/badge/lib.rs-v0.0.3-orange.svg?style=for-the-badge 'Lib.rs badge'
[license-badge]: https://img.shields.io/crates/l/cmn.svg?style=for-the-badge 'License badge'
[made-with-rust-badge]: https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust 'Made With Rust badge'

## Changelog 📚
