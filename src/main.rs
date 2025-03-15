use colored::*;
use std::env;
use std::str;
use std::process::Command;

fn get_os_name() -> Option<String> {
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
}

fn get_info_name(args: &str) -> Option<String> {
    let output = Command::new("uname").arg(args).output().ok()?;
    Some(String::from_utf8(output.stdout).ok()?.trim().to_string())
}

fn get_host_name() -> Option<String> {
    let output = Command::new("hostname").output().ok()?;
    Some(String::from_utf8(output.stdout).ok()?.trim().to_string())
}

fn get_cpu_name() -> Option<String> {
    #[cfg(any(target_os = "freebsd", target_os = "openbsd"))]
    {
        let output = Command::new("sysctl")
            .args(["-n", "hw.model"])
            .output()
            .ok()?;
        Some(String::from_utf8(output.stdout).ok()?.trim().to_string())
    }

    #[cfg(target_os = "netbsd")]
    {
        let output = Command::new("sysctl")
            .args(["-n", "machdep.cpu_brand"])
            .output()
            .ok()?;
        Some(String::from_utf8(output.stdout).ok()?.trim().to_string())
    }

    #[cfg(target_os = "illumos")]
    {
        let output = Command::new("kstat")
            .args(["-p", "cpu_info:::brand"])
            .output()
            .ok()?;
      
        let output_str = str::from_utf8(&output.stdout).ok()?;

        for line in output_str.lines() {
            if let Some((_, model)) = line.split_once('\t') {
                return Some(model.trim().to_string());
            }
        }
        None
    }
    
    #[cfg(any(target_os = "linux", target_os = "android"))]
    {
        let info = std::fs::read_to_string("/proc/cpuinfo").ok()?;
        #[cfg(target_os = "android")]
        let cpu_string = "Hardware";
        #[cfg(target_os = "linux")]
        let cpu_string = "model name";
        for line in info.lines() {
            if line.starts_with(cpu_string) {
                let (_, name) = line.split_once(':')?;
                return Some(name.trim().to_owned());
            }
        }
        None
    }
}

fn get_shell() -> String {
    env::var("SHELL")
        .ok()
        .and_then(|s| s.split('/').last().map(|s| s.to_string()))
        .unwrap_or_else(|| "Unknown".into())
}

fn get_pkgs() -> String {
    let package_managers: &[(&str, &[&str], &str)] = &[
        ("xbps-query", &["-l"][..], "xbps"),
        ("apk", &["info"], "apk"),
        ("rpm", &["-qa"], "rpm"),
        ("flatpak", &["list"], "flatpak"),
        ("dpkg-query", &["-f", "'.\\n'", "-W"], "apt"),
        ("pacman", &["-Q", "-q"], "pacman"),
        ("qlist", &["-I"], "portage"),
        ("pkg", &["info"], "pkg"),
        ("pkgin", &["info"], "pkgin"),
        ("pkg_info", &[], "pkg_info"),
        ("snap", &["list"], "snap"),
        ("eopkg", &["li"], "eopkg"),
        ("opkg", &["list-installed"], "opkg"),
        ("nix-user-pkgs", &[], "nix-user"),
        ("nix-store", &["-qR", "/run/current-system/sw"], "nix-system"),
    ];

    package_managers
        .iter()
        .filter_map(|(cmd, args, tag)| {
            Command::new(cmd)
                .args(*args)
                .output()
                .ok()
                .and_then(|output| {
                    let count = String::from_utf8_lossy(&output.stdout)
                        .lines()
                        .count();
                    (count > 0).then(|| format!("{}({})", count, tag))
                })
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn main() {
    let os_name = get_os_name().unwrap_or("Unknown".to_string());
    let cpu = get_cpu_name().unwrap_or("Unknown".to_string());
    let hostname = get_host_name().unwrap_or("Unknown".to_string());
    let shell = get_shell();
    let desktop = env::var("XDG_CURRENT_DESKTOP").unwrap_or_else(|_| "Unknown".to_string());


    println!(
        "
    {}

    {}   {}
    {} {}
    {}   {}
    {}  {}
    {}  {}
    ",
        "~ system info ~".bright_cyan(),
        "host".bright_yellow(),
        hostname,
        "kernel".bright_green(),
        os_name,
        "pkgs".bright_magenta(),
        get_pkgs(),
        "shell".bright_blue(),
        shell,
        "de/wm".bright_red(),
        desktop,
    );

    println!(
        "    {}

    {}   {}
    ",
        "~ hardware info ~".bright_cyan(),
        "cpu".bright_green(),
        cpu,
    );
}
