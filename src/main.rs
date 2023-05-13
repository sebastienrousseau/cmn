// Copyright © 2023 Common (CMN) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! This is the main entry point for the cmn application.
fn main() {
    // Call the `run()` function from the `Common (CMN)` module.
    if let Err(err) = cmn::run() {
        eprintln!("Error running cmn: {}", err);
        std::process::exit(1);
    }
}
