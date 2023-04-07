
use std::io;
use std::f32::consts::PI;

fn main() {
    println!(" Program to calculate Area of Circle ⭕");
    println!("=======================================");
    
    let mut input_radius = String::new();
    println!("\nEnter the radius: ");
    
    io::stdin()
        .read_line(&mut input_radius)
        .expect("Failed to read input radius.");
        
    let radius: f32 = input_radius.trim().parse().expect("Invalid input radius.");
    
    let area = PI * radius * radius;
    
    println!("Area of the circle with radius {radius} is: {area}");
}
