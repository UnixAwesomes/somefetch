use std::process::Command;

pub fn get_os_name() -> Option<String> {
    #[cfg(any(target_os = "linux", target_os = "android"))]
    {
        get_info_name("-rm")
    }
    #[cfg(any(target_os = "openbsd", target_os = "netbsd"))]
    {
        get_info_name("-srm")
    }
    #[cfg(target_os = "freebsd")]
    {
        get_info_name("-rom")
    }
    #[cfg(target_os = "illumos")]
    {
        get_info_name("-v")
    }
    #[cfg(target_os = "macos")]
    {
	get_info_name("-sr")
    }
    #[cfg(target_os = "windows")]
    {
        let mut kernel = String::new();

        let output = Command::new("wmic")
            .args(&["OS", "get", "Caption"])
            .output()
            .ok()?;

        let output = str::from_utf8(&output.stdout).ok()?;

        for line in output.lines() {
            if !line.contains("Caption") && line.trim().len() > 0 {
                kernel = line.to_string();
            }
        }

        Some(kernel)
    }
}

pub fn get_info_name(args: &str) -> Option<String> {
    let output = Command::new("uname").arg(args).output().ok()?;
    Some(str::from_utf8(&output.stdout).ok()?.trim().to_string())
}