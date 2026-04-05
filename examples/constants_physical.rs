// Copyright © 2023-2026 Common (CMN) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Demonstrates all physical constants and their relationships.

use cmn::constants::*;

fn main() {
    println!("=== Fundamental Constants ===\n");

    println!("SPEED_OF_LIGHT     = {SPEED_OF_LIGHT} m/s");
    println!("PLANCK             = {PLANCK} J·s");
    println!("PLANCK_REDUCED     = {PLANCK_REDUCED} J·s");
    println!("ELEMENTARY_CHARGE  = {ELEMENTARY_CHARGE} C");
    println!("BOLTZMANN          = {BOLTZMANN} J/K");
    println!("AVOGADRO           = {AVOGADRO} /mol");
    println!(
        "GRAVITATIONAL_CST  = {GRAVITATIONAL_CONSTANT} m³/(kg·s²)"
    );
    println!("FINE_STRUCTURE     = {FINE_STRUCTURE}");

    println!("\n=== Derived Constants ===\n");

    println!("GAS_CONSTANT       = {GAS_CONSTANT} J/(mol·K)");
    println!("FARADAY            = {FARADAY} C/mol");
    println!("STEFAN_BOLTZMANN   = {STEFAN_BOLTZMANN} W/(m²·K⁴)");
    println!("RYDBERG            = {RYDBERG} /m");
    println!("BOHR_RADIUS        = {BOHR_RADIUS} m");

    println!("\n=== Electromagnetic ===\n");

    println!("COULOMB            = {COULOMB} N·m²/C²");
    println!("VACUUM_PERMEAB.    = {VACUUM_PERMEABILITY} N/A²");
    println!("VACUUM_PERMITT.    = {VACUUM_PERMITTIVITY} F/m");
    println!("MAG_FLUX_QUANTUM   = {MAGNETIC_FLUX_QUANTUM} Wb");
    println!("CONDUCT_QUANTUM    = {CONDUCTANCE_QUANTUM} S");

    println!("\n=== Particle Masses ===\n");

    println!("ELECTRON_MASS      = {ELECTRON_MASS} kg");
    println!("PROTON_MASS        = {PROTON_MASS} kg");
    println!("NEUTRON_MASS       = {NEUTRON_MASS} kg");
    println!("ATOMIC_MASS_UNIT   = {ATOMIC_MASS_UNIT} kg");

    println!("\n=== Thermodynamic ===\n");

    println!("WIEN_DISPLACEMENT  = {WIEN_DISPLACEMENT} m·K");
    println!("STANDARD_GRAVITY   = {STANDARD_GRAVITY} m/s²");
    println!("STANDARD_ATMOSPH.  = {STANDARD_ATMOSPHERE} Pa");

    println!("\n=== Relationship Checks ===\n");

    let r_computed = BOLTZMANN * AVOGADRO;
    println!(
        "R = k_B * N_A      : {r_computed:.6} \
         (expected {GAS_CONSTANT})"
    );

    let f_computed = AVOGADRO * ELEMENTARY_CHARGE;
    println!(
        "F = N_A * e        : {f_computed:.5} \
         (expected {FARADAY})"
    );

    let em_product = VACUUM_PERMEABILITY
        * VACUUM_PERMITTIVITY
        * SPEED_OF_LIGHT
        * SPEED_OF_LIGHT;
    println!(
        "mu0*eps0*c^2 == 1  ? {} (got {em_product})",
        (em_product - 1.0).abs() < 1e-4
    );

    let flux_computed = PLANCK / (2.0 * ELEMENTARY_CHARGE);
    println!(
        "Phi0 = h/(2e)      : {flux_computed:.9e} \
         (expected {MAGNETIC_FLUX_QUANTUM})"
    );

    let ratio = PROTON_MASS / ELECTRON_MASS;
    println!("m_p/m_e            : {ratio:.2} (expected ~1836.15)");

    println!("\n=== Uncertainties ===\n");

    println!(
        "G uncertainty       = ±{GRAVITATIONAL_CONSTANT_UNCERTAINTY}"
    );
    println!("m_e uncertainty     = ±{ELECTRON_MASS_UNCERTAINTY}");
    println!("alpha uncertainty   = ±{FINE_STRUCTURE_UNCERTAINTY}");
}
