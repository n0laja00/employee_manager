use std::io::{self};

pub fn workhour_query_main() -> String {
    let mut prompt: String = String::new();
    println!("Add/remove/edit WorkHour with: 'add/remove/edit hours employee_id'");

    io::stdin()
        .read_line(&mut prompt)
        .expect("Failed to read line");

    prompt = match prompt.to_lowercase().trim().parse() {
        Ok(string) => string,
        Err(_) => panic!(),
    };

    prompt
}

pub fn get_workhour_from_body(body_as_vector: &mut Vec<&str>) -> Result<f64, &'static str> {
    if body_as_vector.is_empty() {
        return Err("Error: Prompt is empty");
    }

    let parsed_float = body_as_vector.remove(0).parse::<f64>();

    match parsed_float {
        Ok(parsed_float) => {
            if parsed_float > 0.0 {
                Ok(parsed_float)
            } else {
                Err("Error: Hours need to be greater than 0!")
            }
        }
        Err(_) => Err("Error: unable to parse float from prompt"),
    }
}
