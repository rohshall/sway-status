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
        let current_local: DateTime<Local> = Local::now();
        let datetime = current_local.format("%a,%v %H:%M");
        let battery_status = get_battery_status(&path);
        let battery_charge_percentage = get_battery_charge_percentage(&path);
        let battery_status = match battery_status.as_str() {
            "full" => "󰂅",
            "charging" => match battery_charge_percentage {
                0..33 => "󰂆",
                33..66 => "󱊥",
                66..99 => "󱊦",
                _ => "󰂑"
            },
            "discharging" => match battery_charge_percentage {
                0..33 => "󱊡",
                33..66 => "󱊢",
                66..99 => "󱊣",
                _ => "󰂑"
            },
            _ => "󰂑"
        };

        println!("{} {}%   {}", battery_status, battery_charge_percentage, datetime);

        thread::sleep(POLL_DURATION);
    }
}

fn get_battery_status(path: &PathBuf) -> String {
    let battery_status = fs::read_to_string(path.join("status")).expect("Failed to read battery status");
    String::from(battery_status.trim().to_lowercase())
}

fn get_battery_charge_percentage(path: &PathBuf) -> u64 {
    let charge_full = fs::read_to_string(path.join("charge_full")).expect("Failed to read battery charge_full");
    let charge_now = fs::read_to_string(path.join("charge_now")).expect("Failed to read battery charge_now");
    let charge_full: u64 = charge_full.trim().parse().expect("Not a valid charge_full value");
    let charge_now: u64 = charge_now.trim().parse().expect("Not a valid charge_now value");
    (charge_now * 100) / charge_full
}




