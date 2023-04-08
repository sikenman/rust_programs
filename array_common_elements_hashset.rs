use std::collections::HashSet;

fn main() {
    let first_array  = [999, 1, 1159, 200, 36, 69, 1024, 2048];
    let second_array = [69, 36, 555, 1024, 1, 125, 1159, 720, 747];

    // passing arrays by ref
    let result = find_common_elements_optimized(&first_array, &second_array);

    println!("Common elements: {:?}", result);
}

fn find_common_elements_optimized(array1: &[u16], array2: &[u16]) -> Vec<u16> {

    let mut common_elements = Vec::new(); // empty vector array
    let set1: HashSet<_> = array1.iter().collect(); // convert array1 to a hash set
    //println!("{:?}",set1);
    
    // finding the common elements in the array
    for &elem in array2 {
        if set1.contains(&elem) {
            common_elements.push(elem); // add the common element to the vector
        }
    }
    
    common_elements // return the vector
}
