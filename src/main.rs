use colored::*;
use std::env;
use std::str;
use std::process::Command;
use std::fs::read_to_string;

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

fn get_info_name(args: &str) -> Option<String> {
    let output = Command::new("uname").arg(args).output().ok()?;
    Some(str::from_utf8(&output.stdout).ok()?.trim().to_string())
}

fn get_host_name() -> Option<String> {
    let output = Command::new("hostname").output().ok()?;
    Some(str::from_utf8(&output.stdout).ok()?.trim().to_string())
}

fn get_cpu_name() -> Option<String> {
    #[cfg(any(target_os = "freebsd", target_os = "openbsd"))]
    {
        let output = Command::new("sysctl")
            .args(["-n", "hw.model"])
            .output()
            .ok()?;
        Some(str::from_utf8(&output.stdout).ok()?.trim().to_string())
    }

    #[cfg(target_os = "netbsd")]
    {
        let output = Command::new("sysctl")
            .args(["-n", "machdep.cpu_brand"])
            .output()
            .ok()?;
        Some(str::from_utf8(&output.stdout).ok()?.trim().to_string())
    }

    #[cfg(target_os = "macos")]
    {
        let output = Command::new("sysctl")
            .args(["-n", "machdep.cpu.brand_string"])
            .output()
            .ok()?;
        Some(str::from_utf8(&output.stdout).ok()?.trim().to_string())
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

    #[cfg(target_os = "windows")]
    {
            let mut cpu = String::new();

            let output = Command::new("wmic")
                .args(["cpu", "get", "name"])
                .output()
                .ok()?;

            let output_str = str::from_utf8(&output.stdout).ok()?;

            for line in output_str.lines() {
                if !line.contains("Name") && line.trim().len() > 0{
                    cpu = line.to_string();
                }

            }
            Some(cpu)
    }
    #[cfg(target_os = "android")]
    {
        let output = Command::new("lscpu")
            .output()
            .ok()?;

        let output_str = str::from_utf8(&output.stdout).ok()?;

        for line in  output_str.lines() {
            if line.starts_with("Model name") {
                let (_, name) = line.split_once(':')?;
                return Some(name.trim().to_owned());
            }
        }
        None
    }

    #[cfg(target_os = "linux")]
    {
        let info = read_to_string("/proc/cpuinfo").ok()?;
        for line in info.lines() {
            if line.starts_with("model name") {
                let (_, name) = line.split_once(':')?;
                return Some(name.trim().to_owned());
            }
        }
        None
    }
}

fn get_shell() -> String {
    #[cfg(not(windows))]
    {
        env::var("SHELL")
            .ok()
            .and_then(|s| s.split('/').last().map(|s| s.to_string()))
            .unwrap_or_else(|| "Unknown".into())
    }
    #[cfg(target_os="windows")]
    {
        "PowerShell".to_string()
    }
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
        ("brew", &["list"], "homebrew"),
        ("pkgin", &["info"], "pkgin"),
        ("pkg_info", &[], "pkg_info"),
        ("snap", &["list"], "snap"),
        ("eopkg", &["li"], "eopkg"),
        ("scoop.cmd", &["list"], "scoop"),
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

fn get_desktop() -> String {
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        let desktop_env = std::env::var("XDG_CURRENT_DESKTOP")
            .ok()
            .unwrap_or_else(|| "Unknown".into());

        let display_backend_result = std::env::var("XDG_SESSION_TYPE");

        let mut display_backend = display_backend_result.unwrap_or_default();
        if let Some(c) = display_backend.as_mut_str().get_mut(0..1) {
            c.make_ascii_uppercase();
        }

        let display_backend = if display_backend.is_empty() {
            "Unknown"
        } else {
            &display_backend
        }
            .to_string();

        format!("{desktop_env} ({display_backend})")
    }
    #[cfg(target_os="windows")]
    {
        "Fluent".to_string()
    }
    #[cfg(target_os="macos")]
    {
        "Quartz Compositor".to_string()
    }
}

fn get_terminal() -> String {
    #[cfg(not(windows))]
    {
        
        if let Ok(mut term_program) = env::var("TERM_PROGRAM") {
            match term_program.as_str() {
                "kgx" => term_program = "gnome-console".to_string(),
                "iTerm.app" => term_program = "iTerm2".to_string(),
                "Terminal.app" => term_program = "Apple Terminal".to_string(),
                "Hyper" => term_program = "HyperTerm".to_string(),
                _ => {}
            }
            return term_program;
        }

        if let Ok(mut term) = env::var("TERM") {
            match term.as_str() {
                "rxvt-unicode-256color" => term = "urxvt".to_string(),
                "xterm-256color" => term = "xterm".to_string(),
                "xterm-kitty" => term = "kitty".to_string(),
                "linux" => term = "tty".to_string(),
                _ => {}
            }
            return term;
        }
        "Unknown".to_string()
    }
    #[cfg(windows)]
    {
        "Windows Terminal".to_string()
    }
}

fn parse_memory_value(line: &str) -> u64 {
    line.split_whitespace()
        .nth(1)
        .unwrap_or("0")
        .parse::<u64>()
        .unwrap_or(0)
}

fn get_memory() -> String {
    let kb_to_gb = |kilobytes: u64| kilobytes as f64 / (1024.0 * 1024.0);

    let info = read_to_string("/proc/meminfo")
        .ok()
        .unwrap_or_else(|| "Unknown".to_string());
    let mut mem_total: u64 = 0;

        #[cfg(target_os = "linux")]
        {
            let mut mem_available: u64 = 0;
            for line in info.lines() {
                    if line.starts_with("MemTotal:") {
                        mem_total = parse_memory_value(&line);
                    } else if line.starts_with("MemAvailable:") {
                        mem_available = parse_memory_value(&line);
                    }
                }


            let used = mem_total - mem_available;
            return format!("{:.2} GiB / {:.2} GiB", kb_to_gb(used), kb_to_gb(mem_total));
        }

        #[cfg(target_os = "netbsd")]
        {
            let mut mem_free: u64 = 0;
            for line in info.lines() {
                    if line.starts_with("MemTotal:") {
                        mem_total = parse_memory_value(&line);
                    } else if line.starts_with("MemAvailable:") || line.starts_with("MemFree:") {
                        mem_free = parse_memory_value(&line);
                    }
                }


            let used = mem_total - mem_free;
            return format!("{:.2} GiB / {:.2} GiB", kb_to_gb(used), kb_to_gb(mem_total));
        }
    }


fn main() {
    let os_name = get_os_name().unwrap_or("Unknown".to_string());
    let cpu = get_cpu_name().unwrap_or("Unknown".to_string());
    let hostname = get_host_name().unwrap_or("Unknown".to_string());
    let pkgs = get_pkgs();
    let shell = get_shell();
    let desktop = get_desktop();
    let terminal = get_terminal();
    let ram = get_memory();


    println!(
        "
    {}

    {}   {}
    {} {}
    {}   {}
    {}  {}
    {}  {}
    {}   {}
    ",
        "~ system info ~".bright_cyan(),
        "host".bright_yellow(),
        hostname,
        "kernel".bright_green(),
        os_name,
        "pkgs".bright_magenta(),
        pkgs,
        "shell".bright_blue(),
        shell,
        "de/wm".bright_red(),
        desktop,
        "term".bright_purple(),
        terminal,
    );

    println!(
        "    {}

    {}   {}
    {}   {}
    ",
        "~ hardware info ~".bright_cyan(),
        "cpu".bright_green(),
        cpu,
        "ram".bright_red(),
        ram,
    );
}