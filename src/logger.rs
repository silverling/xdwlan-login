use std::io::Write;

const TIME_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3f%:z";

#[cfg(debug_assertions)] // Debug mode.
pub fn setup_logger() {
    use log::LevelFilter;

    env_logger::Builder::from_default_env()
        .format(|buf, record| {
            writeln!(
                buf,
                "[{} {} {}] {}",
                chrono::Local::now().format(TIME_FORMAT),
                record.level(),
                record.target(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Debug)
        .init();
}

#[cfg(all(not(debug_assertions), target_os = "linux"))] // Release mode on Linux.
pub fn setup_logger() {
    // Allow logger to be configured via an environment variable.
    let env = env_logger::Env::default()
        .default_filter_or("info,headless_chrome=error")
        .default_write_style_or("always");

    env_logger::Builder::from_env(env)
        .format(|buf, record| {
            writeln!(
                buf,
                "[{} {} {}] {}",
                chrono::Local::now().format(TIME_FORMAT),
                record.level(),
                record.target(),
                record.args()
            )
        })
        .init();
}

#[cfg(all(not(debug_assertions), target_os = "windows"))] // Release mode on Windows.
pub fn setup_logger() {
    use crate::utils::get_program_folder;

    let program_folder = get_program_folder();
    let log_file_path = format!("{}/log.txt", program_folder);

    let target = Box::new(
        std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(log_file_path.as_str())
            .expect("Failed to open log file."),
    );

    // Allow logger to be configured via an environment variable.
    let env = env_logger::Env::default()
        .default_filter_or("info,headless_chrome=error")
        .default_write_style_or("always");

    env_logger::Builder::from_env(env)
        .format(|buf, record| {
            writeln!(
                buf,
                "[{} {} {}] {}",
                chrono::Local::now().format(TIME_FORMAT),
                record.level(),
                record.target(),
                record.args()
            )
        })
        .target(env_logger::Target::Pipe(target))
        .init();
}
