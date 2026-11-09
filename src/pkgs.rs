use std::env;
use std::process::Command;

pub fn get_pkgs() -> String {
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
        ("pkgin", &["list"], "pkgin"),
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
        .copied()
        .filter_map(|(cmd, args, tag)| {
            // pkg_add is the preferred option for NetBSD
			if cmd == "pkgin" && env::consts::OS == "netbsd" {
			    return None;
			}
			Command::new(cmd)
                .args(args)
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