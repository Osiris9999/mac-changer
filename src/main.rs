use std::process::Command;
fn main() {
    //ifconfig wlan0 down
    // ifconfig wlan0 hw ether 00:11:22:33:44:55
    //ifconfig wlan0 up

    let interface = String::from("wlan0");
    let mut new_mac = String::from("00:11:22:33:44:55");

    let mut output = Command::new("ifconfig")
        .arg(interface)
        .arg("down")
        .output()
        .expect("Failed to execute command");

    let mut output2 = Command::new("ifconfig")
        .arg(interface)
        .arg("hw")
        .arg("ether")
        .arg(new_mac)
        .output()
        .expect("Failed to execute command");

    let mut output3 = Command::new("ifconfig")
        .arg(interface)
        .arg("up")
        .output()
        .expect("Failed to execute command");
}
