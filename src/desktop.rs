pub fn get_desktop() -> String {
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