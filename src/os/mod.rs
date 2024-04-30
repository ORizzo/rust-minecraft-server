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

/*

let os = get_os();

match os {
    OsType::Windows => compose::download_docker_windows(),
    OsType::Linux => compose::download_docker_linux(),
    _ => {
        println!("Your OS do not have support yet")
    }
} */
