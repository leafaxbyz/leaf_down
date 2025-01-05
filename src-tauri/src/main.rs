// Prevents additional console window on Windows in release, DO NOT REMOVE!!

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use leaf_down_lib::loader::init::log_init;
use log::info;

fn main() {
    log_init();
    info!("项目正在启动....");
    leaf_down_lib::run()
}
