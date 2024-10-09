use std::env;
use std::fs;
use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use chrono::{DateTime, Local};
use std::path::PathBuf;


const POLL_DURATION: Duration = Duration::from_secs(60);

fn main() {
    let battery_id = env::args().nth(1).unwrap_or(String::from(""));
    loop {
        let current_local: DateTime<Local> = Local::now();
        let datetime = current_local.format("%a,%v %H:%M");

        let mem_info = get_mem_info();
        let load_avg = get_load_avg();
        // If battery ID is provided, show its status.
        if battery_id.is_empty() {
            println!("󰻠  {} {} {}  󰍛 {}/{}({}) GB     {}", load_avg[0], load_avg[1], load_avg[2], mem_info.get("MemAvailable").unwrap(), mem_info.get("MemTotal").unwrap(), mem_info.get("MemFree").unwrap(), datetime);
        } else {
            let (battery_status, battery_charge_percentage) = get_battery_info(&battery_id);
            println!("󰻠  {} {} {}  󰍛 {}/{}({}) GB   {} {}%     {}", load_avg[0], load_avg[1], load_avg[2], mem_info.get("MemAvailable").unwrap(), mem_info.get("MemTotal").unwrap(), mem_info.get("MemFree").unwrap(), battery_status, battery_charge_percentage, datetime);
        }
        // sleep
        thread::sleep(POLL_DURATION);
    }
}

fn get_battery_info(battery: &str) -> (String, u64) {
    let path = format!("/sys/class/power_supply/{}", battery);
    let path = PathBuf::from(path);
    let charge_full = fs::read_to_string(path.join("charge_full")).expect("Failed to read battery charge_full");
    let charge_now = fs::read_to_string(path.join("charge_now")).expect("Failed to read battery charge_now");
    let charge_full: u64 = charge_full.trim().parse().expect("Not a valid charge_full value");
    let charge_now: u64 = charge_now.trim().parse().expect("Not a valid charge_now value");
    let battery_charge_percentage = (charge_now * 100) / charge_full;
    let battery_status = fs::read_to_string(path.join("status")).expect("Failed to read battery status");
    let battery_status = String::from(battery_status.trim().to_lowercase());
    let battery_status = match battery_status.as_str() {
        "full" => "󰂅 ",
        "charging" => match battery_charge_percentage {
            0..33 => "󰂆 ",
            33..66 => "󱊥 ",
            66..99 => "󱊦 ",
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
    (String::from(battery_status), battery_charge_percentage)
}

fn get_mem_info() -> HashMap<String, u64> {
    let contents = std::fs::read_to_string("/proc/meminfo").expect("Could not read /proc/meminfo");
    let mut meminfo = HashMap::new();
    for line in contents.lines().take(3) {
        let mut split_line = line.split_whitespace();
        let label = split_line.next();
        let value = split_line.next();
        if value.is_some() && label.is_some() {
            let label = label.unwrap().split(':').nth(0).unwrap();
            let value = value.unwrap().parse::<u64>().ok().unwrap();
            meminfo.insert(String::from(label), value/1000000);
        }
    }
    meminfo
}

fn get_load_avg() -> Vec<f64> {
    let contents = std::fs::read_to_string("/proc/loadavg").expect("Could not read /proc/loadavg");
    contents.trim().split(' ')
        .take(3)
        .map(|val| val.parse::<f64>().unwrap())
        .collect::<Vec<f64>>()
}

