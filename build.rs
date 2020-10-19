use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=static/bulma.min.css");
    let mut child =
        Command::new("npm")
        .args(&["run", "build"])
        .spawn()
        .unwrap();
    child.wait().unwrap();
}
