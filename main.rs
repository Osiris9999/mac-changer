use std::process::Command;

fn main() {
    let interface = String::from("wlan0");
    let new_mac = String::from("00:11:22:33:44:55");
    let output1 = Command::new("ifconfig")
        .arg(interface.clone())
        .arg("down")
        .output()
        .expect("Failed to execute command");
    println!("\x1b[1;31m{}", String::from_utf8_lossy(&output1.stderr).replace("\n", ""));
    let output2 = Command::new("ifconfig")
        .arg(interface.clone())
        .arg("hw")
        .arg("ether")
        .arg(new_mac)
        .output()
        .expect("Failed to execute command");
        println!("\x1b[1;31m{}", String::from_utf8_lossy(&output2.stderr).replace("\n", ""));
    let output3 = Command::new("ifconfig")
        .arg(interface.clone())
        .arg("up")
        .output()
        .expect("Failed to execute command");
    println!("\x1b[1;31m{}", String::from_utf8_lossy(&output3.stderr).replace("\n", ""));
}
