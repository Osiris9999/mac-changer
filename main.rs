use std::io;
use std::io::{stdout, Write};
use std::process::Command;

fn main() {
    print!("\x1b[1;36mPlease enter the interface for which you want to change the mac address: \x1b[0m");
    let _=stdout().flush();
    let mut interface = String::default();
    io::stdin()
        .read_line(&mut interface)
        .expect("Failed to read line");
    let new_mac = String::from("00:11:22:33:44:55");
    let output1 = Command::new("ifconfig")
        .arg(interface.clone().replace("\n", ""))
        .arg("down")
        .output()
        .expect("Failed to execute command");
    println!("\x1b[1;31m{:#?}", String::from_utf8_lossy(&output1.stderr).replace("\n", ""));
    let output2 = Command::new("ifconfig")
        .arg(interface.clone().replace("\n", ""))
        .arg("hw")
        .arg("ether")
        .arg(new_mac)
        .output()
        .expect("Failed to execute command");
        println!("\x1b[1;31m{}", String::from_utf8_lossy(&output2.stderr).replace("\n", ""));
    let output3 = Command::new("ifconfig")
        .arg(interface.clone().replace("\n", ""))
        .arg("up")
        .output()
        .expect("Failed to execute command");
    println!("\x1b[1;31m{}", String::from_utf8_lossy(&output3.stderr).replace("\n", ""));
}
