use std::env;
use std::fs;
use std::thread;
use std::time::{Duration, Instant};
use std::path::PathBuf;


const POLL_DURATION: Duration = Duration::from_secs(1);
const BATT_UPDATE_INTERVAL: u64 = 30;

fn main() {
    let battery: String = env::args().nth(1).expect("battery ID not provided");

    let mut battery_info = update_battery_info(&battery);

    let start = Instant::now();
    loop {
        let duration = start.elapsed().as_secs();
        if duration % BATT_UPDATE_INTERVAL == 0 {
            battery_info = update_battery_info(&battery);
        }
        let datetime = chrono::Local::now().to_rfc2822();

        println!("{0} | {1}", battery_info, datetime);

        thread::sleep(POLL_DURATION);
    }
}

fn update_battery_info(battery: &String) -> String {
    let mut path = PathBuf::from("/sys/class/power_supply");
    path.push(battery);
    let status = fs::read_to_string(path.join("status")).expect("Failed to read battery status");
    let status = status.trim();
    let charge_full = fs::read_to_string(path.join("charge_full")).expect("Failed to read battery charge_full");
    let charge_now = fs::read_to_string(path.join("charge_now")).expect("Failed to read battery charge_now");
    let charge_full: u64 = charge_full.trim().parse().expect("Not a valid charge_full value");
    let charge_now: u64 = charge_now.trim().parse().expect("Not a valid charge_now value");
    let charge_percentage = (charge_now * 100) / charge_full;

    return format!("{}: {}%", status, charge_percentage);
}




