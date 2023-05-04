/*
This program uses the `sin()`, `cos()`, `tan()`, and `ln()` functions from the Rust standard library 
to calculate the sine, cosine, tangent, and natural logarithm of some numbers. 
*/

use std::f64::consts::PI;

fn main() {
    let x: f64 = PI/2.0;
    let y: f64 = PI/3.0;
    let z: f64 = PI/6.0;

    println!("pi = {}", PI);
    println!("sin({}) = {}", x, x.sin());
    println!("cos({}) = {}", y, y.cos());
    println!("tan({}) = {}", z, z.tan());
    println!("log({}) = {}", 100.0 * x * y * z, (100.0 * x * y * z).ln());
}

/* Output of the program

pi = 3.141592653589793
sin(1.5707963267948966) = 1
cos(1.0471975511965976) = 0.5000000000000001
tan(0.5235987755982988) = 0.5773502691896257
 log(86.12854633416615) = 4.455840905080182

*/
