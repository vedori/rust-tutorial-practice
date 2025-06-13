use std::collections::HashMap;
use std::vec::Vec;

const TITLE_EXCEPTIONS: [&str; 10] = ["a", "an", "in", "of", "the", "for", "to", "and", "n", "as"];

// Using a hash map and vectors,
// create a text interface to allow a user to add employee names to a department in a company;
// for example, “Add Sally to Engineering” or “Add Amir to Sales.”
// Then let the user retrieve a list of all people in a department
// or all people in the company by department, sorted alphabetically.
fn main() {
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        println!("Enter a number to complete the specified action below.");
        println!("1. Add an Employee to a Department");
        println!("2. List All People in a Department");
        println!("3. List All People in the Company");
        println!("Q. To exit the program");

        let mut action_choice = String::new();
        let action_choice = match std::io::stdin().read_line(&mut action_choice) {
            Ok(_) => action_choice.trim().to_lowercase(),
            Err(_) => {
                println!("Error reading from input! Please try again.");
                continue;
            }
        };
        if action_choice == "1" {
            add_an_employee_to_a_department(&mut departments);
        } else if action_choice == "2" {
            list_all_employees_in_a_department(&mut departments);
        } else if action_choice == "3" {
            list_all_employees_in_the_company(&mut departments);
        } else if action_choice == "q" {
            break;
        }
    }
}

fn list_all_employees_in_the_company(departments: &mut HashMap<String, Vec<String>>) {
    for (department, employee_list) in departments {
        println!("{department}: {employee_list:?}");
    }
}

fn add_an_employee_to_a_department(departments: &mut HashMap<String, Vec<String>>) {
    println!("Enter the name of the department");
    if departments.is_empty() {
        println!("There are currently no departments.");
        println!("Add a department.");
    } else {
        println!("The current departments are listed below. You may add a new department.");
        let list_of_departments: Vec<&String> = departments.keys().collect();
        println!("{:?}", list_of_departments);
    }

    // Get department choice
    if let Some(department_choice) = get_department_choice_from_user() {
        println!("Enter the employee's name");
        if let Some(name_choice) = get_name_from_user() {
            // Add a unique employee to the specified department
            match departments.get_mut(&department_choice) {
                // If no entry exists then it will initilize the employee list
                // with the name of the employee
                None => {
                    departments.insert(department_choice, vec![name_choice]);
                }
                // If there is already an employee list for the department
                // then the employee name is added if it is not already on the list

                // Binary search is implemented because the employees list is required
                // to be alphabetically sorted making it more performant than the `contains`
                // method
                Some(v) => match v.binary_search(&name_choice) {
                    Ok(_) => {
                        println!(
                            "{} is already assigned to {}!",
                            &name_choice, &department_choice
                        );
                    }
                    Err(_) => {
                        v.push(name_choice);
                        // Alphabetically sorts the employee list
                        v.sort();
                    }
                },
            }
        }
    }
}

fn list_all_employees_in_a_department(departments: &mut HashMap<String, Vec<String>>) {
    // Leaves the function if there are no departments
    if departments.is_empty() {
        println!("There are no recorded departments!");
        println!("To add a new department, assign an employee to it.");
        return;
    }

    let list_of_departments: Vec<&String> = departments.keys().collect();
    println!("The current departments are listed below.");
    println!("{:?}", list_of_departments);

    println!("Enter the department you want to see the employees of");

    // Get department choice
    while let Some(choice) = get_department_choice_from_user() {
        if let Some(employee_list) = departments.get(&choice) {
            println!("{employee_list:?}");
            break;
        } else {
            println!("That department is not recorded.");
            println!("To add a new department, assign an employee to it.");
            println!("Enter the department you want to see the employees of");
        }
    }
}

fn get_name_from_user() -> Option<String> {
    let mut name_choice = String::new();
    match std::io::stdin().read_line(&mut name_choice) {
        Ok(_) => {
            // Will not allow an empty name after trim() even if it sucessfully reads user input
            let choice = name_choice.trim();
            if choice.is_empty() {
                return None;
            }

            Some(name_choice.trim().to_string())
        }
        Err(_) => {
            println!("Error reading from input! Please try again.");
            None
        }
    }
}

// Gets department choice in title case regardless of the user input's caseing
fn get_department_choice_from_user() -> Option<String> {
    let mut choice = String::new();
    match std::io::stdin().read_line(&mut choice) {
        Ok(_) => {
            // Will not allow an empty name after trim() even if it sucessfully reads user input
            let choice = choice.trim();
            if choice.is_empty() {
                return None;
            }
            // Ensures the department is entered in title case regardless of the user input
            Some(get_title_case(choice))
        }
        Err(_) => {
            println!("Error reading from input!");
            None
        }
    }
}

// Turns any &str into title case
fn get_title_case(input: &str) -> String {
    let mut title_case = String::new();
    // splits the user input by whitespace to acount for departments names containing
    // multiple words such as "West Wing"
    for (word_pos, title) in input.split_ascii_whitespace().enumerate() {
        // After the first word is processed
        // certain words will be except from title casing
        if word_pos > 0 && TITLE_EXCEPTIONS.contains(&title) {
            title_case.push_str(title.to_lowercase().as_str());

            // Adds back the spacing that was removed when the user input was split
            title_case.push(' ');
            continue;
        }
        for (char_pos, c) in title.char_indices() {
            if char_pos == 0 {
                title_case.push(c.to_ascii_uppercase());
            } else {
                title_case.push(c.to_ascii_lowercase());
            }
        }
        // Adds back the spacing that was removed when the user input was split
        title_case.push(' ');
    }
    // Removes the extra space
    title_case.pop();

    title_case
}
