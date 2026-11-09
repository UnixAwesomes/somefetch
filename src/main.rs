use colored::*;
use std::str;

use crate::memory::get_memory;
use crate::cpu::get_cpu_name;
use crate::host::get_host_name;
use crate::shell::get_shell;
use crate::terminal::get_terminal;
use crate::desktop::get_desktop;
use crate::pkgs::get_pkgs;
use crate::os::get_os_name;

mod desktop;
mod host;
mod memory;
mod os;
mod cpu;
mod terminal;
mod pkgs;
mod shell;

fn print_section(title: &str, rows: &[(&str, &str, Color)], width: usize) {
    println!();
    println!("    {}", title.bright_cyan());
    println!();
    for (label, value, color) in rows {
        println!("    {:width$} {}", label.color(*color), value, width = width);
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
    let memory = get_memory();

    let system = [
        ("host",   hostname.as_str(), Color::BrightYellow),
        ("kernel", os_name.as_str(),  Color::BrightGreen),
        ("pkgs",   pkgs.as_str(),     Color::BrightMagenta),
        ("shell",  shell.as_str(),    Color::BrightBlue),
        ("de/wm",  desktop.as_str(),  Color::BrightRed),
        ("term",   terminal.as_str(), Color::BrightMagenta),
    ];
    let hardware = [
        ("cpu", cpu.as_str(), Color::BrightGreen),
        ("ram", memory.as_str(), Color::BrightRed),
    ];

    let width = system.iter().chain(hardware.iter())
        .map(|(l, _, _)| l.len())
        .max()
        .unwrap_or(0);

    print_section("~ system info ~", &system, width);
    print_section("~ hardware info ~", &hardware, width);
    println!();
}
