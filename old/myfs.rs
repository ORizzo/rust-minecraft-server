use std::{fs, path::Path};

pub fn creating_directory() {
    let path = Path::new("./data");

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
