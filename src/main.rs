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
    #[cfg(any(target_os = "freebsd", target_os = "openbsd", target_os = "netbsd"))]
    {
        let output = Command::new("sysctl")
            .args(["-n", "hw.model"])
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
    let shell = env::var("SHELL").expect("Unknown");
    let parts: Vec<&str> = shell.split('/').collect();
    parts.last().unwrap().to_string()
}


fn get_pkgs() -> String {
    let mut pkg: Vec<String> = Vec::new();

    if let Ok(pkgf) = Command::new("xbps-query").arg("-l").output() {
        let pkgsf = String::from_utf8_lossy(&pkgf.stdout);
        let pkgfs: Vec<&str> = pkgsf.split("\n").collect();
        pkg.push(format!("{pgk}(xbps), ", pgk = (pkgfs.len() - 1)));
    }

    if let Ok(pkgf) = Command::new("apk").arg("info").output() {
        let pkgsf = String::from_utf8_lossy(&pkgf.stdout);
        let pkgfs: Vec<&str> = pkgsf.split("\n").collect();
        pkg.push(format!("{pgk}(apk), ", pgk = (pkgfs.len() - 1)));
    }

    if let Ok(pkgf) = Command::new("flatpak").arg("list").output() {
        let pkgsf = String::from_utf8_lossy(&pkgf.stdout);
        let pkgfs: Vec<&str> = pkgsf.split("\n").collect();
        pkg.push(format!("{pgk}(flatpak), ", pgk = (pkgfs.len() - 1)));
    }

    if let Ok(pkgf) = Command::new("dpkg-query").args(["-f", "'.\n'", "-W"]).output() {
        let pkgsf = String::from_utf8_lossy(&pkgf.stdout);
        let pkgfs: Vec<&str> = pkgsf.split("\n").collect();
        pkg.push(format!("{pgk}(apt), ", pgk = (pkgfs.len() - 1)));
    }

    if let Ok(pkgf) = Command::new("dnf").args(["list", "installed"]).output() {
        let pkgsf = String::from_utf8_lossy(&pkgf.stdout);
        let pkgfs: Vec<&str> = pkgsf.split("\n").collect();
        pkg.push(format!("{pgk}(dnf), ", pgk = (pkgfs.len() - 1)));
    }

    if let Ok(pkgf) = Command::new("pacman").args(["-Q", "-q"]).output() {
        let pkgsf = String::from_utf8_lossy(&pkgf.stdout);
        let pkgfs: Vec<&str> = pkgsf.split("\n").collect();
        pkg.push(format!("{pgk}(pacman), ", pgk = (pkgfs.len() - 1)));
    }

    if let Ok(pkgf) = Command::new("qlist").arg("-I").output() {
        let pkgsf = String::from_utf8_lossy(&pkgf.stdout);
        let pkgfs: Vec<&str> = pkgsf.split("\n").collect();
        pkg.push(format!("{pgk}(portage), ", pgk = (pkgfs.len() - 1)));
    }

    if let Ok(pkgf) = Command::new("zypper").args(["se", "--installed-only"]).output() {
        let pkgsf = String::from_utf8_lossy(&pkgf.stdout);
        let pkgfs: Vec<&str> = pkgsf.split("\n").collect();
        pkg.push(format!("{pgk}(zypper), ", pgk = (pkgfs.len() - 1)));
    }

    if let Ok(pkgf) = Command::new("nix-env").args(["-qa", "--installed", "\"*\""]).output() {
        let pkgsf = String::from_utf8_lossy(&pkgf.stdout);
        let pkgfs: Vec<&str> = pkgsf.split("\n").collect();
        pkg.push(format!("{pgk}(nix), ", pgk = (pkgfs.len() - 1)));
    }


    if let Ok(pkgf) = Command::new("pkg").arg("info").output() {
            let pkgsf = String::from_utf8_lossy(&pkgf.stdout);
            let pkgfs: Vec<&str> = pkgsf.split("\n").collect();
            pkg.push(format!("{pgk}(pkg), ", pgk = (pkgfs.len() - 1)));
        }

    if let Ok(pkgf) = Command::new("pkgin").arg("info").output() {
            let pkgsf = String::from_utf8_lossy(&pkgf.stdout);
            let pkgfs: Vec<&str> = pkgsf.split("\n").collect();
            pkg.push(format!("{pgk}(pkgin), ", pgk = (pkgfs.len() - 1)));
        }    

    if let Ok(pkgf) = Command::new("pkg_info").output() {
            let pkgsf = String::from_utf8_lossy(&pkgf.stdout);
            let pkgfs: Vec<&str> = pkgsf.split("\n").collect();
            pkg.push(format!("{pgk}(pkg_info), ", pgk = (pkgfs.len() - 1)));
        }   
    

    if let Ok(pkgf) = Command::new("snap").arg("list").output() {
            let pkgsf = String::from_utf8_lossy(&pkgf.stdout);
            let pkgfs: Vec<&str> = pkgsf.split("\n").collect();
            pkg.push(format!("{pgk}(snap), ", pgk = (pkgfs.len() - 1)));
        }   

    let mut pkgs: String = pkg.into_iter().collect::<String>();
    let mut v: Vec<char> = pkgs.chars().collect();
    v.remove(v.len() - 2);
    pkgs = v.into_iter().collect();
    pkgs
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
