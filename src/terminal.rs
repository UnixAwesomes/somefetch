use std::env;

pub fn get_terminal() -> String {
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
