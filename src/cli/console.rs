use chrono as time;
use owo_colors::OwoColorize;

pub fn log(message: &str) {
    println!(
        "[{}] {}",
        time::Local::now().format("%H:%M:%S").yellow(),
        message
    )
}

pub fn err(message: &str) {
    eprintln!(
        "[{}] [{}] {}",
        time::Local::now().format("%H:%M:%S").yellow(),
        "ERROR".red(),
        message
    )
}

pub fn success(message: &str) {
    println!(
        "[{}] [{}] {}",
        time::Local::now().format("%H:%M:%S").yellow(),
        "SUCCESS".green(),
        message
    )
}

pub fn info(message: &str) {
    println!(
        "[{}] [{}] {}",
        time::Local::now().format("%H:%M:%S").yellow(),
        "INFO".blue(),
        message
    )
}

pub fn warn(message: &str) {
    eprintln!(
        "[{}] [{}] {}",
        time::Local::now().format("%H:%M:%S").yellow(),
        "WARNING".bright_yellow(),
        message
    )
}
