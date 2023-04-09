fn main() {
    // Initialize Array without data type
    let arr1 = ['a', 'b'];
    let arr2 = [11, 21, 31, 41, 51];
    let arr3 = [true, false, true];

    // Initialize Array with data type
    let arr4: [f32; 3] = [1.0, 2.0, 3.0];

    // Initialize Array with default values
    let arr5 = [0; 10];
    let arr6: [char; 5]  = ['🌹'; 5];
    
    // Print out the arrays
    println!("arr1 = {:?}", arr1);
    println!("arr2 = {:?}", arr2);
    println!("arr3 = {:?}", arr3);
    println!("arr4 = {:?}", arr4);
    println!("arr5 = {:?}", arr5);
    println!("arr6 = {:?}", arr6);
}

/* Output File

arr1 = ['a', 'b']
arr2 = [11, 21, 31, 41, 51]
arr3 = [true, false, true]
arr4 = [1.0, 2.0, 3.0]
arr5 = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
arr6 = ['🌹', '🌹', '🌹', '🌹', '🌹']

*/
