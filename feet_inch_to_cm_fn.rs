
fn main() {
    let feet_inch = vec![5.0, 6.0];

    if feet_inch.len() > 1 {
        let feet = feet_inch[0];
        let inches = if feet_inch.len() > 1 {
            feet_inch[1]
        } else {
            0.0
        };

        let cm = feet_and_inches_to_cm(feet, inches);

        println!("{} feet {} inches is {:.2} cm", feet, inches, cm);
        println!("{}' {}\" = {:.2} cm", feet, inches, cm);
    }
}

fn feet_and_inches_to_cm(feet: f64, inches: f64) -> f64 {
    let total_inches = feet * 12.0 + inches;
    let cm = total_inches * 2.54;
    cm
}

/* Output of Program 

5 feet 6 inches is 167.64 cm
5' 6" = 167.64 cm

*/
