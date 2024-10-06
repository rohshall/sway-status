use std::env;
use std::fs;
use std::thread;
use std::time::Duration;
use chrono::{DateTime, Local};
use std::path::PathBuf;


const POLL_DURATION: Duration = Duration::from_secs(60);

fn main() {
    let battery: String = env::args().nth(1).expect("battery ID not provided");
    let path = format!("/sys/class/power_supply/{}", battery);
    let path = PathBuf::from(path);

    loop {
        let battery_info = update_battery_info(&path);
        let current_local: DateTime<Local> = Local::now();
        let datetime = current_local.format("%a,%v %H:%M");

        println!("{} | {}", battery_info, datetime);

        thread::sleep(POLL_DURATION);
    }
}

fn update_battery_info(path: &PathBuf) -> String {
    let status = fs::read_to_string(path.join("status")).expect("Failed to read battery status");
    let status = status.trim();
    let charge_full = fs::read_to_string(path.join("charge_full")).expect("Failed to read battery charge_full");
    let charge_now = fs::read_to_string(path.join("charge_now")).expect("Failed to read battery charge_now");
    let charge_full: u64 = charge_full.trim().parse().expect("Not a valid charge_full value");
    let charge_now: u64 = charge_now.trim().parse().expect("Not a valid charge_now value");
    let charge_percentage = (charge_now * 100) / charge_full;

    return format!("{}: {}%", status, charge_percentage);
}




