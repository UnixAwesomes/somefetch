use colored::*;
use std::env;
use std::process::Command;

fn get_os_name() -> Option<String> {
    #[cfg(any(target_os = "linux", target_os = "android"))]
    {
        let output = Command::new("uname").arg("-rm").output().ok()?;
        Some(String::from_utf8(output.stdout).ok()?.trim().to_string())
    }
    #[cfg(any(target_os = "openbsd", target_os = "netbsd"))]
    {
        let output = Command::new("uname").arg("-srm").output().ok()?;
        Some(String::from_utf8(output.stdout).ok()?.trim().to_string())
    }
    #[cfg(target_os = "freebsd")]
    {
        let output = Command::new("uname").arg("-rom").output().ok()?;
        Some(String::from_utf8(output.stdout).ok()?.trim().to_string())
    }
    #[cfg(target_os = "illumos")]
    {
        let output = Command::new("uname").arg("-v").output().ok()?;
        Some(String::from_utf8(output.stdout).ok()?.trim().to_string())
    }
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
    match Command::new("xbps-query").arg("-l").output() {
        Ok(_) => {
            let pkgx = Command::new("xbps-query").arg("-l").output().unwrap();
            let pkgsx = String::from_utf8(pkgx.stdout).unwrap();
            let pkgxs: Vec<&str> = pkgsx.split("\n").collect();
            pkg.push(format!("{pgk}(xbps), ", pgk = (pkgxs.len() - 1)));
        }
        Err(_why) => {}
    }

    match Command::new("apk").arg("info").output() {
        Ok(_) => {
            let pkga = Command::new("apk").arg("info").output().unwrap();
            let pkgsa = String::from_utf8(pkga.stdout).unwrap();
            let pkgas: Vec<&str> = pkgsa.split("\n").collect();
            pkg.push(format!("{pgk}(apk), ", pgk = (pkgas.len() - 1)));
        }
        Err(_why) => {}
    }

    match Command::new("flatpak").arg("list").output() {
        Ok(_) => {
            let pkgf = Command::new("flatpak").arg("list").output().unwrap();
            let pkgsf = String::from_utf8(pkgf.stdout).unwrap();
            let pkgfs: Vec<&str> = pkgsf.split("\n").collect();
            pkg.push(format!("{pgk}(flatpak), ", pgk = (pkgfs.len() - 1)));
        }
        Err(_why) => {}
    }

    match Command::new("dpkg-query")
        .args(["-f", "'.\n'", "-W"])
        .output()
    {
        Ok(_) => {
            let pkgf = Command::new("dpkg-query")
                .args(["-f", "'.\n'", "-W"])
                .output()
                .unwrap();
            let pkgsf = String::from_utf8(pkgf.stdout).unwrap();
            let pkgfs: Vec<&str> = pkgsf.split("\n").collect();
            pkg.push(format!("{pgk}(apt), ", pgk = (pkgfs.len() - 1)));
        }
        Err(_why) => {}
    }

    match Command::new("dnf").args(["list", "installed"]).output() {
        Ok(_) => {
            let pkgf = Command::new("dnf")
                .args(["list", "installed"])
                .output()
                .unwrap();
            let pkgsf = String::from_utf8(pkgf.stdout).unwrap();
            let pkgfs: Vec<&str> = pkgsf.split("\n").collect();
            pkg.push(format!("{pgk}(dnf), ", pgk = (pkgfs.len() - 1)));
        }
        Err(_why) => {}
    }

    match Command::new("pacman").args(["-Q", "-q"]).output() {
        Ok(_) => {
            let pkgf = Command::new("pacman").args(["-Q", "-q"]).output().unwrap();
            let pkgsf = String::from_utf8(pkgf.stdout).unwrap();
            let pkgfs: Vec<&str> = pkgsf.split("\n").collect();
            pkg.push(format!("{pgk}(pacman), ", pgk = (pkgfs.len() - 1)));
        }
        Err(_why) => {}
    }

    match Command::new("qlist").arg("-I").output() {
        Ok(_) => {
            let pkgf = Command::new("qlist").arg("-I").output().unwrap();
            let pkgsf = String::from_utf8(pkgf.stdout).unwrap();
            let pkgfs: Vec<&str> = pkgsf.split("\n").collect();
            pkg.push(format!("{pgk}(portage), ", pgk = (pkgfs.len() - 1)));
        }
        Err(_why) => {}
    }

    match Command::new("zypper")
        .arg("se")
        .arg("--installed-only")
        .output()
    {
        Ok(_) => {
            let pkgf = Command::new("zypper")
                .args(["se", "--installed-only"])
                .output()
                .unwrap();
            let pkgsf = String::from_utf8(pkgf.stdout).unwrap();
            let pkgfs: Vec<&str> = pkgsf.split("\n").collect();
            pkg.push(format!("{pgk}(zypper), ", pgk = (pkgfs.len() - 1)));
        }
        Err(_why) => {}
    }

    match Command::new("nix-env")
        .args(["-qa", "--installed", "\"*\""])
        .output()
    {
        Ok(_) => {
            let pkgf = Command::new("nix-env")
                .args(["-qa", "--installed", "\"*\""])
                .output()
                .unwrap();
            let pkgsf = String::from_utf8(pkgf.stdout).unwrap();
            let pkgfs: Vec<&str> = pkgsf.split("\n").collect();
            pkg.push(format!("{pgk}(nix), ", pgk = (pkgfs.len() - 1)));
        }
        Err(_why) => {}
    }

    match Command::new("pkg").arg("info").output() {
        Ok(_) => {
            let pkgf = Command::new("pkg").arg("info").output().unwrap();
            let pkgsf = String::from_utf8(pkgf.stdout).unwrap();
            let pkgfs: Vec<&str> = pkgsf.split("\n").collect();
            pkg.push(format!("{pgk}(pkg), ", pgk = (pkgfs.len() - 1)));
        }
        Err(_why) => {}
    }

    match Command::new("pkgin").arg("list").output() {
        Ok(_) => {
            let pkgf = Command::new("pkgin").arg("list").output().unwrap();
            let pkgsf = String::from_utf8(pkgf.stdout).unwrap();
            let pkgfs: Vec<&str> = pkgsf.split("\n").collect();
            pkg.push(format!("{pgk}(pkgin), ", pgk = (pkgfs.len() - 1)));
        }
        Err(_why) => {}
    }

    match Command::new("pkg_info").output() {
        Ok(_) => {
            let pkgf = Command::new("pkg_info").output().unwrap();
            let pkgsf = String::from_utf8(pkgf.stdout).unwrap();
            let pkgfs: Vec<&str> = pkgsf.split("\n").collect();
            pkg.push(format!("{pgk}(pkg_add), ", pgk = (pkgfs.len() - 1)));
        }
        Err(_why) => {}
    }

    match Command::new("snap").arg("list").output() {
        Ok(_) => {
            let pkgf = Command::new("snap").arg("list").output().unwrap();
            let pkgsf = String::from_utf8(pkgf.stdout).unwrap();
            let pkgfs: Vec<&str> = pkgsf.split("\n").collect();
            pkg.push(format!("{pgk}(snapd), ", pgk = (pkgfs.len() - 1)));
        }
        Err(_why) => {}
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
