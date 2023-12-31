use std::process::Command;

pub fn download_docker_windows() {
    let output = Command::new("powershell")
        .arg("docker -v")
        .output()
        .expect("Couldn't spawn a powershell instance");

    let string = String::from_utf8(output.stdout.clone())
        .expect("Couldn't convert vec output to string output");

    println!("{}", string.trim());
}

pub fn download_docker_linux() {}
