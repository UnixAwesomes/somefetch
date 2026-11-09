use std::process::Command;

pub fn get_host_name() -> Option<String> {
    let output = Command::new("hostname").output().ok()?;
    Some(str::from_utf8(&output.stdout).ok()?.trim().to_string())
}