pub mod download;

use crate::binaries;
use os_type;
use std::{fs, fs::File, io::Write, path::Path};

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

#[derive(Debug)]
pub enum OsType {
    Windows,
    Mac,
    Linux,
}

pub fn get_os() -> OsType {
    let os = os_type::current_platform();

    let os: OsType = match os.os_type {
        os_type::OSType::Unknown => OsType::Windows,
        os_type::OSType::OSX => OsType::Mac,
        _ => OsType::Linux,
    };

    return os;
}

pub fn download_docker() {
    let os = get_os();

    match os {
        OsType::Windows => download::download_docker_windows(),
        OsType::Linux => download::download_docker_linux(),
        _ => {
            println!("Your OS do not have support yet")
        }
    }
}

pub fn undo() {
    let path = Path::new(DIR_NAME);

    let _ = fs::remove_dir_all(path);
    println!("Minecraft server folder successfully removed");

    return;
}
