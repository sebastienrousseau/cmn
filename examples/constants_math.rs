// Copyright © 2023-2026 Common (CMN) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Demonstrates all mathematical constants in the CMN library.

use cmn::constants::*;

fn main() {
    println!("=== Core Mathematical Constants ===\n");

    println!("PI             = {PI}");
    println!("TAU            = {TAU}");
    println!("EULER (e)      = {EULER}");
    println!("PHI (golden)   = {PHI}");
    println!("GAMMA (E-M)    = {GAMMA}");
    println!("SQRT2          = {SQRT2}");
    println!("SQRT3          = {SQRT3}");
    println!("SQRT5          = {SQRT5}");

    println!("\n=== Series & Special Constants ===\n");

    println!("APERY (zeta3)  = {APERY}");
    println!("CATALAN        = {CATALAN}");
    println!("KHINCHIN       = {KHINCHIN}");
    println!("GLAISHER_KINK. = {GLAISHER_KINKELIN}");
    println!("SILVER_RATIO   = {SILVER_RATIO}");

    println!("\n=== Logarithmic Constants ===\n");

    println!("LN_2           = {LN_2}");
    println!("LN_10          = {LN_10}");
    println!("LOG2_E         = {LOG2_E}");
    println!("LOG10_E        = {LOG10_E}");

    println!("\n=== Pi Fractions ===\n");

    println!("FRAC_PI_2      = {FRAC_PI_2}  (90°)");
    println!("FRAC_PI_3      = {FRAC_PI_3}  (60°)");
    println!("FRAC_PI_4      = {FRAC_PI_4}  (45°)");
    println!("FRAC_PI_6      = {FRAC_PI_6}  (30°)");
    println!("FRAC_PI_8      = {FRAC_PI_8}  (22.5°)");
    println!("FRAC_1_PI      = {FRAC_1_PI}");
    println!("FRAC_2_PI      = {FRAC_2_PI}");
    println!("FRAC_2_SQRT_PI = {FRAC_2_SQRT_PI}");
    println!("FRAC_1_SQRT_2  = {FRAC_1_SQRT_2}");

    println!("\n=== Identity Checks ===\n");

    println!(
        "PHI^2 == PHI+1 ? {}",
        (PHI * PHI - (PHI + 1.0)).abs() < 1e-14
    );
    println!(
        "SQRT2^2 == 2   ? {}",
        (SQRT2 * SQRT2 - 2.0).abs() < 1e-15
    );
    println!("TAU == 2*PI    ? {}", (TAU - 2.0 * PI).abs() < 1e-15);
    println!("e^LN_2 == 2    ? {}", (LN_2.exp() - 2.0).abs() < 1e-14);
    println!(
        "LOG2_E*LN_2==1 ? {}",
        (LOG2_E * LN_2 - 1.0).abs() < 1e-15
    );
}
