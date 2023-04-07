use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    print!("Enter an RGB value in the format (X,X,X): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    // Parse the input string into separate RGB components
    let components: Vec<&str> = input.trim().trim_matches(|c| c == '(' || c == ')')
                                       .split(',')
                                       .collect();
    if components.len() != 3 {
        panic!("Invalid input: expected 3 comma-separated values");
    }
    println!("{:?}", components);

    // Parse the RGB components as integers and validate their range
    let mut rgb = [0u8; 3];
    for (i, component) in components.iter().enumerate() {
        let value = component.trim().parse::<u8>().unwrap();
        if value < 1 || value > 255 {
            panic!("Invalid input: RGB component {} is out of range", i+1);
        }
        rgb[i] = value;
    }

    // Convert the RGB value to a hexadecimal string
    //{:02X} makes sure one digit get 0 padding; eg. 2 -> 02
    let hex = format!("#{:02X}{:02X}{:02X}", rgb[0], rgb[1], rgb[2]);
    
    println!("The rgb{} => {}", input.trim(), hex);
}
