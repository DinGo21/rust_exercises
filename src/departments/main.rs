use std::io;
use std::collections::HashMap;

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let mut buf = String::new();
        println!("Type \"Add (employee) to (deparment)\"");
        io::stdin()
            .read_line(&mut buf)
            .expect("Failed to read line");
        if buf.trim() == "quit" {
            break;
        }
        let Some((employee, department)) = read_buf(&buf) else {
            continue;
        };
        let employees = company.entry(department).or_default();
        employees.push(employee);
        for (d, emps) in &company {
            println!("Department: {d}");
            println!("Employees:");
            for e in emps {
                println!("{e}")
            }
        }
    }
}

fn read_buf(buf: &str) -> Option<(String, String)> {
    let mut args: Vec<&str> = Vec::new();
    for word in buf.split_whitespace() {
        args.push(word);
    }
    if args.len() != 4 {
        println!("Wrong arguments");
        return None;
    }
    let employee: String = match args[0] == "Add" {
        true => String::from(args[1]),
        false => {
            println!("Wrong command");
            return None;
        }
    };
    let department: String = match args[2] == "to" {
        true => String::from(args[3]),
        false => {
            println!("Wrong command");
            return None;
        }
    };
    Some((employee, department))
}

