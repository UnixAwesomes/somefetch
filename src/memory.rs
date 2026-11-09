use std::fs::read_to_string;
use std::process::Command;

pub fn parse_memory_value(line: &str) -> u64 {
    line.split_whitespace()
        .nth(1)
        .unwrap_or("0")
        .parse::<u64>()
        .unwrap_or(0)
}

pub fn get_memory() -> String {
    #[cfg(any(target_os = "linux", target_os = "netbsd", target_os = "android"))]
    {
        let kb_to_gb = |kilobytes: u64| kilobytes as f64 / (1024.0 * 1024.0);

        let info = read_to_string("/proc/meminfo")
            .ok()
            .unwrap_or_else(|| "Unknown".to_string());
        let mut mem_total: u64 = 0;

        #[cfg(any(target_os = "linux", target_os = "android"))]
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
    #[cfg(target_os = "illumos")]
    {
        let pages_to_gb = |pages: u64| (pages * 4) as f64 / 1024.0 / 1024.0;

        let pagestotal = Command::new("kstat")
            .args(["-p", "unix:0:system_pages:pagestotal"])
            .output()
            .ok();

        let pagesfree = Command::new("kstat")
            .args(["-p", "unix:0:system_pages:pagesfree"])
            .output()
            .ok();

        let mut pages_total: u64 = 0;
        let mut pages_free: u64 = 0;

        if let Some(output) = pagestotal {
            if let Ok(output_str) = str::from_utf8(&output.stdout) {
                for line in output_str.lines() {
                    if line.starts_with("unix:0:system_pages:pagestotal\t") {
                        pages_total = parse_memory_value(line);
                        break;
                    }
                }
            }
        }

        if let Some(output) = pagesfree {
            if let Ok(output_str) = str::from_utf8(&output.stdout) {
                for line in output_str.lines() {
                    if line.starts_with("unix:0:system_pages:pagesfree\t") {
                        pages_free = parse_memory_value(line);
                        break;
                    }
                }
            }
        }

        let used = pages_total.saturating_sub(pages_free);
        return format!("{:.2} GiB / {:.2} GiB", pages_to_gb(used), pages_to_gb(pages_total));
    }
    #[cfg(target_os = "freebsd")]
    {
        let kb_to_gb = |kilobytes: u64| kilobytes as f64 / 1024.0 / 1024.0 / 1024.0;

        let memtotal = Command::new("sysctl")
            .args(["hw.physmem"])
            .output()
            .ok();

        let memfree = Command::new("sysctl")
            .args(["hw.usermem"])
            .output()
            .ok();

        let mut mem_total: u64 = 0;
        let mut mem_free: u64 = 0;

        if let Some(output) = memtotal {
            if let Ok(output_str) = str::from_utf8(&output.stdout) {
                for line in output_str.lines() {
                    if line.starts_with("hw.physmem:") {
                        mem_total = parse_memory_value(line);
                        break;
                    }
                }
            }
        }

        if let Some(output) = memfree {
            if let Ok(output_str) = str::from_utf8(&output.stdout) {
                for line in output_str.lines() {
                    if line.starts_with("hw.usermem:") {
                        mem_free = parse_memory_value(line);
                        break;
                    }
                }
            }
        }

        let used = mem_total.saturating_sub(mem_free);
        return format!("{:.2} GiB / {:.2} GiB", kb_to_gb(used), kb_to_gb(mem_total));
    }
}