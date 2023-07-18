use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

struct NoMode;
struct EmployeeMode;
struct WorkHoursMode;
#[derive(Debug, PartialEq, Eq, PartialOrd)]
enum Command {
    Add,
    Remove,
    Edit,
}
#[derive(Debug, PartialEq, Eq, PartialOrd)]
struct Employee {
    department: String,
    first_name: String,
    last_name: String,
    id: u64,
}
#[derive(Debug, PartialEq, Eq, PartialOrd)]
struct EmployeeModeInput {
    command: Command,
    employee: Employee,
}
#[derive(Debug)]
struct WorkHoursModeInput {
    command: Command,
    workhours: WorkHours,
}
#[derive(Debug)]
struct Input<State = NoMode> {
    command: Command,
    body: String,
    state: std::marker::PhantomData<State>,
}

#[derive(Debug)]
struct WorkHours {
    hours: f64,
    employee_id: u64,
}

impl WorkHours {
    fn new(hours: f64, employee_id: u64) -> Self {
        WorkHours { hours, employee_id }
    }
    fn add_hours(&self, hours: f64) -> f64 {
        self.hours + hours
    }
    fn remove_hours(&self, hours: f64) -> f64 {
        self.hours - hours
    }
    fn get_hours(&self) -> f64 {
        self.hours
    }
}

impl Input<NoMode> {
    fn employee_mode(command: Command, body: String) -> Input<EmployeeMode> {
        Input {
            command: command,
            body: body,
            state: std::marker::PhantomData::<EmployeeMode>,
        }
    }
    fn workhours_mode(command: Command, body: String) -> Input<WorkHoursMode> {
        Input {
            command: command,
            body: body,
            state: std::marker::PhantomData::<WorkHoursMode>,
        }
    }
}

impl Input<WorkHours> {
    fn make_workhours_input(command: Command, workhours: WorkHours) -> WorkHoursModeInput {
        WorkHoursModeInput { command, workhours }
    }
}
impl Input<EmployeeMode> {
    fn make_employee_input(command: Command, employee: Employee) -> EmployeeModeInput {
        EmployeeModeInput { command, employee }
    }
}

impl Employee {
    fn new(department: String, first_name: String, last_name: String, id: u64) -> Self {
        Employee {
            department,
            first_name,
            last_name,
            id,
        }
    }
}

