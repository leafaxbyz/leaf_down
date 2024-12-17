// Prevents additional console window on Windows in release, DO NOT REMOVE!!



#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use leaf_down_lib::loader::log_init;

fn main() {
    log_init();
    leaf_down_lib::run()
}
