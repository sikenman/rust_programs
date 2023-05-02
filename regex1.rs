use regex::Regex;

fn main() {
    // Create a new regex pattern that matches any character between space and tilde,
    // and that does not matches any character between space and tilde.
    
    let re1 = Regex::new(r"[ -~]+").unwrap();
    let re2 = Regex::new(r"[^ -~]+").unwrap();

    // Search String
    let text1 = "Hello, bro! ß Where do you live? » ~ I live in Dubai. ¿Thank You.";
    let text2 ="Hello World, こんにちは世界.";
    
    // Find all matches
    let result1 = re1.find_iter(text1)
        .map(|m| m.as_str())
        .collect::<Vec<_>>();
        
    let result2 = re2.find_iter(text2)
        .map(|m| m.as_str())
        .collect::<Vec<_>>();

    // Print out the results
    println!("{:?}", result1);
    println!("{:?}", result2);
}

/* Output of the file

["Hello, bro! ", " Where do you live? ", " ~ I live in Dubai. ", "Thank You."]
["こんにちは世界"]

*/
