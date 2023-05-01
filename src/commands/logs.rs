pub fn show_log_path() {
    let device_name = whoami::devicename().to_lowercase();
    let log_path = format!("~/.config/softwarfare/logs/{}/", device_name);
    println!("> Log file location: {}", log_path);
}