fn main() {
    let save_file = "saved_data.txt";
    let mut employee_list: Vec<Employee> = Vec::new();
    let mut workhours_list: Vec<WorkHours> = Vec::new();

    employee_list = {
        match read_file(save_file) {
            Ok(contents) => contents,
            Err(_) => {
                eprintln!("");
                employee_list
            }
        }
    };

    println!("{:#?}", employee_list);

    loop {
        let mut prompt: String = String::new();
        println!("Select mode which can either be: 'employee or workhours'");
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
                println!(
                    "Add/remove/edit employees with: 'add/remove/edit Department John Doe id'"
                );

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
                    let id: u64 = {
                        let mut id: u64 = 0;
                        // Go over vector prompt and find an integer The last integer is the id
                        for string in &body_as_vector {
                            if let Ok(parsed_int) = string.parse::<u64>() {
                                id = parsed_int;
                            } else {
                                id = 0;
                            }
                        }
                        id
                    };
                    Employee::new(department, first_name, last_name, id)
                };

                //Create employee_mode_input
                let mut employee_mode_input: EmployeeModeInput =
                    Input::make_employee_input(input.command, employee);

                //***************************
                //MATCH ADD || REMOVE || EDIT
                //***************************
                match &employee_mode_input.command {
                    Command::Add => {
                        employee_mode_input.employee.id =
                            check_id_availability(employee_mode_input.employee.id, &employee_list);
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
                    Command::Edit => {
                        if let Some(position) = employee_list
                            .iter()
                            .position(|x| *x == employee_mode_input.employee)
                        {
                            let mut prompt: String = String::new();
                            println!("{:#?}", &employee_list);
                            println!("Insert edit: 'Department John Doe'");
                            //io::stdin() to read line
                            io::stdin()
                                .read_line(&mut prompt)
                                .expect("Failed to read line.");
                            let prompt_as_vector: Vec<&str> = prompt.split_whitespace().collect();
                            let department: String = prompt_as_vector
                                .get(0)
                                .unwrap_or(&"no_department")
                                .to_string();
                            let first_name: String = prompt_as_vector
                                .get(1)
                                .unwrap_or(&"no_first_name")
                                .to_string();
                            let last_name: String = prompt_as_vector
                                .get(2)
                                .unwrap_or(&"no_last_name")
                                .to_string();

                            let edited_employee = Employee::new(
                                department,
                                first_name,
                                last_name,
                                employee_mode_input.employee.id,
                            );
                            employee_list.remove(position);
                            employee_list.push(edited_employee);
                        } else {
                            println!("Failed to edit a nonexistent entity!");
                        }
                    }
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

                let mut data_file = File::create("data.txt").expect("Creation failed!");
                let mut save_file = File::create(save_file).expect("Creation failed!");
                for employee in &employee_list {
                    write!(
                        data_file,
                        "{} {} works in {} and their id is {} \r",
                        &employee.first_name,
                        &employee.last_name,
                        &employee.department,
                        &employee.id
                    )
                    .expect("Failed to write!");
                    write!(
                        save_file,
                        "{},{},{},{} \r",
                        &employee.first_name,
                        &employee.last_name,
                        &employee.department,
                        &employee.id
                    )
                    .expect("Failed to write!");
                }
            }
            "workhours" => {
                let mut prompt: String = String::new();
                println!("Add/remove/edit workhours with: 'add/remove/edit employee_id hours'");
                io::stdin()
                    .read_line(&mut prompt)
                    .expect("Failed to read line");
                let prompt: String = match prompt.to_lowercase().trim().parse() {
                    Ok(string) => string,
                    Err(_) => continue,
                };
                let prompt_as_vector: Vec<&str> = prompt.split_whitespace().collect();
                let command: Command = match prompt_as_vector[0] {
                    "add" => Command::Add,
                    "remove" => Command::Remove,
                    "edit" => Command::Edit,
                    _ => continue,
                };
                let input = Input::workhours_mode(command, prompt_as_vector[1..].join(" "));

                let workhours: WorkHours = {
                    let body_as_vector: Vec<&str> = input.body.split(" ").collect();
                    let hours: f64 = {
                        let mut hours: f64 = 0.0;
                        // Go over vector prompt and find an integer The last integer is the hours
                        if let Ok(parsed_float) =
                            body_as_vector.get(0).unwrap_or(&"0.0").parse::<f64>()
                        {
                            hours = parsed_float
                        } else {
                            hours = 0.0
                        }
                        hours
                    };

                    let id: u64 = {
                        let mut id: u64 = 0;
                        // Go over vector prompt and find an integer The last integer is the id
                        for string in &body_as_vector {
                            if let Ok(parsed_int) = string.parse::<u64>() {
                                id = parsed_int;
                            } else {
                                id = 0;
                            }
                        }
                        id
                    };
                    WorkHours::new(hours, id)
                };
                let mut workhours_mode_input: WorkHoursModeInput =
                    Input::make_workhours_input(input.command, workhours);

                // TODO Matching for Workhours
            }
            _ => continue,
        };
    }
}

//Read file
fn read_file(filename: &str) -> io::Result<Vec<Employee>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut employee_list: Vec<Employee> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let fields: Vec<&str> = line.split(',').map(|field| field.trim()).collect();
        if fields.len() >= 2 {
            let employee = Employee::new(
                fields[0].to_string(),
                fields[1].to_string(),
                fields[2].to_string(),
                fields[3].parse::<u64>().unwrap_or(0),
            );
            employee_list.push(employee);
        }
    }
    Ok(employee_list)
}

//***************************
//CONSTRUCTING THE ID
//***************************
fn check_id_availability(mut id: u64, employee_list: &Vec<Employee>) -> u64 {
    let mut all_ids: Vec<u64> = employee_list.iter().map(|p| p.id).collect();
    //if id already exists in all ids, take the last id from all_ids and add 1.
    if all_ids.contains(&id) {
        if let Some(last_id) = all_ids.pop() {
            id = last_id + 1;
        }
    }
    id
}
