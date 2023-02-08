extern crate cmn;

use cmn::constants::{Constant, ConstantValue, Constants};
pub use cmn::Words;

fn main() {
    // Create a Constants instance
    let c = Constants {};

    // Retrieve the list of constants
    let constants = c.constants();

    // Serialize the constants for readability
    let serialized = serde_json::to_string_pretty(&constants).unwrap();

    // Print the serialized constants to the console
    println!("🦀 Constants: ✅ {serialized}");

    /// The JSON string to deserialize
    static JSON: &str = r#"[{"name":"EULER","value":"2.718281828459045"},{"name":"PI","value":"3.141592653589793"},{"name":"TAU","value":"6.283185307179586"},{"name":"SQRT2","value":"1.4142135623730951"},{"name":"SQRT1_2","value":"0.7071067811865476"},{"name":"LN2","value":"0.6931471805599453"},{"name":"LN10","value":"2.302585092994046"},{"name":"LOG2E","value":"1.4426950408889634"},{"name":"LOG10E","value":"0.4342944819032518"},{"name":"PHI","value":"1.618033988749895"},{"name":"GOLDEN_RATIO","value":"1.618033988749895"},{"name":"INFINITY","value":"Infinity"},{"name":"NEG_INFINITY","value":"-Infinity"},{"name":"NAN","value":"NaN"}]"#;

    // Deserialize the constants from the JSON string
    let deserialized: Vec<Constant> = serde_json::from_str(JSON).unwrap();

    // Print the deserialized constants
    println!("🦀 Deserialized: ✅ {deserialized:?}");

    // Convert a constant value to ConstantValue
    let pi = ConstantValue::Float(std::f64::consts::PI);
    println!("🦀 ConstantValue of PI: ✅ {pi:?}");

    // Retrieve a constant by name and print it (in this case, EULER)
    let euler_constant = c.constant("EULER");
    println!("🦀 ConstantValue of EULER: ✅ {euler_constant:?}");
}
