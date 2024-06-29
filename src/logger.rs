use std::io::Write;

#[cfg(debug_assertions)]
pub fn setup_logger() {
    use log::LevelFilter;

    env_logger::Builder::from_default_env()
        .format(|buf, record| {
            writeln!(
                buf,
                "[{} {}\t{}] - {}",
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S%.3f%:z"),
                record.level(),
                record.target(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Debug)
        .init();
}

#[cfg(not(debug_assertions))]
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
        .default_filter_or("info,headless_chrome=warn")
        .default_write_style_or("always");

    env_logger::Builder::from_env(env)
        .format(|buf, record| {
            writeln!(
                buf,
                "[{} {}\t{}] - {}",
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S%.3f%:z"),
                record.level(),
                record.target(),
                record.args()
            )
        })
        .target(env_logger::Target::Pipe(target))
        .init();
}
