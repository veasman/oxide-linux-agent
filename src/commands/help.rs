pub fn show_commands() {
    println!("about                             : Lists about page information.");
    println!("exit                              : Exits the program.");
    println!("factors <factor>                  : Lists current factors. Sets factor if provided with name or Id.");
	println!("healthcheck                       : Tests the connection to the server.");
    println!("logs                              : Lists path to log files.");
    println!("reset                             : Resets Enum and chosen factor.");
    println!("settings <enum/tenantUrl> <value> : Lists current settings. Sets enum tenantUrl if provided.");
	println!("status                            : Lists current status.");
    println!("sync <factor>                     : Syncs with saved enum. Will overide set factor if provided.");
}
