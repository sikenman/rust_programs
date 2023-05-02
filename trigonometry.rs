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
