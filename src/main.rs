use std::io;
use std::process::{exit, Command};
fn display_menu() {
    println!("--Welcome to my rust Multi-Tool--");
    println!("If you paid this tool, you're scammed, so report it");
    println!("My discord: https://discord.gg/GsARdNqSVh ");
    println!("1. Flush DNS ( Powershell)"); // first option
    println!("2. Flush DNS ( ipconfig )"); // second option ipconfig /flushdns
    println!("99. EXIT"); // exit option
    println!("Please select an option: "); // fix it if we can't select an option, in my case I could do it
}

// script to flush DNS in powershell
fn flush_dns() {
    let ps_script = r#"
    Clear-DnsClientCache
    "#;
    
    println!("\x1b[33mExecuting DNS Flush command...\x1b[0m");
    println!("\x1b[36mCommand: Clear-DnsClientCache\x1b[0m");
    
    let output = Command::new("powershell")
        .args([
            "-NoProfile",
            "-Command",
            ps_script
        ])
        .output()
        .expect("Failed to execute Powershell script, check the command");

    println!("\x1b[32mDNS Cache has been successfully flushed!\x1b[0m");
    println!("\x1b[33mTip: You may need to restart your browser to see DNS changes take effect!\x1b[0m");
    println!("\x1b[33mTip: I recommend you to go the settings chrome://net-internals/#dns or about:networking#dns\n\x1b[0m");
    
    if !output.status.success() {
        eprintln!("\x1b[31mError: {}\x1b[0m", String::from_utf8_lossy(&output.stderr));
    }
}

// script to flush resolver cache
fn flush_resolver_cache() {
    let output = Command::new("ipconfig")
        .args(["/flushdns"])
        .output()
        .expect("Failed to execute command, check the program");
        println!("{}", String::from_utf8_lossy(&output.stdout));
        println!("\x1b[33mTip: You may need to restart your browser to see DNS changes take effect!\x1b[0m");
        println!("\x1b[33mTip: I recommend you to go the settings chrome://net-internals/#dns or about:networking#dns\n)\x1b[0m");
        if !output.status.success() {
            eprintln!("\x1b[31mError: {}\x1b[0m", String::from_utf8_lossy(&output.stderr));
        }
}
// loop to dispmlay the menu, here we can fix the menu and display
fn main() {
    loop {
        print!("\x1B[2J\x1B[H");
        display_menu();

        let mut choice = String::new(); // variable to store user input
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed at opening stdin"); // catch error

            match choice.trim() {
                "1" => {
                    flush_dns();
                    println!("\x1b[36mPress enter to continue ...\x1b[0m");
                    let mut temp = String::new();
                    io::stdin().read_line(&mut temp).unwrap();
                }
                "2" => {
                    flush_resolver_cache();
                    println!("\x1b[36mPress enter to continue ...\x1b[0m");
                    let mut temp = String::new();
                    io::stdin().read_line(&mut temp).unwrap();
                }
                "99" => {
                    println!("\x1b[32mExit, bye!\x1b[0m");
                    exit(0);
                }
                _ => {
                    println!("\x1b[31mInvalid choice, please try a valid option\x1b[0m");
                    
                }
            }
        }
    }