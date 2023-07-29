use crate::models::*;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

//Read file
pub fn read_employee_save_file() -> io::Result<Vec<models::Employee>> {
    let save_file = "employee_save_file";
    let file = File::open(save_file)?;
    let reader = BufReader::new(file);
    let mut employee_list: Vec<models::Employee> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let fields: Vec<&str> = line.split(',').map(|field| field.trim()).collect();
        if fields.len() >= 2 {
            let employee = models::Employee::new(
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

//Read file
pub fn read_workhour_save_file() -> io::Result<Vec<models::WorkHour>> {
    let save_file = "workhours_save_file";
    let file = File::open(save_file)?;
    let reader = BufReader::new(file);
    let mut workhour_list: Vec<models::WorkHour> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let fields: Vec<&str> = line.split(',').map(|field| field.trim()).collect();
        if fields.len() >= 2 {
            let workhour = models::WorkHour::new(
                fields[0].parse::<f64>().unwrap_or(0.0),
                fields[1].parse::<u64>().unwrap_or(0),
                fields[2].parse::<u64>().unwrap_or(0),
            );
            workhour_list.push(workhour);
        }
    }
    Ok(workhour_list)
}

//Save file
pub fn write_employee_files(employee_list: &Vec<models::Employee>) {
    let save_file = "workhours_save_file";
    let data_file = "Workhours_data_file";
    let mut data_file = File::create(data_file).expect("Creation failed!");
    let mut save_file = File::create(save_file).expect("Creation failed!");
    for employee in employee_list {
        write!(
            data_file,
            "{} {} works in {} and their id is {} \r",
            &employee.first_name, &employee.last_name, &employee.department, &employee.id
        )
        .expect("Failed to write!");
        write!(
            save_file,
            "{},{},{},{} \r",
            &employee.department, &employee.first_name, &employee.last_name, &employee.id
        )
        .expect("Failed to write!");
    }
}

//Save file
pub fn write_workhour_files(workhour_list: &Vec<models::WorkHour>) {
    let save_file = "workhours_save_file";
    let data_file = "Workhours_data_file";
    let mut data_file = File::create(data_file).expect("Creation failed!");
    let mut save_file = File::create(save_file).expect("Creation failed!");
    for workhour in workhour_list {
        write!(
            data_file,
            "length is {} it's id is {} and it's made by id: {} \r",
            &workhour.hours, &workhour.id, &workhour.employee_id
        )
        .expect("Failed to write!");
        write!(
            save_file,
            "{},{},{},\r",
            &workhour.hours, &workhour.id, &workhour.employee_id
        )
        .expect("Failed to write!");
    }
}
