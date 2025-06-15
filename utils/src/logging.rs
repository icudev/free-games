pub fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}] {} {}: {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(fern::log_file("./log.log")?)
        .level_for("tracing", log::LevelFilter::Error)
        .level_for("actix_server", log::LevelFilter::Error)
        .level_for("reqwest", log::LevelFilter::Error)
        .level_for("api", log::LevelFilter::Info)
        .level_for("bot", log::LevelFilter::Info)
        .level_for("scraper", log::LevelFilter::Info)
        .apply()?;
    Ok(())
}
