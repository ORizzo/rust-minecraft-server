use std::process::Command;

pub fn run() {
    Command::new("./bin/sh")
        .args([
            "ls",
            "&",
            "cd",
            "./minecraft-server",
            "&",
            "docker",
            "compose",
            "start",
        ])
        .spawn()
        .expect("failed to execute process");

    println!("Everything alright!")
}
