fn main() {
    
    let mut ascii_code = 32;    // 32 is for space
    let mut line_break = 0;
    
    loop {
        let ascii_char = ascii_code as u8 as char;

        // bypassing characters that are not printable
        if ascii_code > 126 && ascii_code <= 160 {
            ascii_code += 1;
            continue;
        }
        
        print!("{} ", ascii_char);
        ascii_code += 1;
        
        // break line after printing 15 characters
        if line_break==15 {
            println!();
            line_break = 0;
        } else {
            line_break += 1;
        }
        
        // go out of loop once we reach end of ASCII
        if ascii_code == 256 {
            break;
        }
    }
}
