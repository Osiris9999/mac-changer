use std::io;
use std::process::Command;

fn main() {
    println!("Please enter the interface for which you want to change the mac address: ");
    let mut interface = String::new();
    io::stdin()
        .read_line(&mut interface)
        .expect("Failed to read line");
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
