use std::io;

fn main() {
    let mut jaewhaun = String::new();
    io::stdin().read_line(&mut jaewhaun).expect("input error");
    let jaehwaun = jaewhaun.trim();
    let mut doctor = String::new();
    io::stdin().read_line(&mut doctor).expect("input error");
    let doctor = doctor.trim();

    if jaehwaun.len() >= doctor.len() {
        println!("go");
    } else {
        println!("no");
    }
}
