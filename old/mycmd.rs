use std::{fs::File, io::Write, process::Command};

pub fn shell() {
    let result = Command::new("powershell")
        .args(["ls"])
        .output()
        .expect("failed to execute process");
    //let stdout = String::from_utf8(result.stdout).expect("Our bytes should be valid utf8");

    let mut f = File::create("./result.txt").expect("Failed to create output file");
    _ = f.write(&result.stdout);

    println!("{:?}", result);
}
