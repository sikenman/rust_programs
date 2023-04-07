fn main() {

    let numbers = [5, 12, 99, 1, 7, 13, 28, 6, -4, 0];

    let mut min = numbers[0];
    let mut max = numbers[0];

    for &num in numbers.iter() {
        if num < min {
            min = num;
        }
        if num > max {
            max = num;
        }
    }

    println!("Minimum number: {}", min);
    println!("Maximum number: {}", max);
}
