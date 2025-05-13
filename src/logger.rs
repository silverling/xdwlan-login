use std::io::Write;

use env_logger::fmt::Formatter;

const TIME_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3f%:z";

fn logger_formatter(buf: &mut Formatter, record: &log::Record<'_>) -> std::io::Result<()> {
    writeln!(
        buf,
        "[{} {} {}] {}",
        chrono::Local::now().format(TIME_FORMAT),
        record.level(),
        record.target(),
        record.args()
    )
}

#[cfg(debug_assertions)] // Debug mode.
pub fn setup_logger() {
    use log::LevelFilter;

    env_logger::Builder::from_default_env()
        .format(logger_formatter)
        .filter(None, LevelFilter::Debug)
        .init();
}

#[cfg(all(not(debug_assertions), target_os = "linux"))] // Release mode on Linux.
pub fn setup_logger() {
    // Allow logger to be configured via an environment variable.
    let env = env_logger::Env::default()
        .default_filter_or("info")
        .default_write_style_or("always");

    env_logger::Builder::from_env(env)
        .format(logger_formatter)
        .init();
}

#[cfg(all(not(debug_assertions), target_os = "windows"))] // Release mode on Windows.
pub fn setup_logger() {
    use crate::utils::get_program_folder;

    let program_folder = get_program_folder();
    let log_file_path = program_folder.join("log.txt");

    let target = Box::new(
        std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(log_file_path.to_str().unwrap())
            .expect("Failed to open log file."),
    );

    // Allow logger to be configured via an environment variable.
    let env = env_logger::Env::default()
        .default_filter_or("info")
        .default_write_style_or("always");

    env_logger::Builder::from_env(env)
        .format(logger_formatter)
        .target(env_logger::Target::Pipe(target))
        .init();
}
