// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::{Context, Result};
use time_grasp_lib::model;

fn main() -> Result<()> {
    model::main().context("model run failed")?;
    Ok(time_grasp_lib::run())
}
