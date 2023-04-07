fn main() {

    // character data type
    let dt_char: char = '⭐';
    
    // boolean data type
    let dt_true_bool: bool = true;
    let dt_false_bool: bool = false;
    
    // integer data types
    let dt_int: i32 = 1024;
    let dt_u8: u8 = 255;
    let dt_u16: u16 = 65535;
    let dt_i64: i64 = -9223372036854775808;

    // floating-point data types
    let dt_float: f32 = 3.1415926535;
    let dt_double: f64 = 3.1415926535897932384;

    /*
    ** String **
    There are two types of strings in Rust: String and &str.
    A String is stored as a vector of bytes (Vec<u8>). 
    -> https://doc.rust-lang.org/rust-by-example/std/str.html
    */
    let dt_string: String = String::from("🌎 Hello to Rust!");

    println!("Character is {}", dt_char);
    println!("Boolean value is {}", dt_true_bool);
    println!("Boolean value is {}", dt_false_bool);
    println!();
    println!("Integer is {}", dt_int);
    println!("Unsigned 8-bit integer is {}", dt_u8);
    println!("Unsigned 16-bit integer is {}", dt_u16);
    println!("Signed 64-bit integer is {}", dt_i64);
    println!();
    println!("Single-precision float is {}", dt_float);
    println!("Double-precision float is {}", dt_double);
    println!();
    println!("String is {}", dt_string);
}
