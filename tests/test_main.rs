#![allow(missing_docs)]
// Copyright © 2023-2026 Common (CMN) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[cfg(test)]
mod tests {
    use assert_cmd::prelude::*;
    use std::process::Command;

    // ---------------------------------------------------------------
    // Binary execution — CMN_TEST_MODE=1 triggers error path
    // ---------------------------------------------------------------

    #[test]
    fn binary_test_mode_exits_with_error() {
        let output = Command::cargo_bin("cmn")
            .unwrap()
            .env("CMN_TEST_MODE", "1")
            .output()
            .expect("Failed to execute command");

        assert!(!output.status.success());
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(
            stderr.contains("Error running cmn: Simulated error"),
            "Expected simulated error in stderr, got: {stderr}"
        );
    }

    // ---------------------------------------------------------------
    // Binary execution — normal mode prints welcome
    // ---------------------------------------------------------------

    #[test]
    fn binary_normal_mode_prints_welcome() {
        let output = Command::cargo_bin("cmn")
            .unwrap()
            .output()
            .expect("Failed to execute command");

        assert!(output.status.success());
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains("Welcome to `CMN`"));
        assert!(stdout.contains(
            "A Rust library for accessing a collection of mathematical and cryptographic constants."
        ));
    }

    // ---------------------------------------------------------------
    // Binary execution — CMN_TEST_MODE=0 is not error mode
    // ---------------------------------------------------------------

    #[test]
    fn binary_test_mode_zero_succeeds() {
        let output = Command::cargo_bin("cmn")
            .unwrap()
            .env("CMN_TEST_MODE", "0")
            .output()
            .expect("Failed to execute command");

        assert!(output.status.success());
    }
}
