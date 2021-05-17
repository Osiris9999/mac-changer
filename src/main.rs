use std::process::Command;
fn main() {
    //ifconfig wlan0 down
    // ifconfig wlan0 hw ether 00:11:22:33:44:55
    //ifconfig wlan0 up

    let interface = String::from("wlan0");
    let new_mac = String::from("00:11:22:33:44:55");

    let _output = Command::new("ifconfig")
        .arg(interface.clone())
        .arg("down")
        .output()
        .expect("Failed to execute command");

    let _output2 = Command::new("ifconfig")
        .arg(interface.clone())
        .arg("hw")
        .arg("ether")
        .arg(new_mac)
        .output()
        .expect("Failed to execute command");

    let _output3 = Command::new("ifconfig")
        .arg(interface.clone())
        .arg("up")
        .output()
        .expect("Failed to execute command");
}
