fn main() {

    let array1 = [999, 1, 720, 200, 69, 1024, 2048];
    let array2 = [69, 555, 1024, 1, 125, 1111, 720, 747];

    let result = find_common_elements(&array1, &array2);

    println!("Common elements: {:?}", result);
}


fn find_common_elements(array1: &[u16], array2: &[u16]) -> Vec<u16>{

    let mut common_elements = Vec::new(); // empty vector array

    // finding the common elements in the array
    for i in 0..array1.len() {
        for j in 0..array2.len() {
            if array1[i] == array2[j] {
                common_elements.push(array1[i]); // add the common element 
                break; // break the inner loop to avoid duplicates
            }
        }
    }
    common_elements // return
}
