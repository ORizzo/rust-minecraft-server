use std::{
    fs::{self},
    io::{self},
};

pub struct User {
    username: String,
    password: String,
}

pub fn saving_user() -> String {
    println!("Enter you username:");

    let mut username = String::new();

    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read line");

    println!("Enter you password:");

    let mut password = String::new();

    io::stdin()
        .read_line(&mut password)
        .expect("Failed to read line");

    let user = User {
        username: username,
        password: password,
    };

    let trimmed_username = &user.username.clone().trim().to_lowercase();

    let path = ["./data/", trimmed_username, ".txt"].join("");

    _ = fs::write(path, user.username + &user.password.trim_end());

    return trimmed_username.to_owned();
}

pub fn reading_user(username: String) -> String {
    let path = ["./data/", &username, ".txt"].join("");

    let content = fs::read_to_string(path).expect("Cannot open file");

    println!("{}", content);

    return content;
}
