use std::process::Command;
fn main() {
    //ifconfig wlan0 down
    // ifconfig wlan0 hw ether 00:11:22:33:44:55
    //ifconfig wlan0 up

    let interface = String::from("wlan0");
    let mut new_mac = String::from("00:11:22:33:44:55");

    output = Command::new("ifconfig")
        .arg(interface, "down")
        .output()
        .expect("Failed to execute command");

    output2 = Command::new("ifconfig")
        .arg("interface", "hw", "ether", new_mac)
        .output()
        .expect("Failed to execute command");

    output3 = Command::new("ifconfig")
        .arg("interface", "up")
        .output()
        .expect("Failed to execute command");
}
