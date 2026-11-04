use std::{
    process::Command,
    time::{Duration, Instant},
};

use conpty::Process;

// The conhost.exe that CreatePseudoConsole starts as a child of this process.
fn consoles() -> usize {
    let filter = format!(
        "Name='conhost.exe' AND ParentProcessId={}",
        std::process::id()
    );
    let count = format!("@(Get-CimInstance Win32_Process -Filter \"{filter}\").Count");
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", &count])
        .output()
        .unwrap();
    String::from_utf8_lossy(&output.stdout)
        .trim()
        .parse()
        .unwrap()
}

#[test]
fn failed_spawn_frees_its_console() {
    for _ in 0..3 {
        assert!(Process::spawn(Command::new(r"C:\missing\program.exe")).is_err());
    }
    let freeing = Instant::now();
    while consoles() > 0 {
        assert!(
            freeing.elapsed() < Duration::from_secs(10),
            "conhost.exe still runs"
        );
    }
}
