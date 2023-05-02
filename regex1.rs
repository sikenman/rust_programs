use regex::Regex;

fn main() {
    // Create a new regex pattern that matches any character between space and tilde
    let re = Regex::new(r"[ -~]+").unwrap();

    // Search String
    let text = "Hello, bro! ß Where do you live? » ~ I live in Dubai. ¿Thank You.";

    // Find all matches
    let result = re.find_iter(text)
        .map(|m| m.as_str())
        .collect::<Vec<_>>();

    // Print out the results
    println!("{:?}", result);
}

/* Output of the file

["Hello, bro! ", " Where do you live? ", " ~ I live in Dubai. ", "Thank You."]

*/
