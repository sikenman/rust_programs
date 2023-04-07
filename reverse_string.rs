fn main() {
    let original_string = "🌏 Hello World !!";
    let mut reversed_string = String::new();
    
    for c in original_string.chars().rev() {
        reversed_string.push(c);
    }
    println!("Original: {}", original_string);
    println!("Reversed: {}", reversed_string);
}
