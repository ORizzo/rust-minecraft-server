use std::{
    thread::{self, sleep},
    time::Duration,
};

pub mod binaries;
pub mod docker;
pub mod os;

fn main() {
    _ = thread::spawn(|| {
        docker::creating_directory();
        //docker::copy_docker_compose();

        sleep(Duration::from_secs(1)) //SÓ PARA DEMORAR UM POUCO PARA APAGAR A PASTA
    })
    .join();

    //docker::execute_docker_compose();

    //docker::undo();
}
