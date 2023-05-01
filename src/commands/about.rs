use mac_address::get_mac_address;

pub fn show_user_info() {
    let user_name = whoami::username();
    let device_name = whoami::devicename().to_lowercase();
    let os_name = whoami::distro();
    let mac_address_string: String = match get_mac_address() {
        Ok(Some(mac_address)) => mac_address.to_string(),
        Ok(None) | Err(_) => String::from("Unknown mac address"),
    };

    //println!(("> Tenant URL                : {}", pSettings->value("tenantUrl", "")));
    println!("> Login User                : {}", user_name);
    println!("> Device Name               : {}", device_name);
    println!("> Operating System          : {}", os_name);
    //println!(("> Authentication Expiration : {}", expireTimeText));
    println!("> MAC Address               : {}", mac_address_string);
}
