/*
This program uses the `sin()`, `cos()`, `tan()`, and `ln()` functions from the Rust standard library 
to calculate the sine, cosine, tangent, and natural logarithm of some numbers. 
*/

use std::f64::consts::PI;

fn main() {
    let x: f64 = 2.0;
    let y: f64 = 3.0;
    let z: f64 = 4.0;

    println!("sin({}) = {}", x, x.sin());
    println!("cos({}) = {}", y, y.cos());
    println!("tan({}) = {}", z, z.tan());
    println!("log({}) = {}", x * y * z, (x * y * z).ln());
    println!("pi = {}", PI);
}

/* Output of the program

sin(1.5707963267948966) = 1
cos(1.5707963267948966) = 0.00000000000000006123233995736766
tan(1.0471975511965976) = 1.7320508075688767
log(2.5838563900249847) = 0.9492830077602001
pi = 3.141592653589793

*/
