// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tokio::main]
async fn main() -> backend::error::Result<()> {
    env_logger::Builder::from_default_env()
        .filter_module("backend", log::LevelFilter::Debug)
        .filter_module("frontend", log::LevelFilter::Debug)
        .filter_module("model", log::LevelFilter::Debug)
        .filter_module("time_grasp_lib", log::LevelFilter::Debug)
        .filter_module("time_grasp", log::LevelFilter::Debug)
        .filter_level(log::LevelFilter::Warn)
        .init();

    log::debug!("main invoked");
    match time_grasp_lib::run() {
        Ok(()) => (),
        Err(e) => println!("{e:?}"),
    }

    Ok(())
}
