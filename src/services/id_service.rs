use crate::models::models::WorkHour;

//***************************
//CONSTRUCTING THE ID
//***************************
pub fn check_id_availability(id: u64, mut list: Vec<u64>) -> u64 {
    let mut id = id;
    //if id already exists in all ids, take the last id from all_ids and add 1.
    if list.contains(&id) {
        if let Some(last_id) = list.pop() {
            id = last_id + 1;
        }
    }
    id
}

pub fn id_from_prompt(body_as_vector: Vec<&str>) -> Option<u64> {
    if body_as_vector.is_empty() {
        return None;
    }

    let parsed_int = match body_as_vector.last() {
        Some(last) => match last.parse::<u64>() {
            Ok(parsed_int) => Some(parsed_int),
            Err(e) => {
                println!("Failed to parse string to int: {:?}", e);
                None
            }
        },
        None => {
            println!("Prompt is empty");
            None
        }
    };
    parsed_int
}

pub fn pick_next_available_id(workhour_list: &Vec<WorkHour>) -> u64 {
    let mut id: u64 = 0;
    id = check_id_availability(id, workhour_list.iter().map(|p| p.id).collect());
    id
}
