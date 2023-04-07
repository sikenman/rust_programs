use std::io;

fn main() {
    let mut to_do_list: Vec<String> = Vec::new();

    loop {
        println!("Enter a task (or enter END to quit):");
        let mut task = String::new();
        
        io::stdin()
            .read_line(&mut task)
            .expect("Failed to read line");
            
        let trimmed_task = task.trim();
        if trimmed_task == "END" {
            println!("Goodbye!");
            break;
        }
        
        to_do_list.push(String::from(trimmed_task));
        to_do_list.sort();
        
        print_to_do_list(&to_do_list);
    }
    print_to_do_list(&to_do_list);
}

fn print_to_do_list(to_do_list: &Vec<String>) {
    println!("\n📜 To-do List:");
    println!("===============");
    for task in to_do_list {
        println!(" - {}", task);
    }
}
