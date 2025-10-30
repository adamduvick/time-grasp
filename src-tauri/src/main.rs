// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use time_grasp_lib::model;

fn main() {
    model::main().unwrap();
    time_grasp_lib::run()
}
