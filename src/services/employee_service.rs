use crate::models::models::Employee;
use crate::models::*;
use std::io::{self};

pub fn employee_mode_function() {
    loop {}
}
pub fn edit_employee(
    employee_list: &mut Vec<models::Employee>,
    employee_mode_input: models::EmployeeModeInput,
) {
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
