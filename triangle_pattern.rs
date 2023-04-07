fn main() {
    const NO_LINES: u8 = 5;
    // Rust supports unicode characters including emoji
    
    // increasing triangle pattern
    for x in 1..=NO_LINES {
        for _ in 1..=x {
            print!("⭐");
        }
        println!();
    }
    
    println!();
    // decreasing triangle pattern
    for x in 1..=NO_LINES {
        for _ in 0..=NO_LINES-x {
            print!("⭐");
        }
        println!();
    }
    
    // combined the above two to form half diamond
    for x in 1..=NO_LINES {
        for _ in 1..=x {
            print!("⭐");
        }
        println!();
    }
    for x in 1..=NO_LINES {
        for _ in 1..=NO_LINES-x {   // this line is different [0 -> 1]
            print!("⭐");
        }
        println!();
    }
}
