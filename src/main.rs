use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
const OUTPUT_PATH: &str = "/home/jedrzej/Desktop/rust_project/user_list.json";

#[derive(Debug, Deserialize, Serialize)]
struct UserInput {
    name: String,
    surname: String,
    action: String,
    phone: String,
}

// enum Action {
//     Create,
//     Retrieve,
// }

fn main() {
    let user_id = get_user_id();
    let database_exists = std::path::Path::new(OUTPUT_PATH).exists();
    if !database_exists {
        create_database();
    }
    let mut user_database = get_user_database();
    let user_exists: bool = check_user_existence(&mut user_database, &user_id);
    if !user_exists && user_id.action == "create" {
        add_user_id(&mut user_database, user_id);
    } else {
        println!("The name exists in the database");
    }
}

fn get_user_id() -> UserInput {
    let user_name: String = ask_user("name");
    let user_surname: String = ask_user("surname");
    let user_phone: String = ask_user("phone number");
    let chosen_action: String = ask_user("action");
    let user_id: UserInput = UserInput {
        name: user_name,
        surname: user_surname,
        phone: user_phone,
        action: chosen_action,
    };
    user_id
}

fn ask_user(question: &str) -> String {
    if question == "action" {
        println!("Pick one action - create or retrieve:");
    } else {
        println!("Enter the {}:", question);
    }
    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .expect("failed to readline");
    answer.pop();
    answer
}

fn create_database() {
    let empty_database: Vec<UserInput> = Vec::new();
    let new_database: String = serde_json::to_string(&empty_database).unwrap();
    std::fs::write(OUTPUT_PATH, new_database).unwrap();
}

fn get_user_database() -> Vec<UserInput> {
    let database_data = fs::read_to_string(OUTPUT_PATH).expect("Unable to read file");
    let database: Vec<UserInput> = serde_json::from_str(&database_data).unwrap();
    database
}

fn check_user_existence(database: &mut Vec<UserInput>, user_id: &UserInput) -> bool {
    if database.iter().any(|i| i.name == user_id.name) {
        return true;
    }
    return false;
}

fn add_user_id(database: &mut Vec<UserInput>, user_id: UserInput) {
    println!("Creating user id...");
    database.push(user_id);
    let updated_database: String = serde_json::to_string(database).unwrap();
    std::fs::write(OUTPUT_PATH, updated_database).unwrap();
    println!("Done");
}
