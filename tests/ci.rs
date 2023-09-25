use std::process::Command;

use cargo_pretty_test::parsing::parse_stderr;

#[test]
fn check_stderr_stdout() {
    let output = Command::new("cargo")
        .args(["test", "-F", "colored/no-color"])
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("stderr = {stderr:?}\n\nstdout = {stdout:?}");
}

#[test]
fn check_stderr() {
    let stderr = "\n     Finished test [optimized + debuginfo] target(s) in 0.20s
     Running unittests src/lib.rs (target/debug/deps/torrust_tracker-8c4e3a20c54a4e7b)
     Running unittests src/main.rs (target/debug/deps/torrust_tracker-5d2d33080cf6537c)";
    dbg!(parse_stderr(stderr));
}
