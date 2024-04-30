pub mod compose;
use std::{fs, fs::File, io::Write, path::Path};

use crate::binaries;

const DIR_NAME: &str = "minecraft-server";

pub fn copy_docker_compose() {
    let path = [DIR_NAME, "/docker-compose.yml"].join("");

    println!("{}", path);

    let mut file =
        File::create(path).expect("Couldn't create docker-compose.yml file, verify that it exists");

    file.write(&binaries::DOCKER_COMPOSE_BIN)
        .expect("Could't write docker-compose.yml");

    return;
}

pub fn creating_directory() {
    let path = Path::new(DIR_NAME);

    if path.exists() {
        println!("Directory already exists! Skipping creating...");
        return;
    }

    let dir = fs::create_dir(path);

    if dir.is_ok() {
        println!("Created new data directory successfully!!!")
    } else {
        println!(
            "Some problem occured creating data directory. error:{:?}",
            dir.err()
        )
    }
}

pub fn execute_docker_compose() {
    compose::run()
}

pub fn undo() {
    let path = Path::new(DIR_NAME);

    let _ = fs::remove_dir_all(path);
    println!("Minecraft server folder successfully removed");

    return;
}
