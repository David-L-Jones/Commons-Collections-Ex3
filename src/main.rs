use std::collections::HashMap;
use std::io;

#[derive(Debug)]
enum MenuOption {
    AddPerson,
    PeopleInDepartment,
    PeopleInCompany,
    Quit,
    Unknown,
}

fn main() {
    println!("\n*** Employee Management ***");

    // Call the convenience functions to setup employees and departments.
    let depts = setup_departments();
    let emps = setup_employees();
    let mut emp_dept_map = setup_emp_dept_map();

    loop {
        let choice = menu();
        match &choice {
            MenuOption::AddPerson => add_person_to_dept(&depts, &emps, &mut emp_dept_map),
            MenuOption::PeopleInDepartment => get_people_in_dept(&depts, &mut emp_dept_map),
            MenuOption::PeopleInCompany => get_all_people(&mut emp_dept_map),
            MenuOption::Unknown => continue,
            MenuOption::Quit => {
                println!("Quit");
                break;
            }
        }
    }
}

/// Show the menu options to the user and input their selection
fn menu() -> MenuOption {
    println!("\n1. Add a person to a department");
    println!("2. List people in a department");
    println!("3. Show all people in the company");
    println!("4. Exit the application");
    println!("Enter 1 - 4");

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read menu option");

    let choice: u32 = choice.trim().parse().unwrap_or(0);

    match choice {
        1 => MenuOption::AddPerson,
        2 => MenuOption::PeopleInDepartment,
        3 => MenuOption::PeopleInCompany,
        4 => MenuOption::Quit,
        _ => MenuOption::Unknown,
    }
}

/// Add an employee to a department
fn add_person_to_dept(depts: &[String], emps: &[String], map: &mut HashMap<String, String>) {
    println!("Enter a command:");
    let mut command = String::new();

    io::stdin()
        .read_line(&mut command)
        .expect("Failed to read command");

    let words: Vec<&str> = command.split_whitespace().collect();

    if words.len() != 4 {
        println!("Invalid command, enter: action name to dept");
    } else {
        let name = match words.get(1) {
            Some(name) => name.to_string(),
            None => String::new(),
        };

        let dept = match words.get(3) {
            Some(dept) => dept.to_string(),
            None => String::new(),
        };

        if emps.contains(&name) && depts.contains(&dept) {
            println!("Found name: {} and dept: {}", name, dept);
            map.insert(name, dept);
        } else {
            println!("Didn't find name: {} or dept: {}", name, dept);
        }
    }
}

/// Get a collection of people in a department
fn get_people_in_dept(depts: &[String], map: &mut HashMap<String, String>) {
    println!("Enter a department: ");

    // Get the department from the user
    let mut dept = String::new();
    io::stdin()
        .read_line(&mut dept)
        .expect("Failed to read command");
    let dept = dept.trim().to_string();

    println!("Searching for dept: {}", dept);

    if depts.contains(&dept) {
        // Filter for the user's department and collect the people in that dept.
        let mut people: Vec<_> = map.iter().filter(|&(_, d)| d == &dept).collect();
        println!("\nFound {} people in {}", people.len(), dept);

        // Sort people alphabetically by name
        people.sort();

        for person in people {
            println!("{}", person.0);
        }
    } else {
        println!("Didn't find dept: {}", dept);
    }
}

/// List all the people in the company alphabetically.
fn get_all_people(map: &mut HashMap<String, String>) {
    let mut people: Vec<_> = map.keys().collect();
    people.sort();

    for person in people {
        println!("{}", person);
    }
}

/// Convenience function to create a vector of employees.
fn setup_employees() -> Vec<String> {
    let emps = vec![
        String::from("Sally"),
        String::from("Bob"),
        String::from("Amir"),
        String::from("Sales"),
        String::from("Aaron"),
    ];
    emps
}

/// Convenience function to create a vector of department names.
fn setup_departments() -> Vec<String> {
    let depts = vec![
        String::from("Sales"),
        String::from("Engineering"),
        String::from("HR"),
        String::from("Cybersecurity"),
    ];
    depts
}

/// Convenience function to map employees to departments
fn setup_emp_dept_map() -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();

    map.insert(String::from("Sally"), String::from("Engineering"));
    map.insert(String::from("Bob"), String::from("Engineering"));
    map.insert(String::from("Zara"), String::from("Sales"));
    map.insert(String::from("Aaron"), String::from("Sales"));
    map.insert(String::from("Sally"), String::from("Sales"));
    map.insert(String::from("Amir"), String::from("Sales"));

    map
}
