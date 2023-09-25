use std::process::Command;

#[test]
fn check_stderr_stdout() {
    let output = Command::new("cargo")
        .args(["test", "-F", "colored/no-color"])
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("stderr = {stderr}\n\nstdout = {stdout}");
}
