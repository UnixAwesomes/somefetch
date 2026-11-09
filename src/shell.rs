use std::env;

pub fn get_shell() -> String {
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