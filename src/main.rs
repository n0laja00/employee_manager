mod services {
    pub mod employee_service;
    pub mod file_service;
    pub mod id_service;
    pub mod workhour_service;
}
mod models {
    pub mod models;
}

use models::models::*;
use services::file_service::*;
use services::id_service::*;
use services::workhour_service::*;
use std::collections::HashMap;
use std::io::{self};

use crate::services::employee_service::edit_employee;

fn main() {
    let mut employee_list: Vec<Employee> = Vec::new();
    let mut workhour_list: Vec<WorkHour> = Vec::new();

    employee_list = {
        match read_employee_save_file() {
            Ok(contents) => contents,
            Err(_) => {
                eprintln!("");
                employee_list
            }
        }
    };

    workhour_list = {
        match read_workhour_save_file() {
            Ok(contents) => contents,
            Err(_) => {
                eprintln!("");
                workhour_list
            }
        }
    };

    println!("{:#?}", employee_list);
    println!("{:#?}", workhour_list);
    loop {
        let mut prompt: String = String::new();
        println!("Select mode which can either be: 'employee or workhour'");
        //io::stdin() to read line
        io::stdin()
            .read_line(&mut prompt)
            .expect("Failed to read line.");
        //***************************
        //SANITISE INPUT & PARSE
        //***************************
        let prompt: String = match prompt.to_lowercase().trim().parse() {
            Ok(string) => string,
            Err(_) => continue,
        };
        //***************************
        //BREAK DOWN INPUT
        //***************************
        let prompt_as_vector: Vec<&str> = prompt.split_whitespace().collect();

        match prompt_as_vector[0] {
            "employee" => {
                let mut prompt: String = String::new();
                println!("from interface: Add/remove/edit employees with: 'add/remove/edit Department John Doe id'");

                //io::stdin() to read line
                io::stdin()
                    .read_line(&mut prompt)
                    .expect("Failed to read line.");

                //***************************
                //SANITISE INPUT & PARSE
                //***************************
                let prompt: String = match prompt.to_lowercase().trim().parse() {
                    Ok(string) => string,
                    Err(_) => continue,
                };

                //***************************
                //BREAK DOWN INPUT
                //***************************
                let prompt_as_vector: Vec<&str> = prompt.split_whitespace().collect();
                let command = match prompt_as_vector[0] {
                    "add" => Command::Add,
                    "remove" => Command::Remove,
                    "edit" => Command::Edit,
                    _ => continue,
                };

                let input = Input::employee_mode(command, prompt_as_vector[1..].join(" "));

                //***************************
                //HANDLING BODY
                //***************************

                let employee: Employee = {
                    let body_as_vector: Vec<&str> = input.body.split(" ").collect();
                    let department: String = body_as_vector
                        .get(0)
                        .unwrap_or(&"no_department")
                        .to_string();
                    let first_name: String = body_as_vector
                        .get(1)
                        .unwrap_or(&"no_first_name")
                        .to_string();
                    let last_name: String =
                        body_as_vector.get(2).unwrap_or(&"no_last_name").to_string();

                    //constructing ID
                    let employee_id = {
                        let option_id = id_from_prompt(body_as_vector);

                        let id = if let Some(id) = option_id {
                            id
                        } else {
                            println!("Error in employee id");
                            continue;
                        };
                        id
                    };

                    Employee::new(department, first_name, last_name, employee_id)
                };

                //Create employee_mode_input
                let mut employee_mode_input: EmployeeModeInput =
                    Input::make_employee_input(input.command, employee);

                //***************************
                //MATCH ADD || REMOVE || EDIT
                //***************************
                match &employee_mode_input.command {
                    Command::Add => {
                        employee_mode_input.employee.id = check_id_availability(
                            employee_mode_input.employee.id,
                            employee_list.iter().map(|p| p.id).collect(),
                        );
                        employee_list.push(employee_mode_input.employee);
                    }
                    Command::Remove => {
                        if let Some(position) = employee_list
                            .iter()
                            .position(|x| *x == employee_mode_input.employee)
                        {
                            employee_list.remove(position);
                        } else {
                            println!("Employee not found!");
                            continue;
                        }
                    }
                    Command::Edit => edit_employee(&mut employee_list, employee_mode_input),
                    _ => continue,
                };

                employee_list.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                let mut map = HashMap::new();
                for employee in &employee_list {
                    let count = map.entry(&employee.department).or_insert(0);
                    *count += 1;
                }
                println!("{:#?}", &employee_list);
                println!("{:?}", map);

                write_employee_files(&employee_list)
            }
            "workhour" => {
                let prompt: String = workhour_query_main();

                let prompt_as_vector: Vec<&str> = prompt.split_whitespace().collect();

                let command: Command = match prompt_as_vector[0] {
                    "add" => Command::Add,
                    "remove" => Command::Remove,
                    "edit" => Command::Edit,
                    _ => continue,
                };

                let input = Input::workhour_mode(command, prompt_as_vector[1..].join(" "));

                let workhour: WorkHour = {
                    let mut body_as_vector: Vec<&str> = input.body.split(" ").collect();

                    let hours = if let Ok(result) = get_workhour_from_body(&mut body_as_vector) {
                        result
                    } else {
                        println!("Error occurred in adding WorkHour to an employee.");
                        continue;
                    };

                    let id: u64 = pick_next_available_id(&workhour_list);

                    let employee_id = {
                        let option_id = id_from_prompt(body_as_vector);

                        let id = if let Some(id) = option_id {
                            id
                        } else {
                            println!("Error in employee id");
                            continue;
                        };
                        id
                    };

                    WorkHour::new(hours, id, employee_id)
                };
                let workhour_mode_input: WorkHourModeInput =
                    Input::make_workhour_input(input.command, workhour);

                // TODO Matching for WorkHour
                match &workhour_mode_input.command {
                    Command::Add => {
                        let all_ids: Vec<u64> = employee_list.iter().map(|p| p.id).collect();

                        if all_ids.contains(&workhour_mode_input.workhour.employee_id) {
                            workhour_list.push(workhour_mode_input.workhour);
                        } else {
                            println!("No employee of this ID found!");
                            continue;
                        }
                    }
                    Command::Remove => {
                        let mut prompt: String = String::new();
                        println!("{:#?}", &workhour_list);
                        println!("What worktime would you like to remove? Give ID.");
                        //io::stdin() to read line
                        io::stdin()
                            .read_line(&mut prompt)
                            .expect("Failed to read line.");
                        let prompt_as_vector: Vec<&str> = prompt.split_whitespace().collect();

                        let id: u64 = {
                            let mut id: u64 = 0;
                            if let Ok(parsed_int) = prompt_as_vector[0].parse::<u64>() {
                                id = parsed_int;
                            } else {
                                println!("No workhour ID found.");
                                continue;
                            }
                            id
                        };

                        if let Some(position) = workhour_list.iter().position(|x| x.id == id) {
                            workhour_list.remove(position);
                        } else {
                            println!("WorkHour not found!")
                        }
                    }
                    Command::Edit => continue,
                    _ => continue,
                }
                write_workhour_files(&workhour_list);
            }
            _ => continue,
        };
        println!("{:#?}", workhour_list);
    }
}
