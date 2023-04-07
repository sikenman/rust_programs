use std::io;

fn main() {
    println!("📅 Program to find Weekday or Weekend");
    println!("=====================================");
    
    println!("Enter a day of the week:");
    let mut input_day = String::new();

    io::stdin()
        .read_line(&mut input_day)
        .expect("Failed to read input day.");

    let day = input_day.trim().to_lowercase();

    let day_type = match day.as_str() {
        "monday" | "tuesday" | "wednesday" | "thursday" | "friday" => "Weekday",
        "saturday" | "sunday" => "Weekend",
        "lunes" | "martes" | "miércoles" | "jueves" | "viernes" => "Día laboral",
        "sábado" | "domingo" => "Fin de semana",
        _ => "Invalid day",
    };

    println!("{day} is a {day_type}");
}
