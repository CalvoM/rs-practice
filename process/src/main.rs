use std::process::Command;

fn main() {
    let out = Command::new("ls")
        .arg("-lah")
        .output()
        .expect("Failed to run ls");
    println!("{:?}", out.stdout);
    let cmd = Command::new("which").arg("git").unwrap();
}
