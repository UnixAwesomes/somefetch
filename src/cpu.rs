use std::process::Command;
use std::fs::read_to_string;

pub fn get_cpu_name() -> Option<String> {
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